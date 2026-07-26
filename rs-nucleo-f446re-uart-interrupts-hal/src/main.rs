#![no_main]
#![no_std]

use core::cell::RefCell;

use cortex_m_rt::entry;
use critical_section::Mutex;
use embedded_hal_nb::serial::Write as _;
use fugit::ExtU32;
use panic_halt as _;
use stm32f4xx_hal::{
    ClearFlags, Listen,
    gpio::GpioExt,
    pac::{self, Interrupt, NVIC, interrupt},
    rcc::{self, Rcc, RccExt},
    serial::{self, RxISR, SerialExt, TxISR},
    time::Bps,
    timer::{self, CounterMs, TimerExt},
};

const HALF_PERIOD_MS: u32 = 1000;
const BUFFER_CAPACITY: usize = 32;
const BAUD_RATE: u32 = 115_200;

struct SharedResources {
    buffer: heapless::Deque<u8, BUFFER_CAPACITY>,
    tim6: CounterMs<pac::TIM6>,
    usart2: serial::Serial<pac::USART2, u8>,
}

static SHARED_RESOURCES: Mutex<RefCell<Option<SharedResources>>> = Mutex::new(RefCell::new(None));

fn setup_timer(rcc: &mut Rcc, tim6: pac::TIM6) -> CounterMs<pac::TIM6> {
    let mut tim6 = tim6.counter_ms(rcc);
    tim6.start(HALF_PERIOD_MS.millis()).unwrap();
    tim6.listen(timer::Event::Update);

    tim6
}

fn setup_usart2(
    rcc: &mut Rcc,
    gpioa: pac::GPIOA,
    usart2: pac::USART2,
) -> serial::Serial<pac::USART2, u8> {
    let gpioa = gpioa.split(rcc);
    let tx_pin = gpioa.pa2.into_alternate();
    let rx_pin = gpioa.pa3.into_alternate();
    let config = serial::Config::default().baudrate(Bps(BAUD_RATE));

    let mut usart2_serial = usart2.serial((tx_pin, rx_pin), config, rcc).unwrap();
    usart2_serial.listen(serial::Event::RxNotEmpty);

    usart2_serial
}

#[interrupt]
fn TIM6_DAC() {
    critical_section::with(|cs| {
        let mut shared = SHARED_RESOURCES.borrow(cs).borrow_mut();
        let Some(shared) = shared.as_mut() else {
            return;
        };

        shared.tim6.clear_flags(timer::Flag::Update);

        for &byte in b"\r\nTimer!\r\n" {
            // Ignore overflow in this example
            let _ = shared.buffer.push_back(byte);
        }

        shared.usart2.listen(serial::Event::TxEmpty);
    });
}

#[interrupt]
fn USART2() {
    // Overrun, framing and noise errors not handled in this example
    critical_section::with(|cs| {
        let mut shared = SHARED_RESOURCES.borrow(cs).borrow_mut();
        let Some(shared) = shared.as_mut() else {
            return;
        };

        if shared.usart2.is_rx_not_empty() {
            // Serial::read()'s Err path does not read DR, so ORE is never
            // cleared — confirmed by observing the flag directly. Bypassing
            // Read and hitting SR/DR ourselves is required here.
            let usart = unsafe { &*pac::USART2::ptr() };
            let byte = usart.dr().read().dr().bits() as u8;

            // Ignore overflow in this example
            let _ = shared.buffer.push_back(byte);

            shared.usart2.listen(serial::Event::TxEmpty);
        }

        // HAL doesn't expose a way to check if TXE interrupt is enabled so
        // this gets run every time.
        if shared.usart2.is_tx_empty() {
            if let Some(byte) = shared.buffer.pop_front() {
                let _ = shared.usart2.write(byte);
            } else {
                shared.usart2.unlisten(serial::Event::TxEmpty);
            }
        }
    });
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.freeze(rcc::Config::hsi());
    let tim6 = setup_timer(&mut rcc, dp.TIM6);
    let usart2 = setup_usart2(&mut rcc, dp.GPIOA, dp.USART2);

    critical_section::with(|cs| {
        *SHARED_RESOURCES.borrow(cs).borrow_mut() = Some(SharedResources {
            buffer: heapless::Deque::new(),
            tim6,
            usart2,
        });
    });

    for irq in [Interrupt::TIM6_DAC, Interrupt::USART2] {
        NVIC::unpend(irq);
        unsafe {
            NVIC::unmask(irq);
        }
    }

    loop {
        cortex_m::asm::wfi();
    }
}

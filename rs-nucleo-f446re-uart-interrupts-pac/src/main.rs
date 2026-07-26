#![no_main]
#![no_std]

use core::cell::RefCell;

use cortex_m_rt::entry;
use critical_section::Mutex;
use panic_halt as _;
use stm32f4::stm32f446 as pac;
use pac::{interrupt, Interrupt};

// USART2 TX is on PA2
// USART2 RX is on PA3

const TIM2_PSC_VAL: u16 = 15_999; // 16 MHz / (15_999 + 1) = 1 kHz → 1 ms per tick
const TIMER_PERIOD_TICKS: u32 = 1999; // (1999 + 1) ticks × 1 ms = 2000 ms = 2 s

struct SharedResources {
    usart2: pac::USART2,
    buffer: heapless::Deque<u8, 32>,
    tim2: pac::TIM2,
}

static SHARED_RESOURCES: Mutex<RefCell<Option<SharedResources>>> =
    Mutex::new(RefCell::new(None));

fn init_timer(rcc: &pac::RCC, tim2: &pac::TIM2) {
    rcc.apb1enr().modify(|_, w| w.tim2en().set_bit());
    tim2.dier().modify(|_, w| w.uie().set_bit());
    tim2.psc().write(|w| w.psc().set(TIM2_PSC_VAL));
    tim2.arr().write(|w| w.arr().set(TIMER_PERIOD_TICKS));

    // Force update event to load registers into active shadow registers
    tim2.egr().write(|w| w.ug().set_bit());

    // Start counter
    tim2.cr1().modify(|_, w| w.cen().set_bit());

    // Clear the flag that UG generated so the main loop doesn't instantly
    // jump past the first delay
    tim2.sr().modify(|_, w| w.uif().clear());
}

fn setup_usart2(rcc: &pac::RCC, gpioa: &pac::GPIOA, usart2: &pac::USART2) {
    rcc.ahb1enr().modify(|_, w| w.gpioaen().set_bit());
    rcc.apb1enr().modify(|_, w| w.usart2en().set_bit());

    #[rustfmt::skip]
    gpioa.moder().modify(|_, w| {
        w.moder2().alternate()
         .moder3().alternate()
    });

    // Per Table 11 of the Datasheet (page 58)
    //   * PA2 -> USART2 TX = AF7
    //   * PA3 -> USART2 RX = AF7
    #[rustfmt::skip]
    gpioa.afrl().modify(|_, w| {
        w.afrl2().af7()
         .afrl3().af7()
    });

    // Set baud rate (115200 @ 16 MHz HSI, default reset clocks, no PLL)
    // USARTDIV = 16_000_000 / (16 * 115200) ≈ 8.6875 -> mantissa=8, fraction=11
    #[rustfmt::skip]
    usart2.brr().write(|w| unsafe {
        w.div_mantissa().bits(8)
         .div_fraction().bits(11)
    });

    // TX interrupts are only enabled when bytes are queued
    #[rustfmt::skip]
    usart2.cr1().write(|w|
         w.ue().set_bit()
          .te().set_bit()
          .re().set_bit()
          .rxneie().set_bit()
    );
}

fn start_tx(usart2: &pac::USART2) {
    usart2.cr1().modify(|_, w| w.txeie().set_bit());
}

#[interrupt]
fn TIM2() {
    critical_section::with(|cs| {
        let mut shared = SHARED_RESOURCES.borrow(cs).borrow_mut();
        let Some(shared) = shared.as_mut() else {
            return;
        };

        // Clear the event
        shared.tim2.sr().modify(|_, w| w.uif().clear());

        for &byte in b"\r\nTimer!\r\n" {
            // Ignore overflow in this example
            let _ = shared.buffer.push_back(byte);
        }

        start_tx(&shared.usart2);
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

        let sr = shared.usart2.sr().read();

        let rx_not_empty = sr.rxne().bit_is_set();
        if rx_not_empty {
            let byte = shared.usart2.dr().read().dr().bits() as u8;
            // Ignore overflow in this example
            let _ = shared.buffer.push_back(byte);
            start_tx(&shared.usart2);
        }

        let tx_reg_empty = sr.txe().bit_is_set();
        let txe_interrupt_enabled = shared.usart2.cr1().read().txeie().bit_is_set();
        if tx_reg_empty && txe_interrupt_enabled {
            if let Some(byte) = shared.buffer.pop_front() {
                shared.usart2.dr().write(|w| w.dr().set(byte as u16));
            } else {
                shared.usart2.cr1().modify(|_, w| w.txeie().clear_bit());
            }
        }
    });
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    setup_usart2(&dp.RCC, &dp.GPIOA, &dp.USART2);
    init_timer(&dp.RCC, &dp.TIM2);

    critical_section::with(|cs| {
        *SHARED_RESOURCES.borrow(cs).borrow_mut() = Some(SharedResources {
            usart2: dp.USART2,
            buffer: heapless::Deque::new(),
            tim2: dp.TIM2,
        });
    });

    for irq in [Interrupt::TIM2, Interrupt::USART2] {
        pac::NVIC::unpend(irq);
        unsafe {
            pac::NVIC::unmask(irq);
        }
    }

    loop {
        cortex_m::asm::wfi();
    }
}

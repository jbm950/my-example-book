#![no_main]
#![no_std]

use core::cell::RefCell;

use cortex_m_rt::entry;
use critical_section::Mutex;
use fugit::ExtU32;
use panic_halt as _;
use stm32f4xx_hal::{
    ClearFlags, Listen,
    gpio::{gpioa::PA5, Output, PushPull, GpioExt},
    pac::{self, Interrupt, NVIC, interrupt},
    rcc::{Config, RccExt},
    timer::{CounterMs, Event, Flag, TimerExt},
};

const HALF_PERIOD_MS: u32 = 500;

struct SharedPeripherals {
    led: PA5<Output<PushPull>>,
    tim6: CounterMs<pac::TIM6>,
}

static SHARED_PERIPHERALS: Mutex<RefCell<Option<SharedPeripherals>>> =
    Mutex::new(RefCell::new(None));

#[interrupt]
fn TIM6_DAC() {
    critical_section::with(|cs| {
        let mut shared = SHARED_PERIPHERALS.borrow(cs).borrow_mut();

        if let Some(shared) = shared.as_mut() {
            shared.tim6.clear_flags(Flag::Update);
            shared.led.toggle();
        }
    });
}

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();

    let mut rcc = peripherals.RCC.freeze(Config::hsi());

    let gpioa = peripherals.GPIOA.split(&mut rcc);
    let led = gpioa.pa5.into_push_pull_output();

    let mut tim6 = peripherals.TIM6.counter_ms(&mut rcc);
    tim6.start(HALF_PERIOD_MS.millis()).unwrap();
    tim6.listen(Event::Update);

    critical_section::with(|cs| {
        *SHARED_PERIPHERALS.borrow(cs).borrow_mut() = Some(SharedPeripherals {
            led,
            tim6,
        });
    });

    NVIC::unpend(Interrupt::TIM6_DAC);
    unsafe {
        NVIC::unmask(Interrupt::TIM6_DAC);
    }

    loop {
        cortex_m::asm::wfi();
    }
}

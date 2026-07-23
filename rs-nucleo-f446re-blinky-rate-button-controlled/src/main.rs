#![no_main]
#![no_std]

use core::cell::RefCell;

use cortex_m_rt::entry;
use critical_section::Mutex;
use fugit::ExtU32;
use panic_halt as _;
use stm32f4xx_hal::{
    ClearFlags, Listen,
    gpio::{Edge, ExtiPin, GpioExt, Output, PA5, PC13, PushPull},
    pac::{self, Interrupt, NVIC, interrupt},
    rcc::{Config, Enable, Rcc, RccExt},
    syscfg::SysCfgExt,
    timer::{CounterMs, Event, Flag, TimerExt},
};

const TIM2_PSC_VAL: u16 = 15_999; // 16 MHz / (15_999 + 1) = 1 kHz → 1 ms per tick
const DEBOUNCE_PERIOD_TICKS: u32 = 29; // (29 + 1) ticks × 1 ms = 30 ms

struct SharedResources {
    led: PA5<Output<PushPull>>,
    button: PC13,
    blink_rate: BlinkRateMs,
    blink_timer: CounterMs<pac::TIM6>,
    debounce_timer: pac::TIM2,
}

impl SharedResources {
    fn debounce_active(&self) -> bool {
        self.debounce_timer.cr1().read().cen().bit_is_set()
    }

    fn start_debounce_timer(&self) {
        self.debounce_timer.cr1().modify(|_, w| w.cen().set_bit());
    }

    fn next_blink_rate(&mut self) {
        self.blink_rate.next();
        let half_period = self.blink_rate.millis();

        self.blink_timer.cancel().unwrap();
        self.blink_timer.start(half_period).unwrap();
    }
}

static SHARED_PERIPHERALS: Mutex<RefCell<Option<SharedResources>>> =
    Mutex::new(RefCell::new(None));

#[derive(Copy, Clone)]
enum BlinkRateMs {
    Fast = 500,
    Medium = 1000,
    Slow = 1500,
}

impl BlinkRateMs {
    fn next(&mut self) {
        *self = match self {
            BlinkRateMs::Fast => BlinkRateMs::Medium,
            BlinkRateMs::Medium => BlinkRateMs::Slow,
            BlinkRateMs::Slow => BlinkRateMs::Fast,
        }
    }

    fn millis(&self) -> fugit::MillisDurationU32 {
        (*self as u32).millis()
    }
}

fn setup_button(
    gpioc: pac::GPIOC,
    rcc: &mut Rcc,
    syscfg: pac::SYSCFG,
    exti: &mut pac::EXTI,
) -> PC13 {
    let gpioc = gpioc.split(rcc);
    let mut button = gpioc.pc13.into_floating_input();

    let mut syscfg = syscfg.constrain(rcc);
    button.make_interrupt_source(&mut syscfg);
    button.trigger_on_edge(exti, Edge::Falling);
    button.enable_interrupt(exti);

    button
}

/// Configures TIM2 as a one-pulse-mode debounce timer.
/// One-pulse mode isn't exposed by the `Counter` HAL abstraction,
/// so this configures the peripheral directly via PAC registers.
fn configure_debounce_timer(rcc: &mut Rcc, tim2: &pac::TIM2) {
    pac::TIM2::enable(rcc);
    tim2.psc().write(|w| w.psc().set(TIM2_PSC_VAL));
    tim2.arr().write(|w| w.arr().set(DEBOUNCE_PERIOD_TICKS));
    tim2.cr1().modify(|_, w| w.opm().set_bit());

    // Force update event to load registers into active shadow registers
    tim2.egr().write(|w| w.ug().set_bit());
}

#[interrupt]
fn TIM6_DAC() {
    critical_section::with(|cs| {
        let mut shared = SHARED_PERIPHERALS.borrow(cs).borrow_mut();
        let Some(shared) = shared.as_mut() else {
            return;
        };

        shared.blink_timer.clear_flags(Flag::Update);
        shared.led.toggle();
    });
}

#[interrupt]
fn EXTI15_10() {
    critical_section::with(|cs| {
        let mut shared = SHARED_PERIPHERALS.borrow(cs).borrow_mut();
        let Some(shared) = shared.as_mut() else {
            return;
        };

        if !shared.button.check_interrupt() {
            return;
        }
        shared.button.clear_interrupt_pending_bit();

        if shared.debounce_active() {
            return
        }
        shared.next_blink_rate();
        shared.start_debounce_timer();
    });
}

#[entry]
fn main() -> ! {
    let blink_rate = BlinkRateMs::Medium;

    let mut peripherals = pac::Peripherals::take().unwrap();

    let mut rcc = peripherals.RCC.freeze(Config::hsi());

    let gpioa = peripherals.GPIOA.split(&mut rcc);
    let led = gpioa.pa5.into_push_pull_output();

    let mut blink_timer = peripherals.TIM6.counter_ms(&mut rcc);
    blink_timer.start(blink_rate.millis()).unwrap();
    blink_timer.listen(Event::Update);

    let button = setup_button(
        peripherals.GPIOC,
        &mut rcc,
        peripherals.SYSCFG,
        &mut peripherals.EXTI,
    );
    configure_debounce_timer(&mut rcc, &peripherals.TIM2);

    critical_section::with(|cs| {
        *SHARED_PERIPHERALS.borrow(cs).borrow_mut() = Some(SharedResources {
            led,
            button,
            blink_rate,
            blink_timer,
            debounce_timer: peripherals.TIM2,
        });
    });

    for irq in [Interrupt::EXTI15_10, Interrupt::TIM6_DAC] {
        NVIC::unpend(irq);
        unsafe {
            NVIC::unmask(irq);
        }
    }

    loop {
        cortex_m::asm::wfi();
    }
}

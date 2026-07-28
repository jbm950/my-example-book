#![no_main]
#![no_std]


use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4::stm32f446 as pac;

// Per Table 11 of the Datasheet (page 58)
//   * PB8 -> I2C1 SCL = AF4
//   * PB9 -> I2C1 SDA = AF4

fn setup_i2c1(rcc: &pac::RCC, gpiob: &pac::GPIOB, i2c1: &pac::I2C1) {
    // Enable clocks for I2C1
    rcc.ahb1enr().modify(|_, w| w.gpioben().set_bit());
    rcc.apb1enr().modify(|_, w| w.i2c1en().set_bit());

    // Set up I2C1 pins
    #[rustfmt::skip]
    gpiob.moder().modify(|_, w| {
        w.moder8().alternate()
         .moder9().alternate()
    });

    #[rustfmt::skip]
    gpiob.otyper().modify(|_, w| {
        w.ot8().open_drain()
         .ot9().open_drain()
    });

    #[rustfmt::skip]
    gpiob.pupdr().modify(|_, w| {
        w.pupdr8().pull_up()
         .pupdr9().pull_up()
    });

    #[rustfmt::skip]
    gpiob.afrh().modify(|_, w| {
        w.afrh8().af4()
         .afrh9().af4()
    });

    // I2C1 Reset
    i2c1.cr1().modify(|_, w| w.swrst().set_bit());
    i2c1.cr1().modify(|_, w| w.swrst().clear_bit());

    // I2C1 Timing
    i2c1.cr2().modify(|_, w| unsafe { w.freq().bits(16) }); // APB1 = 16 MHz (default HSI, no PLL)
    i2c1.ccr().modify(|_, w| unsafe { w.ccr().bits(80) });
    i2c1.trise().modify(|_, w| unsafe { w.trise().bits(17) });
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    setup_i2c1(&dp.RCC, &dp.GPIOB, &dp.I2C1);

    // TODO Need to actually scan the addresses
    loop {
        cortex_m::asm::wfi();
    }
}

fn i2c_probe(i2c: pac::I2C1, addr: u8) {
    // TODO
}

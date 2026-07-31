#![no_main]
#![no_std]


use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4::stm32f446 as pac;

// Per Table 11 of the Datasheet (page 58)
//   * PB8 -> I2C1 SCL = AF4
//   * PB9 -> I2C1 SDA = AF4

// Number of poll iterations to wait before giving up on a flag.
/// Not calibrated to a specific time — just large enough that a
/// healthy bus/peripheral will never hit it, and a stuck one won't
/// hang forever.
const POLL_ITERATION_TIMEOUT: u32 = 100_000;

/// Valid 7-bit I2C addresses; 0x00-0x07 and 0x78-0x7F are reserved.
const SCAN_RANGE: core::ops::Range<u8> = 0x08..0x78;

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

    // CCR: sets SCL high/low time for Standard Mode.
    // RM0390 formula (Sm): CCR = PCLK1 / (2 * f_SCL)
    //   PCLK1 = 16,000,000 Hz
    //   f_SCL = 100,000 Hz (target I2C bus speed)
    //   CCR   = 16,000,000 / (2 * 100,000) = 80
    i2c1.ccr().modify(|_, w| unsafe { w.ccr().bits(80) });

    // TRISE: max allowed SCL rise time, in units of PCLK1 periods, +1.
    // RM0390 formula: TRISE = (max_rise_time_ns / T_PCLK1) + 1
    //   T_PCLK1        = 1 / 16 MHz = 62.5 ns
    //   max_rise_time  = 1000 ns (1 us, per I2C spec for Standard Mode)
    //   TRISE = (1000 / 62.5) + 1 = 16 + 1 = 17
    i2c1.trise().write(|w| unsafe { w.trise().bits(17) });

    i2c1.cr1().modify(|_, w| w.pe().set_bit());
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = pac::Peripherals::take().unwrap();

    setup_i2c1(&dp.RCC, &dp.GPIOB, &dp.I2C1);
    rprintln!("Setup I2C complete");

    let mut num_devices = 0u32;
    for addr in SCAN_RANGE {
        if i2c_probe(&dp.I2C1, addr) {
            rprintln!("Found device at 0x{:02X}", addr);
            num_devices += 1;
        }
    }

    rprintln!("Scan complete. {} device(s) found.", num_devices);

    loop {
        cortex_m::asm::nop();
    }
}

fn poll_until(mut cond: impl FnMut() -> bool) -> bool {
    let mut time_remaining = POLL_ITERATION_TIMEOUT;
    while time_remaining > 0 {
        if cond() {
            return true;
        }

        time_remaining -= 1;
    }

    false
}

/// Attempts a zero-byte write to `addr`. Returns true if it's ACKed.
fn i2c_probe(i2c: &pac::I2C1, addr: u8) -> bool {
    i2c.cr1().modify(|_, w| w.start().set_bit());
    if !poll_until(|| i2c.sr1().read().sb().bit_is_set()) {
        rprintln!("Timed out waiting for START at addr 0x{:02X}", addr);
        return false;
    }

    // 7-bit address, shifted up one bit; bit 0 = 0 selects write direction.
    let address_byte = addr << 1;
    i2c.dr().write(|w| unsafe { w.dr().bits(address_byte) });

    poll_until(|| {
        let sr1 = i2c.sr1().read();
        sr1.addr().bit_is_set() || sr1.af().bit_is_set()
    });
    let acked = i2c.sr1().read().addr().bit_is_set();

    if acked {
        // ADDR is cleared by reading SR1 then SR2
        let _ = i2c.sr1().read();
        let _ = i2c.sr2().read();
    } else {
        // Clear acknowledge failure so the next transaction starts cleanly.
        i2c.sr1().modify(|_, w| w.af().clear_bit());
    }

    i2c.cr1().modify(|_, w| w.stop().set_bit());
    if !poll_until(|| i2c.sr2().read().busy().bit_is_clear()) {
        rprintln!("STOP timeout at addr 0x{:02X}", addr);
        return false;
    }

    acked
}

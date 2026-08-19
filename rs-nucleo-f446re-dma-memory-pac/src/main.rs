#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};
use stm32f4::stm32f446 as pac;

const LEN: usize = 16;
const STREAM_IDX: usize = 0; // Stream0, arbitrary choice for mem-mem

fn configure_dma(rcc: pac::RCC, dma2: &pac::DMA2, src_addr: u32, dst_addr: u32) {
    rcc.ahb1enr().modify(|_, w| w.dma2en().enabled());

    let stream = dma2.st(STREAM_IDX);

    // Ensure stream is disabled before configuring
    stream.cr().modify(|_, w| w.en().disabled());
    while stream.cr().read().en().bit_is_set() {};

    // Clear stale interrupt flags
    #[rustfmt::skip]
    dma2.lifcr().write(|w| {
        w.ctcif0().clear()
         .chtif0().clear()
         .cteif0().clear()
         .cdmeif0().clear()
         .cfeif0().clear()
    });

    // Addresses and transfer length
    stream.par().write(|w| unsafe { w.pa().bits(src_addr) });
    stream.m0ar().write(|w| unsafe { w.m0a().bits(dst_addr) });
    stream.ndtr().write(|w| unsafe { w.ndt().bits(LEN as u16) });

    // Configure Transfer: mem-to-mem, increment both, 32-bit words
    stream.cr().write(|w| {
        w.dir().memory_to_memory()
         .minc().incremented()
         .msize().bits32()
         .pinc().incremented()
         .psize().bits32()
    });
}

fn do_transfer(dma2: &pac::DMA2) {
    let stream = dma2.st(STREAM_IDX);

    // Start the transfer
    stream.cr().modify(|_, w| w.en().enabled());

    // Poll for transfer complete flag. Example doesn't check error flags.
    while dma2.lisr().read().tcif0().bit_is_clear() {};

    // Clear the flag and disable the stream
    dma2.lifcr().write(|w| w.ctcif0().clear());
    stream.cr().modify(|_, w| w.en().disabled());
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let mut src: [u32; LEN] = [0; LEN];
    let mut dst: [u32; LEN] = [0; LEN];

    // Set initial SRC values
    for (i, v) in src.iter_mut().enumerate() {
        *v = (i as u32) + 1; // Simple incrementing 1, 2, 3, ..., LEN
    }

    let src_addr = src.as_ptr() as u32;
    let dst_addr = dst.as_mut_ptr() as u32;

    let dp = pac::Peripherals::take().unwrap();

    configure_dma(dp.RCC, &dp.DMA2, src_addr, dst_addr);

    // Full memory barrier before starting DMA so buffer writes above are
    // visible to the DMA controller
    cortex_m::asm::dmb();

    do_transfer(&dp.DMA2);

    // Memory barrier before CPU reads what DMA just wrote
    cortex_m::asm::dmb();

    // Verify
    let mut ok = true;
    for (idx, (s_elem, d_elem)) in src.into_iter().zip(dst).enumerate() {
        if s_elem != d_elem {
            ok = false;
            rprintln!("Mismatch at index {}: src={} dst={}", idx, s_elem, d_elem);
        }
    }

    if ok {
        rprintln!("DMA transfer SUCCESS: all {} words matched", LEN);
    } else {
        rprintln!("DMA transfer FAILED: mismatches detected above");
    }

    loop {
        cortex_m::asm::wfi();
    }
}

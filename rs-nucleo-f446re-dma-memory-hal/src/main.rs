#![no_main]
#![no_std]

use cortex_m::singleton;
use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::{
    dma::{
        MemoryToMemory, Stream0, StreamsTuple, Transfer,
        config::DmaConfig,
        traits::{Direction, StreamISR},
    },
    pac,
    rcc::RccExt,
};

const LEN: usize = 16; // Number of bytes transferred

// Number of bytes transferred. The generic Transfer::init_memory_to_memory
// path currently only has a DMASet impl for MemoryToMemory<u8>, so the HAL
// version of this example moves bytes rather than the 32-bit words the
// PAC version used.
type Mem2MemTransfer =
    Transfer<Stream0<pac::DMA2>, 0, MemoryToMemory<u8>, MemoryToMemory<u8>, &'static mut [u8; LEN]>;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = pac::Peripherals::take().unwrap();

    // DMA requires buffers that remain valid for the lifetime of the transfer.
    let src = singleton!(: [u8; LEN] = [0u8; LEN]).unwrap();
    let dst = singleton!(: [u8; LEN] = [0u8; LEN]).unwrap();

    for (idx, byte) in src.iter_mut().enumerate() {
        *byte = (idx as u8) + 1; // Simple incrementing 1, 2, 3, ..., LEN
    }

    let streams = StreamsTuple::new(dp.DMA2, &mut dp.RCC.constrain());

    // In memory-to-memory mode, the HAL uses the peripheral-address
    // side of the DMA as the second memory address.
    let dma_config = DmaConfig::default()
        .memory_increment(true)
        .peripheral_increment(true)
        .fifo_enable(true); // required: direct mode can't do mem-to-mem at HAL layer

    let mut transfer: Mem2MemTransfer = Transfer::init_memory_to_memory(
        streams.0, // Stream 0, arbitrary choice for memory to memory
        MemoryToMemory::new(),
        dst,
        src,
        dma_config,
    );

    // start() hands us &mut PERIPHERAL for last-moment peripheral setup;
    // there's no real peripheral in mem-to-mem mode, so nothing to do here.
    transfer.start(|_| {});

    // Example doesn't account for is_transfer_error()
    while !transfer.is_transfer_complete() {}
    transfer.clear_transfer_complete();

    let (_stream, _peripheral, dst, src) = transfer.release();
    let src = src.unwrap(); // Double buffer is an optional return

    // Verify
    let mut ok = true;
    for (idx, (s_elem, d_elem)) in src.iter().zip(dst.iter()).enumerate() {
        if s_elem != d_elem {
            ok = false;
            rprintln!("Mismatch at index {}: src={} dst={}", idx, s_elem, d_elem);
        }
    }

    if ok {
        rprintln!("DMA transfer SUCCESS: all {} bytes matched", LEN);
    } else {
        rprintln!("DMA transfer FAILED: mismatches detected above");
    }

    loop {
        cortex_m::asm::wfi();
    }
}

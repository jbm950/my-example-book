#![no_main]
#![no_std]

use core::cell::RefCell;

use cortex_m_rt::entry;
use critical_section::Mutex;
use embedded_hal_nb::serial::Write as _;
use panic_halt as _;
use stm32f4xx_hal::{
    Listen,
    gpio::{DefaultMode, GpioExt, Output, PA2, PA3, PA5, PushPull},
    pac::{self, Interrupt, NVIC, interrupt},
    rcc::{self, Rcc, RccExt},
    serial::{self, RxISR, SerialExt, TxISR},
    time::Bps,
};

const CMD_BUFFER_CAPACITY: usize = 32;
const TX_BUFFER_CAPACITY: usize = 32;
const BAUD_RATE: u32 = 115_200;

struct SharedResources {
    led: PA5<Output<PushPull>>,
    cmd_buffer: [u8; CMD_BUFFER_CAPACITY],
    cmd_buffer_count: usize,
    cmd_buffer_overflow: bool,
    tx_buffer: heapless::Deque<u8, TX_BUFFER_CAPACITY>,
    usart2: serial::Serial<pac::USART2, u8>,
}

impl SharedResources {
    fn echo_byte(&mut self, byte: u8) {
        // Ignore overflow in this example
        let _ = self.tx_buffer.push_back(byte);
    }

    fn handle_cmd_byte(&mut self, byte: u8) {
        if byte == b'\r' {
            self.buffer_tx_bytes(b"\r\n");

            if self.cmd_buffer_overflow {
                self.buffer_tx_bytes(b"Error: command too long\r\n");
            } else {
                let cmd_buffer = self.cmd_buffer;
                match &cmd_buffer[..self.cmd_buffer_count] {
                    b"on" => {
                        self.led.set_high();
                        self.buffer_tx_bytes(b"CMD: LED ON RCVD\r\n");
                    },
                    b"off" => {
                        self.led.set_low();
                        self.buffer_tx_bytes(b"CMD: LED OFF RCVD\r\n");
                    },
                    b"toggle" => {
                        self.led.toggle();
                        self.buffer_tx_bytes(b"CMD: LED TOGGLE RCVD\r\n");
                    },
                    b"status" => {
                        let state: &[u8] = if self.led.is_set_high() { b"ON"} else { b"OFF" };
                        self.buffer_tx_bytes(b"CMD: LED STATE: LED is ");
                        self.buffer_tx_bytes(state);
                        self.buffer_tx_bytes(b"\r\n");
                    },
                    unknown_cmd => {
                        self.buffer_tx_bytes(b"Unknown Command: ");
                        self.buffer_tx_bytes(unknown_cmd);
                        self.buffer_tx_bytes(b"\r\n");
                    }
                }
            }
            
            self.cmd_buffer_count = 0;
            self.cmd_buffer_overflow = false;
            return;
        }

        if self.cmd_buffer_count < CMD_BUFFER_CAPACITY {
            self.cmd_buffer[self.cmd_buffer_count] = byte;
            self.cmd_buffer_count += 1;
        } else {
            self.cmd_buffer_overflow = true;
        }
    }

    fn buffer_tx_bytes(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            let _ = self.tx_buffer.push_back(byte);
        }
    }
}

static SHARED_RESOURCES: Mutex<RefCell<Option<SharedResources>>> = Mutex::new(RefCell::new(None));



fn setup_usart2(
    rcc: &mut Rcc,
    pa2: PA2<DefaultMode>,
    pa3: PA3<DefaultMode>,
    usart2: pac::USART2,
) -> serial::Serial<pac::USART2, u8> {
    let tx_pin = pa2.into_alternate();
    let rx_pin = pa3.into_alternate();
    let config = serial::Config::default().baudrate(Bps(BAUD_RATE));

    let mut usart2_serial = usart2.serial((tx_pin, rx_pin), config, rcc).unwrap();
    usart2_serial.listen(serial::Event::RxNotEmpty);

    usart2_serial
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

            shared.echo_byte(byte);
            shared.handle_cmd_byte(byte);

            shared.usart2.listen(serial::Event::TxEmpty);
        }

        // HAL doesn't expose a way to check if TXE interrupt is enabled so
        // this gets run every time.
        if shared.usart2.is_tx_empty() {
            if let Some(byte) = shared.tx_buffer.pop_front() {
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
    let gpioa = dp.GPIOA.split(&mut rcc);

    let led = gpioa.pa5.into_push_pull_output();
    let usart2 = setup_usart2(&mut rcc, gpioa.pa2, gpioa.pa3, dp.USART2);

    critical_section::with(|cs| {
        *SHARED_RESOURCES.borrow(cs).borrow_mut() = Some(SharedResources {
            led,
            cmd_buffer: [0u8; CMD_BUFFER_CAPACITY],
            cmd_buffer_count: 0,
            cmd_buffer_overflow: false,
            tx_buffer: heapless::Deque::new(),
            usart2,
        });
    });

    NVIC::unpend(Interrupt::USART2);
    unsafe {
        NVIC::unmask(Interrupt::USART2);
    }

    loop {
        cortex_m::asm::wfi();
    }
}

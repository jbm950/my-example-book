use std::os::fd::{AsRawFd, FromRawFd};

// Const and struct definitions taken from
// https://github.com/torvalds/linux/blob/master/include/uapi/linux/gpio.h
const GPIO_V2_LINES_MAX: usize = 64;
const GPIO_MAX_NAME_SIZE: usize = 32;
const GPIO_V2_LINE_NUM_ATTRS_MAX: usize = 10;

const GPIO_V2_LINE_FLAG_OUTPUT: u64 = 8; // _BITULL(3) = 1 << 3

const GPIO_V2_GET_LINE_IOCTL: libc::Ioctl = libc::_IOWR::<GpioV2LineRequest>(0xB4, 0x07);
const GPIO_V2_LINE_SET_VALUES_IOCTL: libc::Ioctl = libc::_IOWR::<GpioV2LineValues>(0xB4, 0x0F);

#[repr(C)]
struct GpioV2LineRequest {
    offsets: [u32; GPIO_V2_LINES_MAX],
    consumer: [libc::c_char; GPIO_MAX_NAME_SIZE],
    config: GpioV2LineConfig,
    num_lines: u32,
    event_buffer_size: u32,
    padding: [u32; 5],
    fd: i32,
}

#[repr(C)]
struct GpioV2LineConfig {
    flags: u64,
    num_attrs: u32,
    padding: [u32; 5],
    attrs: [GpioV2LineConfigAttribute; GPIO_V2_LINE_NUM_ATTRS_MAX],
}

#[repr(C)]
struct GpioV2LineConfigAttribute {
    attr: GpioV2LineAttribute,
    mask: u64,
}

#[repr(C)]
struct GpioV2LineAttribute {
    id: u32,
    padding: u32,
    union: LineAttrUnion,
}

#[repr(C)]
union LineAttrUnion {
    flags: u64,
    values: u64,
    debounce_period_us: u32,
}

#[repr(C)]
struct GpioV2LineValues {
    bits: u64,
    mask: u64,
}

const GPIO_LED_PIN: u32 = 17; // Using GPIO 17 (pin 11) for the example

fn make_consumer(name: &str) -> [libc::c_char; GPIO_MAX_NAME_SIZE] {
    let mut consumer = [0; GPIO_MAX_NAME_SIZE];

    for (index, byte) in name.bytes().enumerate() {
        if index >= GPIO_MAX_NAME_SIZE - 1 {
            break;
        }

        consumer[index] = byte as libc::c_char;
    }

    consumer
}

fn main() {
    let chip = std::fs::File::open("/dev/gpiochip0").unwrap();

    // Could also use
    // `let mut request: GpioV2LineRequest = unsafe { std::mem::zeroed() };`
    // and then fill in fields manually.
    let mut request = GpioV2LineRequest {
        offsets: [0; GPIO_V2_LINES_MAX],
        consumer: make_consumer("rs-rpi3b-blinky"),
        config: GpioV2LineConfig {
            flags: GPIO_V2_LINE_FLAG_OUTPUT,
            num_attrs: 0,
            padding: [0; 5],
            attrs: [const {
                GpioV2LineConfigAttribute {
                    attr: GpioV2LineAttribute {
                        id: 0,
                        padding: 0,
                        union: LineAttrUnion { flags: 0 },
                    },
                    mask: 0,
                }
            }; GPIO_V2_LINE_NUM_ATTRS_MAX],
        },
        num_lines: 1,
        event_buffer_size: 0,
        padding: [0; 5],
        fd: 0,
    };

    request.offsets[0] = GPIO_LED_PIN;

    let result = unsafe { libc::ioctl(chip.as_raw_fd(), GPIO_V2_GET_LINE_IOCTL, &mut request) };
    assert_eq!(result, 0);

    let line_fd = unsafe { std::os::fd::OwnedFd::from_raw_fd(request.fd) };

    let mut values = GpioV2LineValues { bits: 0, mask: 1 };

    loop {
        values.bits = 1;
        let result = unsafe {
            libc::ioctl(line_fd.as_raw_fd(), GPIO_V2_LINE_SET_VALUES_IOCTL, &values)
        };
        assert_eq!(result, 0);
        std::thread::sleep(std::time::Duration::from_millis(1000));

        values.bits = 0;
        let result = unsafe {
            libc::ioctl(line_fd.as_raw_fd(), GPIO_V2_LINE_SET_VALUES_IOCTL, &values)
        };
        assert_eq!(result, 0);
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}

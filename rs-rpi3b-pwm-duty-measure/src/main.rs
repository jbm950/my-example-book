use gpiocdev::{
    Request,
    line::{EdgeEvent, EdgeKind},
};
use std::{fs, path::Path, thread::sleep, time::Duration};

const CAPTURE_CHIP: &str = "/dev/gpiochip0";
const CAPTURE_PIN: u32 = 10; // Using GPIO 10 (pin 19) for the example

// PWM configured to use GPIO 18 (pin 12) at the Raspberry Pi level
const PWM_CHIP: &str = "/sys/class/pwm/pwmchip0";
const PWM_CHANNEL: &str = "0";
const PWM_PERIOD_NS: u64 = 1_000_000; // 1 ms
const PWM_DUTY_NS: u64 = 500_000; // Amount of nano seconds of the period to be active

const NUM_SAMPLES: usize = 50;

struct Pwm {
    chip: String,
    channel: String,
}

impl Pwm {
    fn new(chip: &str, channel: &str, period_ns: u64, duty_ns: u64) -> std::io::Result<Self> {
        let channel_dir = format!("{chip}/pwm{channel}");

        if !Path::new(&channel_dir).exists() {
            fs::write(format!("{chip}/export"), channel)?;
            // Creating the channel directory is not instantaneous so we wait to
            // continue until it exists.
            while !Path::new(&channel_dir).exists() {
                sleep(Duration::from_millis(1));
            }
        }
        fs::write(format!("{channel_dir}/period"), period_ns.to_string())?;
        fs::write(format!("{channel_dir}/duty_cycle"), duty_ns.to_string())?;
        fs::write(format!("{channel_dir}/enable"), "1")?;

        Ok(Self { chip: chip.to_string(), channel: channel.to_string() })
    }

    fn channel_dir(&self) -> String {
        format!("{}/pwm{}", self.chip, self.channel)
    }
}

impl Drop for Pwm {
    fn drop(&mut self) {
        let _ = fs::write(format!("{}/enable", self.channel_dir()), "0");
        let _ = fs::write(format!("{}/unexport", self.chip), &self.channel);
    }
}

// Assumes clean alternating rising/falling sequence with no missed edges
fn capture_duty_cycles(capture: &Request, samples: &mut [f64]) -> gpiocdev::Result<()> {
    let mut cycle_start: Option<u64> = None;
    let mut high_time = 0.0;

    let mut sample_num = 0;

    for edge in capture.edge_events() {
        if sample_num >= NUM_SAMPLES {
            break;
        }

        match edge {
            Ok(EdgeEvent {
                timestamp_ns,
                kind: EdgeKind::Rising,
                ..
            }) => {
                if let Some(start_time) = cycle_start {
                    samples[sample_num] = high_time / (timestamp_ns - start_time) as f64;
                    sample_num += 1;
                }

                cycle_start = Some(timestamp_ns);
            }
            Ok(EdgeEvent {
                timestamp_ns,
                kind: EdgeKind::Falling,
                ..
            }) => {
                if let Some(start_time) = cycle_start {
                    high_time = (timestamp_ns - start_time) as f64;
                }
            }
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

fn main() -> gpiocdev::Result<()> {
    let _pwm = Pwm::new(PWM_CHIP, PWM_CHANNEL, PWM_PERIOD_NS, PWM_DUTY_NS)?;

    let capture = Request::builder()
        .on_chip(CAPTURE_CHIP)
        .with_consumer("rs-rpi3b-timer-capture")
        .with_line(CAPTURE_PIN)
        .with_edge_detection(gpiocdev::line::EdgeDetection::BothEdges)
        .request()?;

    let mut samples = [0.0; NUM_SAMPLES];
    capture_duty_cycles(&capture, &mut samples)?;

    let avg_duty = samples.iter().sum::<f64>() / samples.len() as f64;
    println!("Avg Duty Cycle: {:.4}", avg_duty);

    Ok(())
}

use std::{fs, path::Path, thread::sleep, time::Duration};

// PWM chip 0 was set up for GPIO18 (pin 12) as the Raspberry Pi level

fn main() {
    let chip = "/sys/class/pwm/pwmchip0";
    let channel = "0";
    let channel_dir = format!("{chip}/pwm{channel}");

    fs::write(format!("{chip}/export"), channel).expect("failed to export pwm channel");
    // Creating the channel directory is not instantaneous so we wait to
    // continue until it exists.
    while !Path::new(&channel_dir).exists() {
        sleep(Duration::from_millis(1));
    }

    let period_ns = 1_000_000; // 1 ms
    fs::write(format!("{channel_dir}/period"), period_ns.to_string())
        .expect("failed to set period");

    let duty_ns = 500_000; // Amount of nano seconds of the period to be active
    fs::write(format!("{channel_dir}/duty_cycle"), duty_ns.to_string())
        .expect("failed to set duty cycle");

    fs::write(format!("{channel_dir}/enable"), "1").expect("failed to enable pwm");

    // Only leave the pwm on for 30 seconds so that we return to an off state
    // when done
    sleep(Duration::from_secs(30));

    fs::write(format!("{channel_dir}/enable"), "0").expect("failed to disable pwm");
    fs::write(format!("{chip}/unexport"), channel).expect("failed to unexport pwm channel");
}

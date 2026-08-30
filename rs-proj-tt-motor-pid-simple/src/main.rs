const PWM_TO_RPM: [(f64, f64); 19] = [
    // Values taken from rs-tt-motor-characterize
    (10.0, 7.831),
    (15.0, 15.855),
    (20.0, 24.389),
    (25.0, 32.299),
    (30.0, 40.933),
    (35.0, 48.976),
    (40.0, 56.791),
    (45.0, 64.818),
    (50.0, 73.253),
    (55.0, 82.622),
    (60.0, 91.475),
    (65.0, 100.405),
    (70.0, 110.489),
    (75.0, 118.305),
    (80.0, 127.856),
    (85.0, 137.056),
    (90.0, 145.524),
    (95.0, 155.672),
    (100.0, 163.202),
];

fn motor_speed_from_pwm(pwm: f64) -> f64 {
    let (min, max) = (PWM_TO_RPM[0].0, PWM_TO_RPM[PWM_TO_RPM.len() - 1].0);
    let pwm = pwm.clamp(min, max);

    PWM_TO_RPM
        .windows(2)
        .find_map(|w| {
            let (pwm0, rpm0) = w[0];
            let (pwm1, rpm1) = w[1];

            (pwm <= pwm1).then(|| rpm0 + (pwm - pwm0) / (pwm1 - pwm0) * (rpm1 - rpm0))
        })
        .expect("PWM was clamped to table range")
}

struct Pid {
    kp: f64,
    ki: f64,
    kd: f64,

    integral: f64,
    prev_error: f64,
}

impl Pid {
    fn new(kp: f64, ki: f64, kd: f64) -> Self {
        Self {
            kp,
            ki,
            kd,
            integral: 0.0,
            prev_error: 0.0,
        }
    }

    fn update(&mut self, setpoint: f64, measured: f64, dt: f64) -> f64 {
        // no anti-windup: intentionally minimal
        let error = setpoint - measured;

        let p_out = self.kp * error;

        self.integral += error * dt;
        let i_out = self.ki * self.integral;

        let derivative = (error - self.prev_error) / dt;
        let d_out = self.kd * derivative;

        self.prev_error = error;

        p_out + i_out + d_out
    }
}

// Motor model is intentionally crude
fn update_motor_speed(motor_speed: f64, pwm: f64, dt: f64) -> f64 {
    const MOTOR_RESPONSE: f64 = 5.0; // Value chosen arbitrarily rather than empirically

    let steady_state_speed = motor_speed_from_pwm(pwm);

    motor_speed + (steady_state_speed - motor_speed) * MOTOR_RESPONSE * dt
}

fn main() {
    // PID gains are arbitrary rather than tuned
    let mut pid = Pid::new(0.5, 0.1, 0.01);

    let setpoint = 95.0;
    let mut motor_speed = 0.0;
    let dt = 0.01;

    println!("Step | Setpoint | Measured RPM |    Error |  PWM CMD");

    for step in 0..20 {
        let measured_speed = motor_speed;

        let pwm = pid.update(setpoint, measured_speed, dt).clamp(0.0, 100.0);

        motor_speed = update_motor_speed(motor_speed, pwm, dt);

        println!(
            "{:>4} | {:>8.1} | {:>12.1} | {:>8.1} | {:>8.1}",
            step,
            setpoint,
            measured_speed,
            setpoint - measured_speed,
            pwm,
        );
    }
}

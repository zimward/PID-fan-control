pub struct PID {
    p: f32,
    i: f32,
    d: f32,
    setpoint: f32,
    prev_error: f32,
    integral: f32,
}

impl PID {
    pub fn new(p: f32, i: f32, d: f32, setpoint: f32) -> PID {
        PID {
            p: p / 1000.0,
            i: i / 1000.0,
            d: d / 1000.0,
            setpoint,
            prev_error: 0f32,
            integral: 0f32,
        }
    }
    pub fn run(&mut self, is: f32, delta_t: f32) -> f32 {
        let error = self.setpoint - is;
        let mut output = error * self.p;
        output += ((error - self.prev_error) / delta_t) * self.d;
        self.prev_error = error;
        //clamping output to prevent wind-up
        if output + self.integral * self.i < 1.0f32 && output + self.integral * self.i > 0.0 {
            self.integral += error * delta_t;
        }
        output += self.integral * self.i;
        f32::max(f32::min(output, 1.0f32), 0.0)
    }
}

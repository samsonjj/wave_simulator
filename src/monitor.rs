pub struct Monitor {
    pub display_val: f32,
    acc: f32,
    window: usize,
    step: usize,
}

impl Monitor {
    pub fn new() -> Self {
        Monitor {
            display_val: 0.,
            acc: 0.,
            window: 120,
            step: 0,
        }
    }

    pub fn inc(&mut self, val: f32) {
        self.acc += val;
        self.step += 1;
        if self.step == self.window {
            self.step = 0;
            self.display_val = self.acc / self.window as f32;
            self.acc = 0.;
        }
    }
}


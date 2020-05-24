use std::time::*;

pub struct FpsCounter {
    update_frequency: f64,
    total_time: f64,
    total_frame_count: u32,
    current_value: f64,
}

impl FpsCounter {
    pub fn new(update_frequency: f64) -> FpsCounter {
        FpsCounter {
            update_frequency,
            total_time: 0.0,
            total_frame_count: 0,
            current_value: 0.0,
        }
    }

    pub fn next_frame(&mut self, dt: f64) {
        self.total_time += dt;
        self.total_frame_count += 1;

        if self.total_time > self.update_frequency {
            let fps = self.total_frame_count as f64 / self.total_time;
            self.current_value = fps;

            self.total_time %= self.update_frequency;
            self.total_frame_count = 0;
        }
    }

    pub fn get_fps(&self) -> f64 {
        self.current_value
    }
}

pub struct FrameLimiter {
    pub desired_fps: f64,
    frame_duration: Duration,
    before_render_time: Instant,
}

impl FrameLimiter {
    pub fn new(desired_fps: f64) -> FrameLimiter {
        FrameLimiter {
            desired_fps,
            frame_duration: Duration::from_secs_f64(1.0 / desired_fps),
            before_render_time: Instant::now(),
        }
    }

    pub fn before_update(&mut self) {
        self.before_render_time = Instant::now();
    }

    pub fn after_render(&mut self) {
        let time_taken = Instant::now() - self.before_render_time + Duration::from_secs_f64(0.001);
        if self.frame_duration > time_taken {
            std::thread::sleep(self.frame_duration - time_taken);
        }
    }
}

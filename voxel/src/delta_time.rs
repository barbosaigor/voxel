use std::time;

pub struct DeltaTime {
    pub last: time::Duration,
    pub dt: time::Duration,
}

impl Default for DeltaTime {
    fn default() -> Self {
        Self{
            last: now(),
            dt: time::Duration::from_secs(0),
        }
    }
}

impl DeltaTime {
    pub fn tick(&mut self) -> time::Duration {
        let now = now();
        self.dt = now - self.last;
        self.last = now;

        self.dt
    }
}

fn now() -> time::Duration {
    time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap()
}
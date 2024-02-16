use std::time::Instant;

#[derive(Debug)]
pub struct Timers {
    pub(super) dt: u8,
    pub(super) st: u8,
    last_tick: Instant,
}

impl Timers {
    const HZ_60: u128 = 1_000_000_000 / 60;

    pub(super) fn tick(&mut self) {
        let elapsed = self.last_tick.elapsed();
        if elapsed.as_nanos() >= Timers::HZ_60 {
            if self.dt > 0 {
                self.dt -= 1;
            }

            if self.st > 0 {
                self.st -= 1;
            }

            self.last_tick = Instant::now();
        }
    }

    pub fn delay(&self) -> u8 {
        self.dt
    }

    pub fn sound(&self) -> u8 {
        self.st
    }
}

impl Default for Timers {
    fn default() -> Self {
        Timers {
            dt: 0,
            st: 0,
            last_tick: Instant::now(),
        }
    }
}
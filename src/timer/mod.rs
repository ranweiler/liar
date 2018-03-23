use libc;


pub trait Timer {
    fn mark(&mut self);
    fn since(&self, prev: &Self) -> f64;
}

pub struct CPUTimer {
    ts: libc::timespec,
}

impl CPUTimer {
    pub fn new() -> Self {
        let ts = libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };

        CPUTimer { ts }
    }

    fn to_f64(&self) -> f64 {
        (self.ts.tv_sec as f64) * 1e9 + (self.ts.tv_nsec as f64)
    }
}


impl Timer for CPUTimer {
    fn mark(&mut self) {
        unsafe {
            libc::clock_gettime(libc::CLOCK_PROCESS_CPUTIME_ID, &mut self.ts);
        }
    }

    fn since(&self, prev: &CPUTimer) -> f64 {
        let d = self.to_f64() - prev.to_f64();

        d.max(0.0)
    }
}

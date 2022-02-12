use std::thread;
use std::time::Duration;

pub struct Th {
    number: u32,
    wait_time: u64,
}

impl Th {
    pub fn new(number: u32, wait_time: u64) -> Self {
        Th { number, wait_time }
    }
    pub fn run(&self) {
        println!(
            "Thread {} ha empezado y tiene un tiempo de espera {}",
            self.number, self.wait_time
        );
        let time = Duration::from_millis(self.wait_time);
        thread::sleep(time);
        println!(
            "Thread {} ha terminado y tenia un tiempo de espera {}",
            self.number, self.wait_time
        );
    }
    pub fn start(self) -> std::thread::JoinHandle<()> {
        thread::spawn(move || {
            self.run();
        })
    }
}

use std::sync::{Arc, Mutex};

pub struct DiningPhilosophers {
    forks: Arc<Vec<Mutex<()>>>,
}

impl DiningPhilosophers {
    pub fn new() -> Self {
        DiningPhilosophers {
            forks: Arc::new((0..5).map(|_| Mutex::new(())).collect()),
        }
    }

    pub fn wants_to_eat<F1, F2, F3, F4, F5>(
        &self,
        philosopher: i32,
        pick_left_fork: F1,
        pick_right_fork: F2,
        eat: F3,
        put_left_fork: F4,
        put_right_fork: F5,
    ) where
        F1: FnOnce(),
        F2: FnOnce(),
        F3: FnOnce(),
        F4: FnOnce(),
        F5: FnOnce(),
    {
        let left = philosopher.clone() as usize;
        let right = ((philosopher.clone() + 1) % 5) as usize;

        if philosopher % 2 == 0 {
            let _right = self.forks[right].lock().unwrap();
            let _left = self.forks[left].lock().unwrap();
            pick_left_fork();
            pick_right_fork();
            eat();
            put_left_fork();
            put_right_fork()
        } else {
            let _left = self.forks[left].lock().unwrap();
            let _right = self.forks[right].lock().unwrap();
            pick_left_fork();
            pick_right_fork();
            eat();
            put_left_fork();
            put_right_fork();
        }
    }
}

fn main() {}

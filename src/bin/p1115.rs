fn print_foo() {
    print!("foo");
}
fn print_bar() {
    print!("bar");
}
use std::sync::{Condvar, Mutex};

struct FooBar {
    n: usize,
    state_pair: (Mutex<bool>, Condvar),
}
impl FooBar {
    fn run_func_times<F>(&self, func: F, func_turn: bool)
    where
        F: Fn(),
    {
        let (lock, cvar) = &self.state_pair;
        for _ in 0..self.n {
            let mut is_my_turn = lock.lock().unwrap();
            while *is_my_turn != func_turn {
                is_my_turn = cvar.wait(is_my_turn).unwrap();
            }
            *is_my_turn = !func_turn;
            cvar.notify_one();
            drop(is_my_turn);
            func();
        }
    }
    fn new(n: usize) -> Self {
        FooBar {
            n,
            state_pair: (Mutex::new(true), Condvar::new()),
        }
    }

    fn foo<F>(&self, print_foo: F)
    where
        F: Fn(),
    {
        Self::run_func_times(&self, print_foo, true);
    }

    fn bar<F>(&self, print_bar: F)
    where
        F: Fn(),
    {
        Self::run_func_times(&self, print_bar, false);
    }
}
fn main() {
    let tmp = FooBar::new(10);
    tmp.foo(print_foo);
    tmp.bar(print_bar);
}

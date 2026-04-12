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
    let fb = FooBar::new(1);
    fb.foo(print_foo);
    fb.bar(print_bar);
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn test_alternating_output() {
        let n = 5;
        let foobar = Arc::new(FooBar::new(n));
        let results = Arc::new(Mutex::new(Vec::new()));

        let res_foo = Arc::clone(&results);
        let res_bar = Arc::clone(&results);
        let fb_foo = Arc::clone(&foobar);
        let fb_bar = Arc::clone(&foobar);

        let t1 = thread::spawn(move || {
            fb_foo.foo(move || {
                res_foo.lock().unwrap().push("foo");
            });
        });

        let t2 = thread::spawn(move || {
            fb_bar.bar(move || {
                res_bar.lock().unwrap().push("bar");
            });
        });

        t1.join().unwrap();
        t2.join().unwrap();

        let actual = results.lock().unwrap();
        let expected: Vec<&str> = (0..n).flat_map(|_| vec!["foo", "bar"]).collect();
        assert_eq!(*actual, expected);
    }
}

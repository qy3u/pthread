use std::mem::zeroed;

use libc::{cpu_set_t, pthread_self, pthread_setaffinity_np, CPU_SET, CPU_ZERO};

pub fn setaffinity_np(cores: &[usize]) {
    unsafe {
        let mut set: cpu_set_t = zeroed();
        CPU_ZERO(&mut set);

        let thread = pthread_self();

        for i in cores {
            CPU_SET(*i, &mut set);
        }

        let s = pthread_setaffinity_np(thread, std::mem::size_of::<cpu_set_t>(), &mut set);

        if s != 0 {
            panic!("bind failed");
        }
    }
}

#[test]
fn test_bind_core() {
    fn fib(n: usize) -> usize {
        if n == 1 || n == 2 {
            return 1;
        }

        fib(n - 1) + fib(n - 2)
    }

    setaffinity_np(&[1]);

    let n = fib(48);
    println!("{n}")
}

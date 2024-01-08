#[allow(dead_code)]
use std::sync::OnceLock;
use std::{cell::RefCell, sync::atomic::AtomicU64};

struct Counter {
    x: u32,
}

impl Counter {
    fn double(&self, y: u32) -> u32 {
        self.x * y
    }
}

fn counter() -> &'static Counter {
    static COUNTER: OnceLock<Counter> = OnceLock::new();
    COUNTER.get_or_init(|| Counter { x: 42 })
}

static COUNTER: AtomicU64 = AtomicU64::new(0);

thread_local!(static ONZIN: RefCell<Vec<u32>> = RefCell::new(Vec::<u32>::new()));

fn main() {
    let c = counter();

    println!("{}", c.double(42));

    let h = std::thread::spawn(|| {
        COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        println!("{}", COUNTER.load(std::sync::atomic::Ordering::Relaxed));
    });

    println!("{}", COUNTER.load(std::sync::atomic::Ordering::Relaxed));
    COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    println!("{}", COUNTER.load(std::sync::atomic::Ordering::Relaxed));

    ONZIN.with(|v| {
        let mut v2 = v.borrow_mut();
        v2.push(42);
    });
    ONZIN.with(|v| {
        println!("{}", v.borrow().get(0).unwrap());
    });

    h.join().unwrap();
}

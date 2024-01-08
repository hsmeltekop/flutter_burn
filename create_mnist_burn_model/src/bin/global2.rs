use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

struct S {
    x: u32,
}

impl S {
    fn up(&mut self) {
        self.x += 1;
    }
}

impl Default for S {
    fn default() -> Self {
        Self {
            x: Default::default(),
        }
    }
}

lazy_static! {
    static ref ONZIN: Arc<Mutex<S>> = Arc::new(Mutex::new(S::default()));
}

fn main() {
    {
        let mut o = ONZIN.lock().unwrap();
        o.up();
    }

    let mut hs = vec![];

    for _ in 0..5 {
        let data = Arc::clone(&ONZIN);
        let h = std::thread::spawn(move || {
            let mut o = data.lock().unwrap();
            o.up();
        });
        hs.push(h);
    }

    for h in hs {
        h.join().unwrap();
    }

    {
        let mut o = ONZIN.lock().unwrap();
        o.up();
    }

    println!("{}", ONZIN.lock().unwrap().x);
}

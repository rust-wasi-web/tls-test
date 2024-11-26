use std::cell::Cell;

fn main() {
    println!("Available parallelism: {:?}", std::thread::available_parallelism());

    TLS.set(MyType("main 1"));
    TLS.set(MyType("main 2"));

    let hnd = std::thread::spawn(my_thread);
    hnd.join().unwrap();
    println!("thread joined");
}

pub struct MyType(&'static str);

impl Drop for MyType {
    fn drop(&mut self) {
        println!("dropping: {}", self.0);
    }
}

thread_local! {
    pub static TLS: Cell<MyType> = Cell::new(MyType("thread init value"));
    pub static TLS_B: Cell<MyType> = Cell::new(MyType("thread init value B"));
}

pub fn my_thread() {
    println!("my thread started");
    TLS.set(MyType("thread set value 1"));
    TLS.set(MyType("thread set value 2"));
    TLS_B.set(MyType("thread set value 2 B"));
    println!("my thread ending");
}

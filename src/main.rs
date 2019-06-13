use std::{thread, time};

fn main() {

    let time = time::Duration::from_secs(1);
    let time2 = time::Duration::from_secs(10);

    loop {
        println!("Hello, worlds !");
        thread::sleep(time);
        println!("good by, worlds !");
        thread::sleep(time2);
    }
}

use std::{thread, time};

fn main() {

    let time = time::Duration::from_secs(1);

    loop {
        println!("Hello, worldsss !");
        thread::sleep(time);
    }
}

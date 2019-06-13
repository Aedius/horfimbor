use std::{thread, time};

fn main() {
    let time = time::Duration::from_secs(1);

    loop {
        println!("Hello, World ! I'm lignée server !");
        thread::sleep(time);
    }
}

pub fn get_world_name() -> &'static str {
    "World"
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_world_name() {
        assert_eq!(get_world_name(), "World");
    }
}
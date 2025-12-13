use std::{io, os::windows};

use kasane_logic::{error::Error, id::space_id::single::SingleID};

fn main() {
    let mut write = true;
    let mut id: SingleID;

    let mut text: String = "neko".to_string();
    let cut = text.split_off(3);

    while write {
        let result = user();

        // if result.is_ok() {
        //     id = result.unwrap();
        //     write = false;
        //     println!("{}", id);
        // } else {
        //     println!("あなたの入力はエラーです");
        //     continue;
        // }

        match result {
            Ok(v) => {
                println!("{}", v);
                return;
            }
            Err(e) => {
                println!("{}", e);
                continue;
            }
        }
    }
}

fn user() -> Result<SingleID, Error> {
    println!("Zを入力してください");
    let z = read_buffer();
    println!("Fを入力してください");
    let f = read_buffer();
    println!("Xを入力してください");
    let x = read_buffer();
    println!("Yを入力してください");
    let y = read_buffer();

    SingleID::new(z as u8, f as i64, x as u64, y as u64)
}

fn read_buffer() -> u32 {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => buffer.trim().parse().expect("Failed to parse."),
        Err(e) => panic!("Failed to read line: {}", e),
    }
}

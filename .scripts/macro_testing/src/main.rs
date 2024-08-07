use rdev::{listen, Event};
use rdev::{simulate, Button, EventType, Key, SimulateError};
use std::{thread, time};

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
    println!("My callback {:?}", event);
    if let Some(string) = event.name {
        println!("User wrote {:?}", string);
    }
}

fn main() {
    // collect args
    //     let args: Vec<String> = std::env::args().collect();
    //     for arg in args.iter().skip(1) {
    //         write(arg);
    //         // write a space after every word
    //         send(&EventType::KeyPress(Key::Space));
    //         send(&EventType::KeyRelease(Key::Space));
    //     }

    // This will block.
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}

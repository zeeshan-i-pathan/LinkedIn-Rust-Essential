use rand::prelude::*;
use std::io;
fn main() {
    let mut x: u16 = 255;
    println!("x is {x}");
    x = x * 2;
    println!("x is {x}");
    let array = [[0; 3]; 2];
    println!("array is {}", array[0][0]);
    let result = square(13);
    println!("Result is {:?}", result);
    let celsius_temp = 23.0;
    let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);
    assert_eq!(fahrenheit_temp, 73.4);
    println!("Test Passed");
    let make_x_odd = true;
    let x = if make_x_odd { 1 } else { 2 };
    println!("X is {x}");
    let mut count = 0;
    let x = loop {
        if count == 10 {
            break count * 10;
        }
        count += 1;
    };
    println!("X is {x} after the loop");
    let message = ['h', 'e', 'l', 'l', 'o'];
    for (index, &item) in message.iter().enumerate() {
        println!("item {} is {}", index, item);
        if item == 'e' {
            break;
        }
    }
    for number in 0..5 {
        println!("Number is {number}");
    }
    let mut matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    for row in matrix.as_mut() {
        for num in row.as_mut() {
            *num += 10;
            print!("{num}\t");
        }
        println!();
    }
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32 = numbers[0];
    let mut min: i32 = numbers[0];
    let mut mean: f64 = 0.0;

    for number in numbers {
        if max < number {
            max = number;
        }
        if min > number {
            min = number;
        }
        mean += number as f64;
    }

    mean /= numbers.len() as f64;

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Test Passed");
    let mut message = String::from("Earth");
    message += " is Home";
    println!("message {message}");

    let message = String::from("Greetings from Earth!");
    println!("message is {message}");
    let last_word = &message[15..];
    println!("last_word is {last_word}");
    let planets = [1, 2, 3, 4, 5, 6, 7, 8];
    let inner_planets: &[i32] = &planets[..4];
    println!("inner planets are {:?}", inner_planets);
    let first_word = get_first_word(&message);
    println!("First word is {first_word}");
    let test1 = "We need more space.";
    assert_eq!(trim_spaces(test1), "We need more space.");
    let test2 = "   There's space in the front.";
    assert_eq!(trim_spaces(test2), "There's space in the front.");
    let test3 = "There's space to the rear.   ";
    assert_eq!(trim_spaces(test3), "There's space to the rear.");
    let test4 = "  We're surrounded by space!    ";
    assert_eq!(trim_spaces(test4), "We're surrounded by space!");
    let test5 = "     ";
    assert_eq!(trim_spaces(test5), "");
    let test6 = "";
    assert_eq!(trim_spaces(test6), "");
    let test7 = " 🚀 ";
    assert_eq!(trim_spaces(test7), "🚀");
    println!("Trim Test Passed");
    let mut buffer = String::new();
    println!("Enter a message");
    io::stdin().read_line(&mut buffer);
    println!("Buffer is {buffer}");
    // let number = buffer.trim().parse::<i32>(); or
    let number: i32 = buffer.trim().parse().unwrap();
    println!("number + 1 is {}", number + 1);
    let number: f64 = random();
    println!("number is {number}");
    let number = thread_rng().gen_range(1..11);
    println!("number is {number}");
}

fn trim_spaces(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut startIndex = 0;
    let mut endIndex = s.len();
    for &item in bytes {
        if item != b' ' {
            break;
        }
        startIndex += 1;
    }
    if startIndex != endIndex {
        for &item in bytes.iter().rev() {
            if item != b' ' {
                break;
            }
            endIndex -= 1;
        }
    }
    &s[startIndex..endIndex]
}

fn get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index];
        }
    }
    &s
}

fn square(x: i32) -> (i32, i32) {
    println!("Squaring {x}");
    (x, x * x)
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 1.8) + 32.0
}

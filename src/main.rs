use std::io::{self, Write};

fn main() {
    println!("String to binary representation converter");
    'start: loop {
        let mode = choose_mode();
        loop {
            match mode {
                1 => {
                    let data = input("Input text value:");
                    let binary = to_binary_string(data);
                    println!("Output:\n{binary}\n");
                }
                2 => {
                    let data = input("Input binary value:");

                    if let Some(string) = from_binary_string(data) {
                        println!("Output:\n{string}");
                    } else {
                        eprintln!("[!] Incorrect value!");
                        continue;
                    }
                }
                _ => {
                    eprintln!("[!] Incorrect input!");
                    break;
                }
            }

            if answer() {
                continue;
            } else {
                continue 'start;
            }
        }
    }
}

fn answer() -> bool {
    loop {
        let next = input("Continue [Y/n]?");
        if yn::yes(next.trim()) {
            return true;
        } else if yn::no(next.trim()) {
            return false;
        }
        continue;
    }
}

fn choose_mode() -> i32 {
    println!("Choose the mode:");
    println!("1) String to binary mode");
    println!("2) Binary to string mode");

    if let Ok(num) = input("Enter program id:").trim().parse() {
        num
    } else {
        0
    }
}

fn input(message: &str) -> String {
    print!("{message} ");
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn to_binary_string(value: String) -> String {
    let mut binary_string = String::new();

    for char in value.clone().into_bytes() {
        binary_string += &format!("{:08b} ", char);
    }
    binary_string
}

fn from_binary_string(bin_str: String) -> Option<String> {
    let vec: Vec<&str> = bin_str.trim().split(' ').collect();
    let mut bytes: Vec<u8> = vec![];

    for i in vec.iter() {
        if let Ok(byte) = u8::from_str_radix(i, 2) {
            bytes.push(byte);
        } else {
            return None;
        }
    }
    if let Ok(v) = std::str::from_utf8(&bytes) {
        return Some(v.to_string());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const STRING: &str = "🦀 Rustacean";
    const BINARY: &str = "11110000 10011111 10100110 10000000 00100000 01010010 01110101 01110011 \
                         01110100 01100001 01100011 01100101 01100001 01101110 ";

    #[test]
    fn to_binary() {
        assert_eq!(to_binary_string(STRING.to_string()), BINARY);
    }

    #[test]
    fn from_binary() {
        assert_eq!(
            from_binary_string(BINARY.to_string()).unwrap(),
            STRING.to_string()
        );
    }
}

use std::io::{self, Write};

fn main() {
    loop {
        let mut input = String::new();

        print!("1) String to binary\n2) Binary to string\nEnter program id > ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let number = if let Ok(num) = input.trim().parse() {
            num
        } else {
            eprintln!("[!] Incorrect input!");
            continue;
        };
        let mut input = String::new();

        match number {
            1 => {
                println!("Input text value:");
                io::stdin().read_line(&mut input).unwrap();
                println!();
                let binary = to_binary_string(input);
                print!("Output:\n{binary}\n\n");
            }
            2 => {
                println!("Input binary value:");
                io::stdin().read_line(&mut input).unwrap();
                println!();
                if let Some(string) = from_binary_string(input) {
                    print!("Output:\n{string}\n");
                } else {
                    eprintln!("[!] Incorrect value!");
                    continue;
                }
            }
            _ => {
                eprintln!("[!] Incorrect input!");
            }
        }
    }
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

    for v in vec.iter() {
        if let Ok(byte) = u8::from_str_radix(v, 2) {
            bytes.push(byte);
        } else {
            return None;
        }
    }
    if let Ok(v) = std::str::from_utf8(&bytes) {
        return Some(v.trim_end_matches(' ').to_string());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_binary() {
        let binary_string = to_binary_string("🦀 Rustacean".to_string());

        assert_eq!(
            binary_string,
            "11110000 10011111 10100110 10000000 00100000 01010010 01110101 01110011 01110100 01100001 01100011 01100101 01100001 01101110 ".to_string(),
        );
    }

    #[test]
    fn from_binary() {
        let binary_string = "11110000 10011111 10100110 10000000 00100000 01010010 01110101 01110011 01110100 01100001 01100011 01100101 01100001 01101110 ".to_string();
        let string = from_binary_string(binary_string).unwrap();

        assert_eq!(string, "🦀 Rustacean".to_string());
    }
}

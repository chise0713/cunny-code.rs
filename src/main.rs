use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, IsTerminal, Read};

struct CunnyCode {
    encode_map: HashMap<char, &'static str>,
    decode_map: HashMap<&'static str, char>,
}

impl CunnyCode {
    fn new() -> Self {
        let encode_map: HashMap<char, &'static str> = [
            ('A', "😭💢"),
            ('B', "💢😭😭😭"),
            ('C', "💢😭💢😭"),
            ('D', "💢😭😭"),
            ('E', "😭"),
            ('F', "😭😭💢😭"),
            ('G', "💢💢😭"),
            ('H', "😭😭😭😭"),
            ('I', "😭😭"),
            ('J', "😭💢💢💢"),
            ('K', "💢😭💢"),
            ('L', "😭💢😭😭"),
            ('M', "💢💢"),
            ('N', "💢😭"),
            ('O', "💢💢💢"),
            ('P', "😭💢💢😭"),
            ('Q', "💢💢😭💢"),
            ('R', "😭💢😭"),
            ('S', "😭😭😭"),
            ('T', "💢"),
            ('U', "😭😭💢"),
            ('V', "😭😭😭💢"),
            ('W', "😭💢💢"),
            ('X', "💢😭😭💢"),
            ('Y', "💢😭💢💢"),
            ('Z', "💢💢😭😭"),
            ('0', "💢💢💢💢💢"),
            ('1', "😭💢💢💢💢"),
            ('2', "😭😭💢💢💢"),
            ('3', "😭😭😭💢💢"),
            ('4', "😭😭😭😭💢"),
            ('5', "😭😭😭😭😭"),
            ('6', "💢😭😭😭😭"),
            ('7', "💢💢😭😭😭"),
            ('8', "💢💢💢😭😭"),
            ('9', "💢💢💢💢😭"),
            ('.', "😭💢😭💢😭💢"),
            (',', "💢💢😭😭💢💢"),
            ('!', "💢😭💢😭💢💢"),
            ('?', "😭😭💢💢😭😭"),
            ('\'', "😭💢💢💢💢😭"),
            ('"', "😭💢😭😭💢😭"),
            ('/', "💢😭😭💢😭"),
            ('(', "💢😭💢💢😭"),
            (')', "💢😭💢💢😭💢"),
            (':', "💢💢💢😭😭😭"),
            (';', "💢😭💢😭💢😭"),
            ('=', "💢😭😭😭💢"),
            ('+', "😭💢😭💢😭"),
            ('-', "💢😭😭😭😭💢"),
            ('_', "😭😭💢💢😭💢"),
            ('@', "😭💢💢😭💢😭"),
            ('`', "😭😭💢💢💢💢"),
            ('~', "😭😭😭💢💢💢"),
            ('\\', "💢😭😭💢💢"),
            ('|', "💢💢💢😭💢"),
            ('#', "😭😭😭💢😭💢"),
            ('$', "😭😭😭💢😭😭💢"),
            ('%', "💢😭😭💢😭💢"),
            ('^', "💢💢💢😭💢💢💢"),
            ('*', "😭💢😭💢💢"),
            ('{', "💢💢💢😭💢😭"),
            ('}', "💢💢💢😭😭💢"),
            ('[', "💢😭😭💢😭😭"),
            (']', "💢😭😭💢💢😭"),
            ('<', "😭😭😭😭💢💢"),
            ('>', "💢💢😭😭😭😭"),
        ]
        .iter()
        .cloned()
        .collect();

        let decode_map: HashMap<&'static str, char> =
            encode_map.iter().map(|(&k, &v)| (v, k)).collect();

        CunnyCode {
            encode_map,
            decode_map,
        }
    }

    fn encode(&self, input: &str) -> String {
        let mut result = String::with_capacity(input.len() * 5);
        let mut unencoded_chars = Vec::new();
        for c in input.chars() {
            if c == '\n' {
                result.push('\n');
            } else if c.is_whitespace() {
                result.push(' ');
            } else if let Some(&encoded) = self.encode_map.get(&c.to_ascii_uppercase()) {
                if c.is_ascii_uppercase() {
                    result.push_str(&format!("^{} ", encoded));
                } else {
                    result.push_str(&format!("{} ", encoded));
                }
            } else {
                unencoded_chars.push(c);
            }
        }
        if !unencoded_chars.is_empty() {
            eprintln!(
                "Error: The following could not be encoded: {}",
                unencoded_chars.iter().collect::<String>()
            )
        }
        result.trim_end().to_string()
    }

    fn decode(&self, input: &str) -> String {
        let mut decoded = String::with_capacity(input.len());
        let parts = input.split("\n");
        let mut undecoded_parts = Vec::new();
        for outter_part in parts {
            let inner_parts = outter_part.split(" ");
            for part in inner_parts {
                let part = part.trim();
                if part.is_empty() {
                    decoded.push_str(" ");
                } else if part.starts_with('^') {
                    if let Some(&c) = self.decode_map.get(&part[1..]) {
                        decoded.push(c);
                    } else {
                        undecoded_parts.push(part);
                    }
                } else if let Some(&c) = self.decode_map.get(part) {
                    decoded.push(c.to_ascii_lowercase());
                } else if !part.is_empty() {
                    undecoded_parts.push(part);
                }
            }
            decoded.push('\n');
        }

        if !undecoded_parts.is_empty() {
            eprintln!(
                "Error: The following could not be decoded: {}",
                undecoded_parts.join(" ")
            )
        }

        decoded.trim_end().to_string()
    }
}

fn main() -> io::Result<()> {
    let cunny_code = CunnyCode::new();
    let args: Vec<String> = env::args().skip(1).collect();

    let input = if args.is_empty() {
        let stdin = io::stdin();
        let mut input = String::new();
        if !stdin.is_terminal() {
            stdin.lock().read_to_string(&mut input)?;
        } else {
            eprintln!(
                "You need to paste your Cunny Code into the {{ STDIN | FILE | ARGS }} first, then I can decode it for you!"
            );
            return Ok(());
        }
        input
    } else {
        match File::open(&args[0]) {
            Ok(file) => {
                let mut reader = io::BufReader::new(file);
                let mut input = String::new();
                reader.read_to_string(&mut input)?;
                input
            }
            Err(_) => args.join(" "),
        }
    };

    let output = match input.chars().next() {
        Some('😭') | Some('💢') => cunny_code.decode(&input),
        Some('^')
            if input
                .chars()
                .nth(1)
                .map_or(false, |c| c == '😭' || c == '💢') =>
        {
            cunny_code.decode(&input)
        }
        _ => cunny_code.encode(&input),
    };

    println!("{}", output);
    Ok(())
}

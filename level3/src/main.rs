use std::{error::Error, fs::File, io::Read};

fn main() -> Result<(), Box<dyn Error>> {
    let to_decode = std::env::args()
        .nth(1)
        .expect("name of file containing encoded text as first arg");

    let mut buf = String::new();
    File::open(to_decode)?.read_to_string(&mut buf)?;

    let mut guesses = (1..=25)
        .map(|radix| {
            let map = cipher_map(radix);
            (
                radix,
                buf.split("\n")
                    .map(|line| line.chars().map(|ch| cipher(ch, &map)).collect::<String>())
                    .collect::<Vec<_>>(),
            )
        })
        .map(|(radix, lines)| {
            (
                radix,
                lines.clone(),
                lines.iter().map(|s| score(s)).sum::<usize>(),
            )
        })
        .collect::<Vec<_>>();

    guesses.sort_by(|a, b| a.2.cmp(&b.2));

    let likeliest = guesses.last().unwrap();

    println!("most likely cipher: left shift by {}", likeliest.0);
    println!("which results in:");
    println!("==========================");
    likeliest.1.iter().for_each(|s| println!("{s}"));

    Ok(())
}

fn cipher_map(radix: u32) -> [char; 26] {
    let base = 'a'..='z';

    let mut map = base.collect::<Vec<char>>();
    map.rotate_left(radix as _);
    map.try_into().unwrap()
}

fn cipher(x: char, cipher_map: &[char; 26]) -> char {
    if !x.is_alphabetic() {
        return x;
    }
    let base_a = x as u32
        - if x.is_ascii_uppercase() {
            'A' as u32
        } else {
            'a' as u32
        };

    cipher_map[base_a as usize]
}

const COMMON_WORDS: &[&str] = &["never", "give", "up", "you", "want", "the", "we"];

fn score(s: &str) -> usize {
    let mut score = 0;
    let lower = s.to_lowercase();

    for q in COMMON_WORDS {
        if lower.contains(q) {
            score += 1;
        }
    }

    score
}

use clap::Parser;
use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::{stdout, BufRead, BufReader, Write};
use std::time::Instant;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t = 100)]
    iterations: u32,

    #[clap(short, long, default_value_t = 10)]
    max: u8,
}

fn main() {
    let args = Args::parse();
    let valid_words = read_words();
    let mut matches: Vec<String> = Vec::new();

    let now = Instant::now();

    let mut index = 1;

    while index < args.iterations {
        print!("\rProcessing {} of {}...", index, args.iterations);
        stdout().flush().unwrap();

        let word = generate_word(&args.max);

        if valid_words.contains(&word) {
            matches.push(word);
        }

        index = index + 1;
    }

    println!(
        "Found the following matches: {:?} in {}s",
        matches,
        now.elapsed().as_secs()
    );
}

fn generate_word(max: &u8) -> String {
    let mut word = String::from("");

    let mut rng = thread_rng();

    let len: u8 = rng.gen_range(5..*max);

    let chars = vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    for _x in 1..len {
        let random_idx = rng.gen_range(1..chars.len());

        word.push(chars[random_idx]);
    }

    return word;
}

fn read_words() -> Vec<String> {
    let mut words: Vec<String> = Vec::new();

    // Note: This is only going to work on a unix system
    let file = File::open("/usr/share/dict/words").expect("Failed to read dict file");

    let buffer = BufReader::new(file);

    for line in buffer.lines() {
        words.push(line.expect("Could not read line").to_uppercase());
    }

    return words;
}

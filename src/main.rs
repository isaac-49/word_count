use std::{env,fs};
use word_count::show_count_words;


/// 教程https://mp.weixin.qq.com/s/G5LiYf7nhHlgnkb8DwoHRw
///
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", args[0]);
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path<top_n>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];
    let top_n: usize = if args.len() > 3 {
        // String parse to usize, default 10
        args[2].parse().unwrap_or(10)
    } else {
        10
    };
    let content = fs::read_to_string(file_path).expect("Could not read file");
    show_count_words(&content, top_n);
}

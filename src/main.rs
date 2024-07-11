use std::{collections::HashMap, env, fs, usize};

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
fn show_count_words(content: &str, top_n: usize) {
    let word_count = count_words(content);
    let mut word_count_vec: Vec<(&String, &usize)> = word_count.iter().collect();
    word_count_vec.sort_by(|a, b| b.1.cmp(a.1));
    print_histogram(&word_count_vec, top_n);
}

fn count_words(content: &str) -> HashMap<String, usize> {
    let mut word_count = HashMap::new();
    for word in content.split_whitespace() {
        let word = word.trim_matches(|c: char| !c.is_alphabetic());
        if word.is_empty() {
            continue;
        }
        *word_count.entry(word.to_string()).or_insert(0) += 1;
    }
    word_count
}

fn print_histogram(word_count_vec: &Vec<(&String, &usize)>, top_n: usize) {
    println!("Top {} words", top_n);
    let max_count = word_count_vec.first().map(|(_, &count)| count).unwrap_or(0);
    for (word, &count) in word_count_vec.iter().take(top_n) {
        let ratio = count as f64 / max_count as f64;
        let repeat = if max_count > 10 {
            (ratio * 10.0).round() as usize
        } else {
            count
        };
        let starts = "*".repeat(repeat);
        // 对齐
        // 111111111 : *********  9
        // 2222222   : *******    7
        // |---10---|  |---10---|
        println!("{:10}: {:10} {}", word,starts,count);
    }
}

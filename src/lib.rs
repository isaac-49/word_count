use std::{collections::HashMap, usize};

pub fn show_count_words(content: &str, top_n: usize) {
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
        println!("{:10}: {:10} {}", word, starts, count);
    }
}

#[cfg(test)]
mod test {
    use rand::{self, Rng};
    use std::{collections::HashMap, str::FromStr, usize};

    use crate::{count_words, print_histogram};

    #[test]
    fn test_print_histogram() {
        let mut map: HashMap<String, usize> = HashMap::new();
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let count: usize = rng.gen_range(1..100);
            let xx = format!("{}-{}", String::from_str("xx").unwrap(), count);
            map.insert(xx, count);
        }

        let mut word_count_vec: Vec<(&String, &usize)> = Vec::new();

        for (k, v) in map.iter() {
            word_count_vec.push((&k, &v));
        }

        word_count_vec.sort_by(|a, b| b.1.cmp(a.1));

        print_histogram(&word_count_vec, 10)
    }

    #[test]
    fn test_count_words() {
        let content = "fd fd asf! ds \ndfdsfd ds";
        let map = count_words(&content);

        assert_eq!(2 as usize, *map.get("fd").unwrap());
        assert_eq!(1 as usize, *map.get("asf").unwrap());
        assert_eq!(2 as usize, *map.get("ds").unwrap());
    }
}

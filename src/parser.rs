use std::time::Instant;

pub fn parse_words(contents: &str) -> String {
    let lower_case_contents = contents.to_ascii_lowercase();
    let mut chars = lower_case_contents.chars();
    let mut temp_word = String::new();
    let mut word_count = 0;
    let mut words = String::new();

    let now = Instant::now();
    while let Some(mut c) = chars.next() {
        if c == '<' {
            let mut tag = String::new();
            while c != '>' {
                tag.push(c);
                c = chars.next().unwrap();
            }
            if tag == "<doc" {
                // println!("");
                words.push('\n');
            }
        } else {
            if c == ' ' || c == '\n' || c == '-' {
                if !temp_word.is_empty() {
                    // println!("{}", &temp_word);
                    words.push('\n');
                    words.push_str(&temp_word);
                    word_count += 1;
                }
                temp_word.clear();
            } else if c.is_ascii_alphanumeric() {
                temp_word.push(c);
            }
        }
    }
    let elapsed = now.elapsed();
    println!("Parse XML elapsed: {:.5?}", elapsed.as_secs_f64());
    println!("Word count: {}", word_count);
    words
}

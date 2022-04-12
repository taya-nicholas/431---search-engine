use std::{fs::File, io::Write, str::Chars};

pub struct Parser {
    parsed_contents: String,
    doc_offsets: Vec<u32>,
    doc_lengths: Vec<u32>,
    doc_word_count: u32,
}

pub fn new() -> Parser {
    Parser {
        parsed_contents: String::new(),
        doc_offsets: vec![],
        doc_lengths: vec![],
        doc_word_count: 0,
    }
}

impl Parser {
    pub fn get_parsed_contents(&self) -> &String {
        &self.parsed_contents
    }

    fn update_doc_word_count(&mut self) {
        self.parsed_contents.push('\n');
        if self.doc_word_count > 0 {
            self.doc_lengths.push(self.doc_word_count);
            self.doc_word_count = 0;
        }
    }

    pub fn parse(&mut self, contents: &str) {
        let lower_case_contents = contents.to_ascii_lowercase();
        let mut chars: Chars = lower_case_contents.chars();

        let mut char_count: u32 = 0;
        let mut temp_word = String::new();

        while let Some(mut c) = chars.next() {
            if c == '<' {
                let mut tag = String::new();
                while c != '>' {
                    tag.push(c);
                    c = chars.next().unwrap();
                    char_count += 1;
                }
                match tag.as_str() {
                    "<doc" => self.update_doc_word_count(),
                    "<docno" => self.doc_offsets.push(char_count + 1),
                    _ => (),
                }
            } else {
                if c == ' ' || c == '\n' {
                    if !temp_word.is_empty() {
                        self.parsed_contents.push('\n');
                        self.parsed_contents.push_str(&temp_word);
                        self.doc_word_count += 1;
                    }
                    temp_word.clear();
                } else if c.is_ascii_alphanumeric() {
                    temp_word.push(c);
                }
            }
            char_count += 1;
        }
        // Push final document length
        self.doc_lengths.push(self.doc_word_count);
    }

    pub fn create_doc_offset_file(&self) {
        let mut file = File::create("./index/doc_offsets.bin").unwrap();

        for offset in &self.doc_offsets {
            let bytes = offset.to_be_bytes();
            let _bytes_written = file.write(&bytes).unwrap();
        }
    }

    pub fn create_doc_length_file(&self) {
        let num_docs = self.doc_lengths.len() as u32;
        let mut file = File::create("./index/doc_lengths.bin").unwrap();

        //Write num docs as first bytes
        let bytes = num_docs.to_be_bytes();
        let _bytes_written = file.write(&bytes).unwrap();

        for length in &self.doc_lengths {
            let bytes = length.to_be_bytes();
            let _bytes_written = file.write(&bytes).unwrap();
        }
    }
}

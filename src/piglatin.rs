pub mod utils {
    pub fn translate(words: &Vec<&str>) -> String {
        let mut sentence = String::new();
        for word in words {
            let new_word = turn_word_to_piglatin(word);
            sentence.push_str(&new_word)
        }
        return sentence;
    }

    fn turn_word_to_piglatin(w: &str) -> String {
        let mut word = String::new();
        let mut curr_idx = 0;
        for c in w.chars() {
            if is_consonant(&c) {
                let consonant_slice = &w[..curr_idx + 1];
                let mut front = consonant_slice.to_string();
                front.push_str("ay");
                let rest = &w[curr_idx + 1..];
                word = format!("{rest}{front} ");
                break;
            }
            curr_idx += 1;
        }
        return word;
    }

    fn is_consonant(c: &char) -> bool {
        if c.is_numeric() {
            return false;
        }

        let lowercase = c.to_ascii_lowercase();
        match lowercase {
            'a' | 'e' | 'i' | 'o' | 'u' => false,
            _ => true,
        }
    }
}

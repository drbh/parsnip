use bnf::Grammar;
use std::fmt::Display;

#[derive(Debug)]
pub struct StatefulString {
    grammar: Grammar,
    pub current_text: String,
}

impl StatefulString {
    pub fn new(grammar: Grammar) -> Self {
        StatefulString {
            grammar,
            current_text: String::new(),
        }
    }

    pub fn append_token(&mut self, token: char) {
        self.current_text.push(token);
    }

    pub fn get_next_token_options<T: Display + Clone>(&self, token_candidates: &[T]) -> Vec<T> {
        token_candidates
            .iter()
            .filter_map(|token| {
                let new_str = format!("{}{}", self.current_text, token);
                let (_, is_valid) = self.grammar.try_parse_input(&new_str);
                if is_valid {
                    Some(token)
                } else {
                    None
                }
            })
            .cloned()
            .collect()
    }

    pub fn binary_mask_next_token_options<T: Display + Clone>(
        &self,
        token_candidates: &[T],
    ) -> Vec<bool> {
        let mut options = Vec::new();
        for _ in 0..token_candidates.len() {
            options.push(false);
        }
        for token in token_candidates {
            let new_str = format!("{}{}", self.current_text, token);
            let (_, is_valid) = self.grammar.try_parse_input(&new_str);
            if is_valid {
                options.push(true);
            }
        }
        options
    }

    // check if character is valid
    pub fn is_valid<T: Display + Clone>(&self, token: T) -> bool {
        let new_str = format!("{}{}", self.current_text, token);
        let (_, is_valid) = self.grammar.try_parse_input(&new_str);
        is_valid
    }

    pub fn is_complete_and_valid(&self) -> (bool, bool) {
        self.grammar.try_parse_input(&self.current_text)
    }
}

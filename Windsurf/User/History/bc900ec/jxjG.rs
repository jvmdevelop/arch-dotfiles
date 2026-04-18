use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Tokenizer {
    vocab: HashMap<String, usize>,
    reverse_vocab: HashMap<usize, String>,
    vocab_size: usize,
}

impl Tokenizer {
    pub fn new() -> Self {
        let mut tokenizer = Self {
            vocab: HashMap::new(),
            reverse_vocab: HashMap::new(),
            vocab_size: 0,
        };
        
        tokenizer.initialize_vocab();
        tokenizer
    }
    
    fn initialize_vocab(&mut self) {
        let basic_vocab = vec![
            "create", "make", "build", "generate", "draw", "form",
            "cube", "square", "box", "rectangle", "line", "point", "vertex",
            "size", "dimension", "length", "width", "height", "scale",
            "small", "large", "big", "tiny", "huge", "medium",
            "simple", "basic", "complex", "detailed", "intricate",
            "at", "in", "on", "with", "and", "or", "to", "from",
            "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10",
            "<pad>", "<unk>", "<start>", "<end>", ".", ",", ":", "(", ")",
        ];
        
        for (i, token) in basic_vocab.iter().enumerate() {
            self.vocab.insert(token.to_string(), i);
            self.reverse_vocab.insert(i, token.to_string());
        }
        
        self.vocab_size = basic_vocab.len();
    }
    
    pub fn tokenize(&self, text: &str) -> Vec<usize> {
        text.to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace() || ".,:()".contains(*c))
            .collect::<String>()
            .split_whitespace()
            .map(|word| {
                self.vocab.get(word)
                    .copied()
                    .unwrap_or_else(|| *self.vocab.get("<unk>").unwrap_or(&1))
            })
            .collect()
    }
    
    pub fn detokenize(&self, tokens: &[usize]) -> String {
        tokens.iter()
            .filter_map(|&token| self.reverse_vocab.get(&token))
            .cloned()
            .collect::<Vec<String>>()
            .join(" ")
    }
    
    pub fn encode_text(&self, text: &str, max_length: usize) -> Vec<f64> {
        let tokens = self.tokenize(text);
        let mut encoded = vec![0.0; max_length];
        
        for (i, &token) in tokens.iter().enumerate() {
            if i < max_length {
                encoded[i] = token as f64 / self.vocab_size as f64;
            }
        }
        
        encoded
    }
    
    pub fn decode_to_vdsl(&self, tokens: &[usize]) -> String {
        let mut vdsl_output = String::new();
        let mut point_counter = 0;
        
        for &token in tokens {
            if let Some(word) = self.reverse_vocab.get(&token) {
                match word.as_str() {
                    "cube" | "square" | "box" => {
                        vdsl_output.push_str(&format!("{}: 0 0 0\n", point_counter));
                        point_counter += 1;
                        vdsl_output.push_str(&format!("{}: 10 0 0\n", point_counter));
                        point_counter += 1;
                        vdsl_output.push_str(&format!("{}: 10 10 0\n", point_counter));
                        point_counter += 1;
                        vdsl_output.push_str(&format!("{}: 0 10 0\n", point_counter));
                        point_counter += 1;
                        vdsl_output.push_str(&format!("{}: 0 0 10\n", point_counter));
                        point_counter += 1;
                        vdsl_output.push_str(&format!("{}: 10 0 10\n", point_counter));
                        point_counter += 1;
                        vdsl_output.push_str(&format!("{}: 10 10 10\n", point_counter));
                        point_counter += 1;
                        vdsl_output.push_str(&format!("{}: 0 10 10\n", point_counter));
                        point_counter += 1;
                        
                        vdsl_output.push_str("l: 0 1\nl: 1 2\nl: 2 3\nl: 3 0\n");
                        vdsl_output.push_str("l: 4 5\nl: 5 6\nl: 6 7\nl: 7 4\n");
                        vdsl_output.push_str("l: 0 4\nl: 1 5\nl: 2 6\nl: 3 7\n");
                    }
                    "line" => {
                        if point_counter >= 2 {
                            vdsl_output.push_str(&format!("l: {} {}\n", point_counter - 2, point_counter - 1));
                        }
                    }
                    "point" => {
                        vdsl_output.push_str(&format!("{}: 0 0 0\n", point_counter));
                        point_counter += 1;
                    }
                    _ => {}
                }
            }
        }
        
        vdsl_output
    }
    
    pub fn vocab_size(&self) -> usize {
        self.vocab_size
    }
    
    pub fn add_to_vocab(&mut self, word: &str) -> usize {
        if let Some(&id) = self.vocab.get(word) {
            return id;
        }
        
        let id = self.vocab_size;
        self.vocab.insert(word.to_string(), id);
        self.reverse_vocab.insert(id, word.to_string());
        self.vocab_size += 1;
        id
    }
}

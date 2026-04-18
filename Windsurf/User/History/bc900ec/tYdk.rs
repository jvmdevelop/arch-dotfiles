use std::collections::HashMap;
use anyhow::Result;

pub struct Tokenizer {
    vocabulary: HashMap<String, usize>,
    reverse_vocabulary: HashMap<usize, String>,
    vocab_size: usize,
}

impl Tokenizer {
    pub fn new() -> Self {
        Self {
            vocabulary: HashMap::new(),
            reverse_vocabulary: HashMap::new(),
            vocab_size: 0,
        }
    }
    
    pub fn build_vocabulary(&mut self, texts: &[String]) -> Result<()> {
        let mut word_counts = HashMap::new();
        
        for text in texts {
            for word in text.to_lowercase().split_whitespace() {
                *word_counts.entry(word.to_string()).or_insert(0) += 1;
            }
        }
        
        let mut sorted_words: Vec<_> = word_counts.into_iter().collect();
        sorted_words.sort_by(|a, b| b.1.cmp(&a.1));
        
        self.vocabulary.insert("<pad>".to_string(), 0);
        self.vocabulary.insert("<unk>".to_string(), 1);
        self.vocabulary.insert("<start>".to_string(), 2);
        self.vocabulary.insert("<end>".to_string(), 3);
        
        self.reverse_vocabulary.insert(0, "<pad>".to_string());
        self.reverse_vocabulary.insert(1, "<unk>".to_string());
        self.reverse_vocabulary.insert(2, "<start>".to_string());
        self.reverse_vocabulary.insert(3, "<end>".to_string());
        
        self.vocab_size = 4;
        
        for (word, _) in sorted_words.into_iter().take(1000) {
            if !self.vocabulary.contains_key(&word) {
                self.vocabulary.insert(word.clone(), self.vocab_size);
                self.reverse_vocabulary.insert(self.vocab_size, word);
                self.vocab_size += 1;
            }
        }
        
        Ok(())
    }
    
    pub fn tokenize(&self, text: &str) -> Vec<usize> {
        let mut tokens = vec![2]; 
        
        for word in text.to_lowercase().split_whitespace() {
            let token_id = self.vocabulary.get(word).copied().unwrap_or(1); 
            tokens.push(token_id);
        }
        
        tokens.push(3); 
        tokens
    }
    
    pub fn detokenize(&self, tokens: &[usize]) -> String {
        tokens.iter()
            .filter_map(|&token| self.reverse_vocabulary.get(&token))
            .cloned()
            .collect::<Vec<_>>()
            .join(" ")
    }
    
    pub fn encode_vdsl(&self, vdsl: &str) -> Vec<f64> {
        let lines: Vec<&str> = vdsl.lines().collect();
        let mut features = Vec::new();
        
        
        let mut point_count = 0;
        let mut line_count = 0;
        let mut comment_count = 0;
        
        for line in &lines {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                comment_count += 1;
            } else if line.starts_with('l') {
                line_count += 1;
            } else if line.chars().next().map_or(false, |c| c.is_ascii_digit()) {
                point_count += 1;
            }
        }
        
        features.push(point_count as f64);
        features.push(line_count as f64);
        features.push(comment_count as f64);
        features.push(lines.len() as f64);
        
        // Extract coordinates from points
        let mut coords = Vec::new();
        for line in &lines {
            if line.trim().chars().next().map_or(false, |c| c.is_ascii_digit()) {
                if let Some(coords_part) = line.split(':').nth(1) {
                    for coord in coords_part.trim().split_whitespace() {
                        if let Ok(val) = coord.parse::<f64>() {
                            coords.push(val);
                        }
                    }
                }
            }
        }
        
        // Add coordinate statistics
        if !coords.is_empty() {
            let min = coords.iter().fold(f64::INFINITY, |a, &b| a.min(b));
            let max = coords.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
            let avg = coords.iter().sum::<f64>() / coords.len() as f64;
            
            features.push(min);
            features.push(max);
            features.push(avg);
        } else {
            features.extend_from_slice(&[0.0, 0.0, 0.0]);
        }
        
        // Pad to fixed size
        while features.len() < 50 {
            features.push(0.0);
        }
        features.truncate(50);
        
        features
    }
    
    pub fn decode_to_vdsl(&self, features: &[f64]) -> String {
        let point_count = features[0] as usize;
        let line_count = features[1] as usize;
        
        let mut vdsl = String::new();
        
        // Generate points in a simple pattern
        for i in 0..point_count {
            let x = (i % 4) as f64 * 10.0;
            let y = ((i / 4) % 4) as f64 * 10.0;
            let z = (i / 16) as f64 * 10.0;
            vdsl.push_str(&format!("{}: {:.0} {:.0} {:.0}\n", i, x, y, z));
        }
        
        // Generate lines to connect points
        for i in 0..line_count.min(point_count - 1) {
            vdsl.push_str(&format!("l: {}:{}\n", i, i + 1));
        }
        
        vdsl
    }
    
    pub fn vocab_size(&self) -> usize {
        self.vocab_size
    }
    
    pub fn get_vocabulary(&self) -> &HashMap<String, usize> {
        &self.vocabulary
    }
}

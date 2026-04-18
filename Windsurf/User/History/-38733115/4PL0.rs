use crate::lexer::Lexer;
use crate::model::{TokenType, TokenTrait};

pub struct StaticLexer;

impl Lexer for StaticLexer {
    fn lex(&self, text: &str) -> Result<Vec<Box<dyn TokenTrait>>, String> {
        let mut tokens = Vec::new();
        let text = text.trim();
        
        if text.is_empty() {
            return Err("file is empty".to_string());
        }
        
        for line in text.lines() {
            let clean_line = line.split('#').next().unwrap_or(line).trim();
            if clean_line.is_empty() {
                continue;
            }
            
            let token_type = TokenType::from_string(clean_line)
                .ok_or_else(|| format!("Unknown token type for line: {}", clean_line))?;
            
            let token = token_type.factory_from_string(clean_line)?;
            tokens.push(token);
        }
        
        Ok(tokens)
    }
}

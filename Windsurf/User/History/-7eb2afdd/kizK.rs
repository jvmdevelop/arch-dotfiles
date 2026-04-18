use crate::model::{Token, PointToken, LineToken, CommentToken};

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    Point,
    Line,
    Comment,
}

impl TokenType {
    pub fn from_string(string: &str) -> Option<Self> {
        let string = string.trim();
        if string.is_empty() {
            return None;
        }
        
        match string.chars().next()? {
            'l' => Some(Self::Line),
            'c' => Some(Self::Comment),
            '<' => Some(Self::Comment),
            c if c.is_ascii_digit() => Some(Self::Point),
            _ => None,
        }
    }

    pub fn factory_from_string(&self, string: &str) -> Result<Box<dyn TokenTrait>, String> {
        let string = string.trim();
        
        match self {
            Self::Point => {
                let parts: Vec<&str> = string.split(':').collect();
                if parts.len() != 2 {
                    return Err("Invalid point format".to_string());
                }
                
                let index = parts[0].trim()
                    .parse::<i32>()
                    .map_err(|_| "Invalid point index".to_string())?;
                
                let coords: Vec<i32> = parts[1].trim()
                    .split_whitespace()
                    .map(|s| s.parse().map_err(|_| "Invalid coordinate".to_string()))
                    .collect::<Result<_, _>>()?;
                
                if coords.len() != 3 {
                    return Err("Point must have 3 coordinates".to_string());
                }
                
                Ok(Box::new(PointToken::new(index, [coords[0], coords[1], coords[2]])))
            }
            Self::Line => {
                let parts: Vec<&str> = string.split(':').collect();
                if parts.len() != 3 {
                    return Err("Invalid line format".to_string());
                }
                
                let start = parts[1].trim()
                    .parse::<i32>()
                    .map_err(|_| "Invalid line start point".to_string())?;
                let end = parts[2].trim()
                    .parse::<i32>()
                    .map_err(|_| "Invalid line end point".to_string())?;
                
                Ok(Box::new(LineToken::new([start, end])))
            }
            Self::Comment => Ok(Box::new(CommentToken::new())),
        }
    }
}

pub trait TokenTrait {
    fn as_any(&self) -> &dyn std::any::Any;
}

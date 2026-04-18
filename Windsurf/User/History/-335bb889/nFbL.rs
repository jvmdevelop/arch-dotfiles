use crate::lexer::{StaticLexer, Lexer};
use crate::exporter::ObjExporter;
use crate::model::TokenTrait;
use std::fs;
use std::io;

pub struct VdslParser {
    lexer: StaticLexer,
    exporter: ObjExporter,
}

impl VdslParser {
    pub fn new() -> Self {
        Self {
            lexer: StaticLexer,
            exporter: ObjExporter::new(),
        }
    }
    
    pub fn parse_and_export(&mut self, vdsl_file_path: &str, obj_file_path: &str) -> io::Result<()> {
        let vdsl_content = fs::read_to_string(vdsl_file_path)?;
        
        let tokens = self.lexer.lex(&vdsl_content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        
        for token in tokens {
            self.exporter.add_token(token);
        }
        
        self.exporter.export_to_file(obj_file_path)?;
        
        println!("Successfully converted {} to {}", vdsl_file_path, obj_file_path);
        println!("Processed {} tokens", tokens.len());
        
        Ok(())
    }
}

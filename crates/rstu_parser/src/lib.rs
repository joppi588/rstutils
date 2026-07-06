// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

pub mod lexer;

pub use document_tree::Document;
pub use anyhow::{anyhow, Result};

/// Parse RST content with stricter validation.
/// 
/// This wrapper adds validation for proper RST directive syntax:
/// - Directives must use single colon (e.g., `.. note:`)
/// - Double colons (e.g., `.. note::`) are not allowed
pub fn parse(content: &str) -> Result<Document> {
    // Validate directive syntax - no double colons
    if content.contains("..") {
        let lines: Vec<&str> = content.lines().collect();
        for (idx, line) in lines.iter().enumerate() {
            if line.trim().starts_with("..") && line.contains("::") {
                return Err(anyhow!(
                    "Invalid directive syntax at line {}: directives must use single colon ':', not '::'. Found: {}",
                    idx + 1,
                    line.trim()
                ));
            }
        }
    }
    
    // If validation passes, use the external parser
    rst_parser::parse(content)
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
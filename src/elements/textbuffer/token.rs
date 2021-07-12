use sfml::graphics::{ Color, TextStyle };
use seamonkey::TokenType;

use elements::{ TextbufferTheme, TextTheme };

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub index: usize,
    pub length: usize,
}

impl Token {

    pub fn new(token_type: TokenType, index: usize, length: usize) -> Self {
        Self {
            token_type: token_type,
            length: length,
            index: index,
        }
    }

    pub fn display_name(&self) -> Option<&'static str> {
        match self.token_type {
            TokenType::Comment(..) => return Some("comment"),
            TokenType::Operator(..) => return Some("operator"),
            TokenType::Keyword(..) => return Some("keyword"),
            TokenType::Identifier(..) => return Some("identifier"),
            TokenType::TypeIdentifier(..) => return Some("type identifier"),
            TokenType::Character(..) => return Some("character"),
            TokenType::String(..) => return Some("string"),
            TokenType::Integer(..) => return Some("integer"),
            TokenType::Float(..) => return Some("float"),
            TokenType::Invalid(..) => return None,
            TokenType::Ignored => return None,
        }
    }

    pub fn get_theme<'t>(&self, theme: &'t TextbufferTheme) -> &'t TextTheme {
        match self.token_type {
            TokenType::Comment(..) => return &theme.comment_theme,
            TokenType::Operator(..) => return &theme.operator_theme,
            TokenType::Keyword(..) => return &theme.keyword_theme,
            TokenType::Identifier(..) => return &theme.identifier_theme,
            TokenType::TypeIdentifier(..) => return &theme.type_identifier_theme,
            TokenType::Character(..) => return &theme.character_theme,
            TokenType::String(..) => return &theme.string_theme,
            TokenType::Integer(..) => return &theme.integer_theme,
            TokenType::Float(..) => return &theme.float_theme,
            TokenType::Invalid(..) => return &theme.invalid_theme,
            TokenType::Ignored => return &theme.text_theme,
        }
    }
}

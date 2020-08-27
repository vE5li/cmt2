use sfml::graphics::{ Color, TextStyle };
use context::Context;
use seamonkey::TokenType;

#[derive(Clone, Debug)]
pub struct EditorToken {
    pub token_type: TokenType,
    pub index: usize,
    pub length: usize,
}

impl EditorToken {

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

    pub fn get_color(&self, context: &Context) -> Color {
        match self.token_type {
            TokenType::Comment(..) => return context.theme.panel.comment,
            TokenType::Operator(..) => return context.theme.panel.operator,
            TokenType::Keyword(..) => return context.theme.panel.keyword,
            TokenType::Identifier(..) => return context.theme.panel.identifier,
            TokenType::TypeIdentifier(..) => return context.theme.panel.type_identifier,
            TokenType::Character(..) => return context.theme.panel.character,
            TokenType::String(..) => return context.theme.panel.string,
            TokenType::Integer(..) => return context.theme.panel.integer,
            TokenType::Float(..) => return context.theme.panel.float,
            TokenType::Invalid(..) => return context.theme.panel.error,
            TokenType::Ignored => return context.theme.panel.text,
        }
    }

    pub fn get_style(&self, context: &Context) -> TextStyle {
        match self.token_type {
            TokenType::Comment(..) => return context.theme.panel.comment_style,
            TokenType::Operator(..) => return context.theme.panel.operator_style,
            TokenType::Keyword(..) => return context.theme.panel.keyword_style,
            TokenType::Identifier(..) => return context.theme.panel.identifier_style,
            TokenType::TypeIdentifier(..) => return context.theme.panel.type_identifier_style,
            TokenType::Character(..) => return context.theme.panel.character_style,
            TokenType::String(..) => return context.theme.panel.string_style,
            TokenType::Integer(..) => return context.theme.panel.integer_style,
            TokenType::Float(..) => return context.theme.panel.float_style,
            TokenType::Invalid(..) => return context.theme.panel.error_style,
            TokenType::Ignored => return context.theme.panel.text_style,
        }
    }
}

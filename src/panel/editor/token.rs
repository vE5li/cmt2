use sfml::graphics::Color;
use context::Context;
use kami::TokenType;

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
}

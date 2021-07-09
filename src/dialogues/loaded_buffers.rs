use seamonkey::*;

use sfml::graphics::*;
use sfml::system::Vector2f;

use elements::ComboBox;
use dialogues::DialogueTheme;
use interface::InterfaceContext;
use system::ResourceManager;
use input::Action;

pub struct LoadedBuffersDialogue {
    pub buffers_box: ComboBox,
}

impl LoadedBuffersDialogue {

    pub fn new() -> Self {
        Self {
            buffers_box: ComboBox::new("recently opened files", 0, false, false, Vec::new()),
        }
    }

    pub fn handle_action(&mut self, interface_context: &InterfaceContext, action: Action) -> (bool, Option<bool>) {

        if let Action::LoadedBuffers = action {
            return (true, Some(false));
        }

        return self.buffers_box.handle_action(interface_context, action);
    }

    pub fn update_variants(&mut self, resource_manager: &ResourceManager) {
        let variants = resource_manager.filebuffers.iter().map(|(name, _)| SharedString::from(&name)).collect();
        self.buffers_box.variants = variants;
    }

    pub fn clear(&mut self) {
        self.buffers_box.clear();
    }

    pub fn add_character(&mut self, character: Character) {
        self.buffers_box.add_character(character);
    }

    pub fn update_layout(&mut self, interface_context: &InterfaceContext, theme: &DialogueTheme, size: Vector2f, position: Vector2f) {
        self.buffers_box.update_layout(interface_context, theme, size, position);
    }

    pub fn render(&self, framebuffer: &mut RenderTexture, interface_context: &InterfaceContext, theme: &DialogueTheme) {
        self.buffers_box.render(framebuffer, interface_context, theme, true);
    }
}

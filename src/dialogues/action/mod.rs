mod item;

use seamonkey::*;

use sfml::graphics::*;
use sfml::system::Vector2f;

use input::Action;
use dialogues::{ DialogueMode, DialogueStatus };
use themes::{ DialogueTheme, ItemTheme, TextfieldTheme };
use elements::{ ComboBox, ComboItem };
use managers::LanguageManager;
use interface::InterfaceContext;

use self::item::ActionItem;

pub struct ActionDialogue {
    combobox: ComboBox<ActionItem>,
}

impl ActionDialogue {

    pub fn new(language_manager: &mut LanguageManager) -> Self {

        let items = vec![
            // no "comfirm", "action", "abort", "hidden_files"
            ActionItem::new(Action::AddSelection, "add selection"),
            ActionItem::new(Action::Append, "append"),
            ActionItem::new(Action::CharacterMode, "character mode"),
            ActionItem::new(Action::CloseWindow, "close window"),
            ActionItem::new(Action::Copy, "copy"),
            ActionItem::new(Action::Cut, "cut"),
            ActionItem::new(Action::DecreaseAntialiasing, "decrease antialiasing"),
            ActionItem::new(Action::Delete, "delete"),
            ActionItem::new(Action::DeleteLine, "delete line"),
            ActionItem::new(Action::Down, "down"),
            ActionItem::new(Action::DuplicateDown, "duplicate down"),
            ActionItem::new(Action::DuplicateUp, "duplicate up"),
            ActionItem::new(Action::End, "end"),
            ActionItem::new(Action::ExtendDown, "extend down"),
            ActionItem::new(Action::ExtendEnd, "extend end"),
            ActionItem::new(Action::ExtendLeft, "extend left"),
            ActionItem::new(Action::ExtendPageDown, "extend page down"),
            ActionItem::new(Action::ExtendPageUp, "extend page up"),
            ActionItem::new(Action::ExtendRight, "extend right"),
            ActionItem::new(Action::ExtendStart, "extend start"),
            ActionItem::new(Action::ExtendUp, "extend up"),
            ActionItem::new(Action::Replace, "find replace"),
            ActionItem::new(Action::FocusNext, "focus next"),
            ActionItem::new(Action::IncreaseAntialiasing, "increase antialiasing"),
            ActionItem::new(Action::Insert, "insert"),
            ActionItem::new(Action::Left, "left"),
            ActionItem::new(Action::LineMode, "line mode"),
            ActionItem::new(Action::Filebuffers, "loaded buffers"),
            ActionItem::new(Action::MoveDown, "move down"),
            ActionItem::new(Action::MoveLeft, "move left"),
            ActionItem::new(Action::MoveRight, "move right"),
            ActionItem::new(Action::MoveUp, "move up"),
            ActionItem::new(Action::NewFile, "new file"),
            ActionItem::new(Action::NewlineDown, "newline down"),
            ActionItem::new(Action::NewlineUp, "newline up"),
            ActionItem::new(Action::NewWindow, "new window"),
            ActionItem::new(Action::Notes, "notes"),
            ActionItem::new(Action::Open, "open file"),
            ActionItem::new(Action::PageDown, "page down"),
            ActionItem::new(Action::PageUp, "page up"),
            ActionItem::new(Action::Paste, "paste"),
            ActionItem::new(Action::Quit, "quit"),
            ActionItem::new(Action::Reload, "reload"),
            ActionItem::new(Action::Remove, "remove"),
            ActionItem::new(Action::RemoveSection, "remove section"),
            ActionItem::new(Action::Right, "right"),
            ActionItem::new(Action::Rotate, "rotate"),
            ActionItem::new(Action::SaveFile, "save file"),
            ActionItem::new(Action::SelectNext, "select next"),
            ActionItem::new(Action::Language, "set language"),
            ActionItem::new(Action::Theme, "set theme"),
            ActionItem::new(Action::Start, "start"),
            ActionItem::new(Action::ToggleAppendLines, "toggle append lines"),
            ActionItem::new(Action::ToggleStatusBar, "toggle status bar"),
            ActionItem::new(Action::ToggleLineNumbers, "toggle line numbers"),
            ActionItem::new(Action::ToggleSelectionLines, "toggle selection lines"),
            ActionItem::new(Action::ToggleHighlighting, "toggle highlighting"),
            ActionItem::new(Action::TogglePreserveLines, "toggle preserve lines"),
            ActionItem::new(Action::ToggleStartAtSymbol, "toggle start at symbol"),
            ActionItem::new(Action::ToggleRelativeLineNumbers, "toggle relative line numbers"),
            ActionItem::new(Action::ToggleUnfocusedSelections, "toggle unfocused selections"),
            ActionItem::new(Action::Up, "up"),
            ActionItem::new(Action::WordMode, "word mode"),
            ActionItem::new(Action::ZoomIn, "zoom in"),
            ActionItem::new(Action::ZoomOut, "zoom out"),
        ];

        Self {
            combobox: ComboBox::new(language_manager, "action", 0, false, items),
        }
    }

    pub fn open(&self) -> DialogueMode {
        return DialogueMode::Action;
    }

    pub fn get_value(&self) -> Action {
        return self.combobox.get_value();
    }

    pub fn handle_action(&mut self, interface_context: &InterfaceContext, language_manager: &mut LanguageManager, action: Action) -> DialogueStatus {
        match action {
            Action::Action => return DialogueStatus::handled(),
            action => return self.combobox.handle_action(interface_context, language_manager, action),
        }
    }

    pub fn add_character(&mut self, language_manager: &mut LanguageManager, character: Character) {
        self.combobox.add_character(language_manager, character);
    }

    pub fn update_layout(&mut self, interface_context: &InterfaceContext, theme: &DialogueTheme, size: Vector2f, position: Vector2f) {
        self.combobox.update_layout(interface_context, theme, size, position);
    }

    pub fn render(&self, framebuffer: &mut RenderTexture, interface_context: &InterfaceContext, theme: &DialogueTheme) {
        self.combobox.render(framebuffer, interface_context, theme, true);
    }
}

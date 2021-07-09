

pub struct TextbufferContext {
    pub line_numbers: bool,
    pub tab_width: usize,
    pub scroll_size: usize,
    pub append_lines: bool,
    pub status_bar: bool,
    pub highlighting: bool,
    pub selection_lines: bool,
    pub preserve_lines: bool,
    pub unfocused_selections: bool,
    pub start_at_symbol: bool,
    pub multiline: bool,
}

impl TextbufferContext {

    pub fn from() -> Self {
        return Self {
            line_numbers: true,
            tab_width: 4,
            scroll_size: 8,
            append_lines: false, // ?
            status_bar: false,
            highlighting: true,
            selection_lines: true,
            preserve_lines: true,
            unfocused_selections: true,
            start_at_symbol: true,
            multiline: true,
        }
    }

    pub fn textbox() -> Self {
        return Self {
            line_numbers: false,
            tab_width: 4,
            scroll_size: 0,
            append_lines: false, // ?
            status_bar: false,
            highlighting: true,
            selection_lines: false,
            preserve_lines: true,
            unfocused_selections: false,
            start_at_symbol: false,
            multiline: false,
        }
    }

    // increase_tab_width

    // decrease_tab_width

    // increase_scroll_size

    // decrease_scroll_size

    // increase_line_spacing

    // decrease_line_spacing

    // increase_character_spacing

    // decrease_character_spacing

    // increase_selection_gap

    // decrease_selection_gap

    pub fn toggle_line_numbers(&mut self) {
        self.line_numbers = !self.line_numbers;
    }

    pub fn toggle_append_lines(&mut self) {
        self.append_lines = !self.append_lines;
    }

    pub fn toggle_status_bar(&mut self) {
        self.status_bar = !self.status_bar;
    }

    pub fn toggle_selection_lines(&mut self) {
        self.selection_lines = !self.selection_lines;
    }

    pub fn toggle_highlighting(&mut self) {
        self.highlighting = !self.highlighting;
    }

    pub fn toggle_preserve_lines(&mut self) {
        self.preserve_lines = !self.preserve_lines;
    }

    pub fn toggle_unfocused_selections(&mut self) {
        self.unfocused_selections = !self.unfocused_selections;
    }

    pub fn toggle_start_at_symbol(&mut self) {
        self.start_at_symbol = !self.start_at_symbol;
    }
}

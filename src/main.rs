extern crate seamonkey;
extern crate sfml;

mod graphics;
mod input;
mod context;
mod panel;

use seamonkey::*;
use sfml::{ graphics::*, system::*, window::* };

use self::context::{ Context, Action };
use self::input::{ KeyEvent, Modifiers, is_modifier_key };
use self::panel::Panel;

const SMALLEST_FONT_SIZE: usize = 8;
const BIGGEST_FONT_SIZE: usize = 34;
const MAXIMUM_PANEL_COUNT: usize = 8;

pub struct Instance<'i> {
    window: RenderWindow,
    panels: Vec<Panel<'i>>,
    focused_panel: usize,
    copy_buffer: String,
    context: Context,
    size: Vector2u,
}

impl<'i> Instance<'i> {

    pub fn new(arguments: &Vec<String>) -> Self {
        let context = display!(Context::new(SharedString::from("/home/.poet/context.data"), &SharedString::from("/home/.poet/configuration/")));

        let mut panels = Vec::new();
        let mut panel1 = display!(Panel::new_editor(context.font_size));

        let character_scaling = context.character_spacing * context.font_size as f32;
        let panel_gap = context.theme.panel.gap * character_scaling;

        let video_mode = VideoMode::desktop_mode();
        let window_size = Vector2u::new(video_mode.width - video_mode.width / 4, video_mode.height - video_mode.height / 4);
        let size = Vector2f::new(window_size.x as f32 - panel_gap * 2.0, window_size.y as f32 - panel_gap * 2.0);
        let position = Vector2f::new(panel_gap, panel_gap);

        panel1.update_graphics(&context, true, size);
        panel1.update_position(position);
        panels.push(panel1);

        let mut window = RenderWindow::new((window_size.x, window_size.y), "poet", Style::DEFAULT, &Default::default());
        //window.set_mouse_cursor_visible(false);
        window.set_vertical_sync_enabled(true);

        Self {
            window: window,
            panels: panels,
            focused_panel: 0,
            copy_buffer: String::new(),
            context: context,
            size: window_size,
        }
    }

    pub fn execute(&mut self) {
        let mut add_character = true;
        'execute: loop {
            self.draw();

            while let Some(event) = self.window.poll_event() {
                match event {

                    Event::Closed => return,

                    //Event::MouseWheelScrolled { wheel, delta, x, y } => {
                    //    push_text!(x, y, "Scroll: {:?}, {}, {}, {}", wheel, delta, x, y);
                    //}

                    //Event::MouseButtonPressed { button, x, y } => {
                    //    push_text!(x, y, "Press: {:?}, {}, {}", button, x, y);
                    //}

                    //Event::MouseButtonReleased { button, x, y } => {
                    //    push_text!(x, y, "Release: {:?}, {}, {}", button, x, y);
                    //}

                    Event::KeyPressed { code, shift, ctrl, alt, system } => {
                        if !is_modifier_key(&code) {
                            let modifiers = Modifiers::from(shift, ctrl, alt, system);
                            let key_event = KeyEvent::new(code, modifiers);

                            for action in self.context.get_matching_actions(&key_event) {

                                if display!(self.panels[self.focused_panel].handle_action(&self.context, action)) {
                                    add_character = false;
                                    continue 'execute;
                                }

                                match action {

                                    Action::Quit => return,

                                    Action::ToggleAppendLines => {
                                        self.context.toggle_append_lines();
                                        add_character = false;
                                        continue 'execute;
                                    },

                                    Action::ToggleStatusBar => {
                                        self.context.toggle_status_bar();
                                        self.update_panels();
                                        add_character = false;
                                        continue 'execute;
                                    },

                                    Action::ToggleLineNumbers => {
                                        self.context.toggle_line_numbers();
                                        self.update_panels();
                                        add_character = false;
                                        continue 'execute;
                                    },

                                    Action::ToggleSelectionLines => {
                                        self.context.toggle_selection_lines();
                                        self.update_panels();
                                        add_character = false;
                                        continue 'execute;
                                    },

                                    Action::ToggleHighlighting => {
                                        self.context.toggle_highlighting();
                                        self.update_panels();
                                        add_character = false;
                                        continue 'execute;
                                    },

                                    Action::TogglePreserveLines => {
                                        self.context.toggle_preserve_lines();
                                        add_character = false;
                                        continue 'execute;
                                    },

                                    Action::ToggleUnfocusedSelections => {
                                        self.context.toggle_unfocused_selections();
                                        self.update_panels();
                                        add_character = false;
                                        continue 'execute;
                                    }

                                    Action::ToggleFocusBar => {
                                        self.context.toggle_focus_bar();
                                        self.update_panels();
                                        add_character = false;
                                        continue 'execute;
                                    }

                                    Action::MoveFocusLeft => {
                                        self.move_focus_left();
                                        add_character = false;
                                        continue 'execute;
                                    },

                                    Action::MoveFocusRight => {
                                        self.move_focus_right();
                                        add_character = false;
                                        continue 'execute;
                                    },

                                    Action::ZoomIn => {
                                        self.zoom_in();
                                        add_character = false;
                                        continue 'execute;
                                    },

                                    Action::ZoomOut => {
                                        self.zoom_out();
                                        add_character = false;
                                        continue 'execute;
                                    },

                                    Action::NewEditor => {
                                        self.new_editor();
                                        add_character = false;
                                        continue 'execute;
                                    },

                                    Action::ClosePanel => {
                                        self.close_panel();
                                        add_character = false;
                                        continue 'execute;
                                    },

                                    _unhandled => { },
                                }
                            }
                        }
                    },

                    Event::TextEntered { unicode } => {
                        if add_character && unicode as u32 == 13 {
                            self.panels[self.focused_panel].add_character(&self.context, Character::from_char('\n'));
                            add_character = true;
                        } else if add_character && unicode as u32 >= 32 && unicode as u32 <= 126 {
                            self.panels[self.focused_panel].add_character(&self.context, Character::from_char(unicode));
                            add_character = true;
                        }
                    },

                    Event::Resized { width, height } => {
                        self.size = Vector2u::new(width, height);
                        self.update_graphics();
                    },

                    _ => { },
                }
            }

            add_character = true;
        }
    }

    fn update_panels(&mut self) {
        for (index, panel) in self.panels.iter_mut().enumerate() {
            panel.update(&self.context, self.focused_panel == index);
        }
    }

    fn update_graphics(&mut self) {

        let view = View::from_rect(&FloatRect::new(0.0, 0.0, self.size.x as f32, self.size.y as f32));
        self.window.set_view(&view);

        let panel_gap = self.context.theme.panel.gap * self.context.font_size as f32;
        let mut remaining_width = self.size.x as f32 - panel_gap * 2.0;
        let panel_count = self.panels.len();

        for index in (0..self.panels.len()).rev() {
            let mut panel_width = remaining_width / (index + 1) as f32;
            remaining_width -= panel_width;
            let mut panel_left = remaining_width + panel_gap;

            if index != 0 {
                panel_left += panel_gap / 2.0;
                panel_width -= panel_gap / 2.0;
            }

            if index != panel_count - 1 {
                panel_width -= panel_gap / 2.0;
            }

            let panel_size = Vector2f::new(panel_width, self.size.y as f32 - panel_gap * 2.0);
            let panel_position = Vector2f::new(panel_left, panel_gap);
            self.panels[index].update_graphics(&self.context, self.focused_panel == index, panel_size);
            self.panels[index].update_position(panel_position);
        }
    }

    fn new_editor(&mut self) {
        if self.panels.len() < MAXIMUM_PANEL_COUNT {
            let new_index = self.focused_panel + 1;
            let mut panel = display!(Panel::new_editor(self.context.font_size));

            match new_index == self.panels.len() {
                true => self.panels.push(panel),
                false => self.panels.insert(new_index, panel),
            }

            self.focused_panel = new_index;
            self.update_graphics();
        }
    }

    fn close_panel(&mut self) {
        if self.panels.len() > 1 {
            self.panels.remove(self.focused_panel);
            if self.focused_panel > 0 {
                self.focused_panel -= 1;
            }
            self.update_graphics();
        }
    }

    fn zoom_in(&mut self) {
        if self.context.font_size > SMALLEST_FONT_SIZE {
            self.context.font_size -= 1;
            self.update_graphics();
        }
    }

    fn zoom_out(&mut self) {
        if self.context.font_size < BIGGEST_FONT_SIZE {
            self.context.font_size += 1;
            self.update_graphics();
        }
    }

    fn move_focus_left(&mut self) {
        if self.focused_panel > 0 {
            self.focused_panel -= 1;
        }
        self.update_panels();
    }

    fn move_focus_right(&mut self) {
        if self.focused_panel < self.panels.len() - 1 {
            self.focused_panel += 1;
        }
        self.update_panels();
    }

    fn draw(&mut self) {
        self.window.clear(self.context.theme.panel.border);
        for panel in self.panels.iter_mut() {
            panel.draw(&mut self.window);
        }
        self.window.display();
    }

    pub fn close(&self) {
        self.context.safe();
    }
}

fn main() {

    use std::env;
    let arguments: Vec<String> = env::args().collect();

    let mut instance = Instance::new(&arguments);
    instance.execute();
    instance.close();
}

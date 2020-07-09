#![feature(allocator_api)]

#[macro_use]
extern crate dst6 as kami;
extern crate sfml;

mod graphics;
mod input;
mod context;
mod panel;

use kami::*;
use sfml::{ graphics::*, system::*, window::* };

use self::context::{ Context, Action };
use self::input::{ KeyEvent, Modifiers, is_modifier_key };
use self::panel::Panel;

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
        let context = display!(Context::new(VectorString::from("/home/.poet/context.data"), &VectorString::from("/home/.poet/configuration/")), &None, &map!(), &map!());

        let mut panels = Vec::new();
        let mut panel1 = display!(Panel::new_editor(context.font_size), &None, &map!(), &map!());

        let character_scaling = context.character_spacing * context.font_size as f32;
        let panel_gap = context.theme.panel.gap * character_scaling;

        let video_mode = VideoMode::desktop_mode();
        let window_size = Vector2u::new(video_mode.width - video_mode.width / 4, video_mode.height - video_mode.height / 4);
        let size = Vector2f::new(window_size.x as f32 - panel_gap * 2.0, window_size.y as f32 - panel_gap * 2.0);
        let position = Vector2f::new(panel_gap, panel_gap);

        panel1.update_graphics(&context, size);
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
                            //println!("PRESSED : {:?} : shft {:?} : ctrl {:?} : alt {:?} : system {:?}", code, shift, ctrl, alt, system);

                            let modifiers = Modifiers::from(shift, ctrl, alt, system);
                            let key_event = KeyEvent::new(code, modifiers);

                            for action in self.context.get_matching_actions(&key_event) {
                                match action {

                                    Action::Quit => return,

                                    Action::ToggleAppendLines => {
                                        self.context.toggle_append_lines();
                                        continue 'execute;
                                    },

                                    Action::ToggleStatusBar => {
                                        self.context.toggle_status_bar();
                                        continue 'execute;
                                    },

                                    Action::ToggleLineNumbers => {
                                        self.context.toggle_line_numbers();
                                        continue 'execute;
                                    },

                                    Action::ToggleSelectionLines => {
                                        self.context.toggle_selection_lines();
                                        continue 'execute;
                                    },

                                    Action::MoveFocusLeft => {
                                        self.move_focus_left();
                                        continue 'execute;
                                    },

                                    Action::MoveFocusRight => {
                                        self.move_focus_right();
                                        continue 'execute;
                                    },

                                    Action::ZoomIn => {
                                        self.zoom_in();
                                        continue 'execute;
                                    },

                                    Action::ZoomOut => {
                                        self.zoom_out();
                                        continue 'execute;
                                    },

                                    unhandled => {
                                        if display!(self.panels[self.focused_panel].handle_action(&self.context, action), &None, &map!(), &map!()) {
                                            continue 'execute;
                                        }
                                    }
                                }
                            }
                        }
                    },

                    Event::TextEntered { unicode } => {
                        //println!("TEXT : {}", unicode);
                    },

                    Event::Resized { width, height } => {
                        self.size = Vector2u::new(width, height);
                        self.update_graphics();
                    },

                    _ => { },
                }
            }
        }
    }

    fn update_graphics(&mut self) {

        let panel_gap = self.context.theme.panel.gap * self.context.font_size as f32;
        let panel_size = Vector2f::new(self.size.x as f32 - panel_gap * 2.0, self.size.y as f32 - panel_gap * 2.0);
        let panel_position = Vector2f::new(panel_gap, panel_gap);

        self.panels.first_mut().unwrap().update_graphics(&self.context, panel_size);
        self.panels.first_mut().unwrap().update_position(panel_position);

        let view = View::from_rect(&FloatRect::new(0.0, 0.0, self.size.x as f32, self.size.y as f32));
        self.window.set_view(&view);
    }

    fn zoom_in(&mut self) {
        if self.context.font_size > 8 {
            self.context.font_size -= 1;
            self.update_graphics();
        }
    }

    fn zoom_out(&mut self) {
        if self.context.font_size < 34 {
            self.context.font_size += 1;
            self.update_graphics();
        }
    }

    fn move_focus_left(&mut self) {
        if self.focused_panel > 0 {
            self.focused_panel -= 1;
        }
    }

    fn move_focus_right(&mut self) {
        if self.focused_panel < self.panels.len() - 1 {
            self.focused_panel += 1;
        }
    }

    fn draw(&mut self) {
        self.window.clear(self.context.theme.panel.border);
        for (index, panel) in self.panels.iter_mut().enumerate() {
            panel.draw(&mut self.window, &self.context, self.focused_panel == index);
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

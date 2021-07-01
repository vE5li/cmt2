use sfml::{ graphics::*, system::*, window::* };

use seamonkey::*;
use context::{ Context, Action };
use input::*;
use editor::Editor;

pub struct PoetWindow<'w> {
    size: Vector2f,
    window: RenderWindow,
    surface: RectangleShape<'w>,
    framebuffer: RenderTexture,
    editor: Editor,
    focused: bool,
}

impl<'w> PoetWindow<'w> {

    pub fn editor(context: &Context) -> Status<Self> {

        let size = Vector2f::new(400.0, 400.0);

        let mut window = RenderWindow::new((400, 400), "poet", Style::DEFAULT, &Default::default());
        window.set_vertical_sync_enabled(true);

        let mut settings = ContextSettings::default();
        settings.set_antialiasing_level(context.antialiasing_level as u32);
        let framebuffer = RenderTexture::with_settings(400, 400, &settings).unwrap();
        let mut surface = RectangleShape::with_size(size);

        let texture_pointer = framebuffer.texture() as *const _;
        surface.set_texture(unsafe { &*texture_pointer }, false);

        let mut editor = confirm!(Editor::new());
        confirm!(editor.open_file(format_shared!("/home/main.cip"))); // new_file

        return success!(Self {
            size: size,
            window: window,
            surface: surface,
            framebuffer: framebuffer,
            editor: editor,
            focused: true,
        });
    }

    pub fn handle_input(&mut self, context: &Context) -> Vec<Action> {
        let mut action_queue = Vec::new();
        let mut handled = false;

        'handle: while let Some(event) = self.window.poll_event() {
            match event {

                Event::Closed => action_queue.push(Action::CloseWindow),

                Event::KeyPressed { code, shift, ctrl, alt, system } => {
                    if !is_modifier_key(code) {
                        let modifiers = Modifiers::from(shift, ctrl, alt, system);
                        let key_event = KeyEvent::new(code, modifiers);

                        //println!("modifiers: {:?}", modifiers);
                        //println!("key event: {:?}", key_event);

                        for action in context.get_matching_actions(&key_event) {

                            //println!("action: {:?}", action);

                            if let Some(unhandled_action) = self.editor.handle_action(context, action) {
                                if unhandled_action.is_global() {
                                    action_queue.push(unhandled_action);
                                    handled = true;
                                    continue 'handle;
                                }
                            } else {
                                self.rerender(context);
                                handled = true;
                                continue 'handle;
                            }
                        }
                    }
                },

                Event::TextEntered { unicode } => {
                    if handled {
                        handled = false;
                        continue 'handle;
                    }

                    let character = match unicode {
                        '\r' => Character::from_char('\n'),
                        char => Character::from_char(char),
                    };

                    self.editor.add_character(context, character);
                    self.rerender(context);
                },

                Event::Resized { width, height } => {
                    self.resize(context, Vector2f::new(width as f32, height as f32));
                    self.rerender(context);
                },

                Event::GainedFocus => {
                    self.focused = true;
                    self.rerender(context);
                },

                Event::LostFocus => {
                    self.focused = false;
                    self.rerender(context);
                },

                Event::MouseWheelScrolled { delta, .. } => {
                    match delta > 0.0 {
                        true => self.editor.scroll_up(context),
                        false => self.editor.scroll_down(context),
                    }
                    self.rerender(context);
                },

                ignored => {},
            }
        }

        return action_queue;
    }

    pub fn reallocate(&mut self, context: &Context) {
        let view = View::from_rect(&FloatRect::new(0.0, 0.0, self.size.x as f32, self.size.y as f32));
        self.window.set_view(&view);

        let mut settings = ContextSettings::default();
        settings.set_antialiasing_level(context.antialiasing_level as u32);
        self.framebuffer = RenderTexture::with_settings(self.size.x as u32, self.size.y as u32, &settings).unwrap();
        self.surface = RectangleShape::with_size(self.size);

        let texture_pointer = self.framebuffer.texture() as *const _;
        self.surface.set_texture(unsafe { &*texture_pointer }, false);
    }

    pub fn resize(&mut self, context: &Context, size: Vector2f) {
        self.size = size;
        self.reallocate(context);
        self.editor.resize(context, size);
    }

    pub fn rerender(&mut self, context: &Context) {
        self.editor.rerender(&mut self.framebuffer, context, self.focused);
    }

    pub fn display(&mut self) {
        self.window.clear(Color::BLACK);
        self.window.draw(&self.surface);
        self.window.display();
    }

    pub fn set_error_state(&mut self, error: Error) {
        self.editor.set_error_state(error);
    }

    pub fn close(&mut self) {
        self.window.close();
    }
}

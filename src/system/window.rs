use seamonkey::*;

#[cfg(feature = "debug")]
use debug::*;

use sfml::{ graphics::*, system::*, window::* };

use input::*;
use input::Action;
use themes::InterfaceTheme;
use interface::{ Interface, InterfaceContext };
use managers::{ FilebufferManager, LanguageManager };
use elements::TextbufferContext;

pub struct PoetWindow<'w> {
    size: Vector2f,
    window: RenderWindow,
    surface: RectangleShape<'w>,
    framebuffer: RenderTexture,
    interface: Interface,
    focused: bool,
}

impl<'w> PoetWindow<'w> {

    pub fn interface(interface_context: &InterfaceContext, filebuffer_manager: &mut FilebufferManager, language_manager: &mut LanguageManager, window_id: usize) -> Status<Self> {

        #[cfg(feature = "debug")]
        let timer = Timer::new("create interface");

        let size = Vector2f::new(400.0, 400.0);

        let mut window = RenderWindow::new((400, 400), "poet", Style::DEFAULT, &Default::default());
        window.set_vertical_sync_enabled(true);

        let mut settings = ContextSettings::default();
        settings.set_antialiasing_level(interface_context.antialiasing_level as u32);
        let framebuffer = RenderTexture::with_settings(400, 400, &settings).unwrap();
        let mut surface = RectangleShape::with_size(size);

        let texture_pointer = framebuffer.texture() as *const _;
        surface.set_texture(unsafe { &*texture_pointer }, false);

        let interface = confirm!(Interface::new(filebuffer_manager, language_manager, window_id));

        #[cfg(feature = "debug")]
        timer.stop();

        return success!(Self {
            size: size,
            window: window,
            surface: surface,
            framebuffer: framebuffer,
            interface: interface,
            focused: true,
        });
    }

    pub fn handle_input(&mut self, interface_context: &InterfaceContext, textbuffer_context: &TextbufferContext, theme: &InterfaceTheme, filebuffer_manager: &mut FilebufferManager, language_manager: &mut LanguageManager, theme_name: &mut SharedString) -> Vec<Action> {
        let mut action_queue = Vec::new();
        let mut force_rerender = false;
        let mut handled = false;

        if self.interface.history_catch_up(textbuffer_context, filebuffer_manager) {
            force_rerender = true;
        }

        'handle: while let Some(event) = self.window.poll_event() {
            match event {

                Event::Closed => action_queue.push(Action::CloseWindow),

                Event::KeyPressed { code, shift, ctrl, alt, system } => {
                    if !is_modifier_key(code) {

                        #[cfg(feature = "debug")]
                        let timer = Timer::new("key press");

                        let modifiers = Modifiers::from(shift, ctrl, alt, system);
                        let key_event = KeyEvent::new(code, modifiers);

                        for action in interface_context.get_matching_actions(&key_event) {
                            if let Some(unhandled_action) = self.interface.handle_action(interface_context, textbuffer_context, filebuffer_manager, language_manager, action, theme_name) {
                                if unhandled_action.is_global() {
                                    action_queue.push(unhandled_action);
                                    handled = true;

                                    #[cfg(feature = "debug")]
                                    print_debug!("dispatch global action {}{:?}{}", yellow(), unhandled_action, none());
                                    #[cfg(feature = "debug")]
                                    timer.stop();

                                    continue 'handle;
                                }

                                #[cfg(feature = "debug")]
                                print_debug!("unhandled action {}{:?}{}", yellow(), action, none());

                            } else {
                                self.rerender(interface_context, textbuffer_context, theme, filebuffer_manager);
                                handled = true;

                                #[cfg(feature = "debug")]
                                print_debug!("action {}{:?}{} handled by interface", yellow(), action, none());
                                #[cfg(feature = "debug")]
                                timer.stop();

                                continue 'handle;
                            }
                        }

                        #[cfg(feature = "debug")]
                        timer.stop();
                    }
                },

                Event::TextEntered { unicode } => {
                    if handled {
                        handled = false;
                        continue 'handle;
                    }

                    let character = match unicode as usize {
                        13 => Character::from_char('\n'),
                        0..=31 => continue 'handle,
                        32..=126 => Character::from_char(unicode),
                        _other => continue 'handle,
                    };

                    #[cfg(feature = "debug")]
                    let timer = Timer::new("add character");

                    self.interface.add_character(textbuffer_context, filebuffer_manager, language_manager, character);
                    force_rerender = true;

                    #[cfg(feature = "debug")]
                    timer.stop();
                },

                Event::Resized { width, height } => {

                    #[cfg(feature = "debug")]
                    let timer = Timer::new("window resize");

                    self.size = Vector2f::new(width as f32, height as f32);
                    self.reallocate(interface_context);
                    self.update_layout(interface_context, textbuffer_context, filebuffer_manager, theme);
                    force_rerender = true;

                    #[cfg(feature = "debug")]
                    timer.stop();
                },

                Event::GainedFocus => {

                    #[cfg(feature = "debug")]
                    let timer = Timer::new("gained focus");

                    self.focused = true;
                    force_rerender = true;

                    #[cfg(feature = "debug")]
                    timer.stop();
                },

                Event::LostFocus => {

                    #[cfg(feature = "debug")]
                    let timer = Timer::new("lost focus");

                    self.focused = false;
                    force_rerender = true;

                    #[cfg(feature = "debug")]
                    timer.stop();
                },

                Event::MouseWheelScrolled { delta, .. } => {

                    #[cfg(feature = "debug")]
                    let timer = Timer::new("mouse wheel scroll");

                    match delta > 0.0 {
                        true => self.interface.scroll_up(textbuffer_context),
                        false => self.interface.scroll_down(filebuffer_manager, textbuffer_context),
                    }
                    force_rerender = true;

                    #[cfg(feature = "debug")]
                    timer.stop();
                },

                _ignored => {},
            }
        }

        if force_rerender {
            self.rerender(interface_context, textbuffer_context, theme, filebuffer_manager);
        }

        return action_queue;
    }

    pub fn reallocate(&mut self, interface_context: &InterfaceContext) {

        #[cfg(feature = "debug")]
        let timer = Timer::new("reallocate resources");

        let view = View::from_rect(&FloatRect::new(0.0, 0.0, self.size.x as f32, self.size.y as f32));
        self.window.set_view(&view);

        let mut settings = ContextSettings::default();
        settings.set_antialiasing_level(interface_context.antialiasing_level as u32);
        self.framebuffer = RenderTexture::with_settings(self.size.x as u32, self.size.y as u32, &settings).unwrap();
        self.surface = RectangleShape::with_size(self.size);

        let texture_pointer = self.framebuffer.texture() as *const _;
        self.surface.set_texture(unsafe { &*texture_pointer }, false);

        #[cfg(feature = "debug")]
        timer.stop();
    }

    pub fn update_layout(&mut self, interface_context: &InterfaceContext, textbuffer_context: &TextbufferContext, filebuffer_manager: &FilebufferManager, theme: &InterfaceTheme) {

        #[cfg(feature = "debug")]
        let timer = Timer::new("update layout");

        self.interface.update_layout(interface_context, textbuffer_context, filebuffer_manager, theme, self.size);

        #[cfg(feature = "debug")]
        timer.stop();
    }

    pub fn rerender(&mut self, interface_context: &InterfaceContext, textbuffer_context: &TextbufferContext, theme: &InterfaceTheme, filebuffer_manager: &FilebufferManager) {

        #[cfg(feature = "debug")]
        let timer = Timer::new("rerender");

        self.framebuffer.clear(Color::BLACK);
        self.interface.render(&mut self.framebuffer, interface_context, textbuffer_context, theme, filebuffer_manager, self.focused);
        self.framebuffer.display();

        #[cfg(feature = "debug")]
        timer.stop();
    }

    pub fn display(&mut self) {
        self.window.clear(Color::BLACK);
        self.window.draw(&self.surface);
        self.window.display();
    }

    pub fn set_error_state(&mut self, error: Error) {
        self.interface.set_error_state(error);
    }

    pub fn close(&mut self) {

        #[cfg(feature = "debug")]
        let timer = Timer::new("close window");

        self.window.close();

        #[cfg(feature = "debug")]
        timer.stop();
    }
}

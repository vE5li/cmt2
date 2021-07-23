use seamonkey::*;

#[cfg(feature = "debug")]
use debug::*;

use input::Action;
use elements::TextbufferContext;
use interface::InterfaceContext;
use themes::InterfaceTheme;
use system::PoetWindow;
use managers::*;

pub struct Instance<'i> {
    windows: Vec<PoetWindow<'i>>,
    interface_context: InterfaceContext,
    textbuffer_context: TextbufferContext,
    interface_theme: InterfaceTheme,
    theme_name: SharedString,
    filebuffer_manager: FilebufferManager,
    language_manager: LanguageManager,
    window_counter: usize,
}

impl<'i> Instance<'i> {

    pub fn new(arguments: &Vec<String>) -> Self {

        #[cfg(feature = "debug")]
        let timer = Timer::new("create instance");

        #[cfg(feature = "debug")]
        let theme_timer = Timer::new("theme");

        let theme_name = SharedString::from("dark.data");
        let theme_file = format_shared!("/home/.config/poet/themes/{}", &theme_name);
        let theme_map = display!(read_map(&theme_file));
        let theme = display!(theme_map.index(&identifier!("interface")));
        let interface_theme = InterfaceTheme::load(theme, &theme_name);

        #[cfg(feature = "debug")]
        theme_timer.stop();

        #[cfg(feature = "debug")]
        let context_timer = Timer::new("context");

        let interface_context = display!(InterfaceContext::temp());
        let textbuffer_context = TextbufferContext::from();

        #[cfg(feature = "debug")]
        context_timer.stop();

        #[cfg(feature = "debug")]
        let manager_timer = Timer::new("managers");

        let filebuffer_manager = FilebufferManager::new();
        let language_manager = LanguageManager::new();

        #[cfg(feature = "debug")]
        manager_timer.stop();

        #[cfg(feature = "debug")]
        timer.stop();

        Self {
            windows: Vec::new(),
            interface_context: interface_context,
            textbuffer_context: textbuffer_context,
            interface_theme: interface_theme,
            theme_name: theme_name,
            filebuffer_manager: filebuffer_manager,
            language_manager: language_manager,
            window_counter: 0,
        }
    }

    pub fn new_interface(&mut self) -> Status<()> {

        #[cfg(feature = "debug")]
        let timer = Timer::new("new interface");

        let mut new_window = confirm!(PoetWindow::interface(&self.interface_context, &mut self.filebuffer_manager, &mut self.language_manager, self.window_counter));
        new_window.rerender(&self.interface_context, &self.textbuffer_context, &self.interface_theme, &mut self.filebuffer_manager);

        self.window_counter += 1;
        self.windows.push(new_window);

        #[cfg(feature = "debug")]
        timer.stop();

        return success!(());
    }

    pub fn has_open_windows(&self) -> bool {
        return !self.windows.is_empty();
    }

    pub fn handle_input(&mut self) {
        let mut index = 0;
        let mut force_rerender = false;
        let mut force_reallocate = false;
        let mut force_update = false;

        'handle: while index < self.windows.len() {
            for action in self.windows[index].handle_input(&self.interface_context, &self.textbuffer_context, &self.interface_theme, &mut self.filebuffer_manager, &mut self.language_manager, &mut self.theme_name) {
                match action {

                    Action::CloseWindow => {
                        self.windows[index].close();
                        self.windows.remove(index);
                        continue 'handle;
                    },

                    Action::NewWindow => {
                        if let Status::Error(error) = self.new_interface() {
                            self.windows[index].set_error_state(error);
                            self.windows[index].rerender(&self.interface_context, &self.textbuffer_context, &self.interface_theme, &self.filebuffer_manager);
                        }
                    },

                    Action::ZoomIn => {
                        if self.interface_context.zoom_in() {
                            force_update = true;
                            force_rerender = true;
                        }
                    },

                    Action::ZoomOut => {
                        if self.interface_context.zoom_out()  {
                            force_update = true;
                            force_rerender = true;
                        }
                    },

                    Action::IncreaseAntialiasing => {
                        if self.interface_context.increase_antialiasing() {
                            force_reallocate = true;
                            force_rerender = true;
                        }
                    },

                    Action::DecreaseAntialiasing => {
                        if self.interface_context.decrease_antialiasing() {
                            force_reallocate = true;
                            force_rerender = true;
                        }
                    },

                    Action::ToggleAppendLines => {
                        self.textbuffer_context.toggle_append_lines();
                        force_rerender = true;
                    },

                    Action::TogglePreserveLines => {
                        self.textbuffer_context.toggle_preserve_lines();
                        force_rerender = true;
                    },

                    Action::ToggleStartAtSymbol => {
                        self.textbuffer_context.toggle_start_at_symbol();
                        force_rerender = true;
                    },

                    Action::ToggleStatusBar => {
                        self.textbuffer_context.toggle_status_bar();
                        force_rerender = true;
                    },

                    Action::ToggleLineNumbers => {
                        self.textbuffer_context.toggle_line_numbers();
                        force_update = true;
                        force_rerender = true;
                    },

                    Action::ToggleSelectionLines => {
                        self.textbuffer_context.toggle_selection_lines();
                        force_rerender = true;
                    },

                    Action::ToggleHighlighting => {
                        self.textbuffer_context.toggle_highlighting();
                        force_rerender = true;
                    },

                    Action::ToggleUnfocusedSelections => {
                        self.textbuffer_context.toggle_unfocused_selections();
                        force_rerender = true;
                    },

                    Action::ToggleRelativeLineNumbers => {
                        self.textbuffer_context.toggle_relative_line_numbers();
                        force_rerender = true;
                    },

                    Action::Quit => {
                        panic!("implement");
                    },

                    Action::Reload => {

                        #[cfg(feature = "debug")]
                        let timer = Timer::new("reload");

                        #[cfg(feature = "debug")]
                        let theme_timer = Timer::new("theme");

                        let theme_file = format_shared!("/home/.config/poet/themes/{}", &self.theme_name);
                        let theme_map = display!(read_map(&theme_file));
                        let theme = display!(theme_map.index(&identifier!("interface")));
                        let interface_theme = InterfaceTheme::load(theme, &self.theme_name);
                        self.interface_theme = interface_theme;

                        #[cfg(feature = "debug")]
                        theme_timer.stop();

                        #[cfg(feature = "debug")]
                        let context_timer = Timer::new("context");

                        let interface_context = display!(InterfaceContext::temp());
                        self.interface_context = interface_context;

                        #[cfg(feature = "debug")]
                        context_timer.stop();

                        force_update = true;
                        force_rerender = true;

                        #[cfg(feature = "debug")]
                        timer.stop();

                        /*let configuration_directory = SharedString::from("/home/.config/poet/");

                        match Context::new(&configuration_directory) {

                            Status::Success(context) => {
                                //self.context = context;
                                force_update = true;
                                force_rerender = true;
                            },

                            Status::Error(error) => {
                                self.windows[index].set_error_state(error);
                                self.windows[index].rerender(&self.interface_context, &self.interface_theme);
                            }
                        }*/
                    },

                    _unhandled => { },
                }
            }

            index += 1;
        }

        if force_update {
            let interface_context = &self.interface_context;
            let textbuffer_context = &self.textbuffer_context;
            let filebuffer_manager = &self.filebuffer_manager;
            let interface_theme = &self.interface_theme;
            self.windows.iter_mut().for_each(|window| window.update_layout(interface_context, textbuffer_context, filebuffer_manager, interface_theme));
        }

        if force_reallocate {
            let interface_context = &self.interface_context;
            self.windows.iter_mut().for_each(|window| window.reallocate(interface_context));
        }

        if force_rerender {
            let interface_context = &self.interface_context;
            let textbuffer_context = &self.textbuffer_context;
            let interface_theme = &self.interface_theme;
            let filebuffer_manager = &self.filebuffer_manager;
            self.windows.iter_mut().for_each(|window| window.rerender(interface_context, textbuffer_context, interface_theme, filebuffer_manager));
        }

        self.windows.iter_mut().for_each(|window| window.display());
    }

    pub fn close(&self) {
    }
}

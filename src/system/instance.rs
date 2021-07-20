use seamonkey::*;

use elements::TextbufferContext;
use interface::{ InterfaceTheme, InterfaceContext };
use input::Action;
use system::{ PoetWindow, ResourceManager, LanguageManager };

pub struct Instance<'i> {
    windows: Vec<PoetWindow<'i>>,
    interface_context: InterfaceContext,
    textbuffer_context: TextbufferContext,
    interface_theme: InterfaceTheme,
    theme_name: SharedString,

    resource_manager: ResourceManager,
    language_manager: LanguageManager,
    window_counter: usize,
}

impl<'i> Instance<'i> {

    pub fn new(arguments: &Vec<String>) -> Self {

        //let configuration_directory = SharedString::from("/home/.config/poet/");
        //let context = display!(Context::new(&configuration_directory));

        //let (interface_context, textbuffer_context) = display!(load_context(&configuration_directory));
        //let interface_theme = display!(InterfaceTheme::new(&configuration_directory));

        let theme_name = SharedString::from("dark");

        let theme_file = format_shared!("/home/.config/poet/themes/{}.data", &theme_name);
        let theme_map = display!(read_map(&theme_file));
        let theme = display!(theme_map.index(&identifier!("interface")));

        let interface_context = display!(InterfaceContext::temp());
        let interface_theme = InterfaceTheme::load(theme, &theme_name);

        Self {
            windows: Vec::new(),
            interface_context: interface_context,
            textbuffer_context: TextbufferContext::from(),
            interface_theme: interface_theme,
            theme_name: theme_name,

            resource_manager: ResourceManager::new(),
            language_manager: LanguageManager::new(),
            window_counter: 0,
        }
    }

    pub fn new_editor(&mut self) -> Status<()> {
        let mut new_window = confirm!(PoetWindow::interface(&self.interface_context, &mut self.resource_manager, &mut self.language_manager, self.window_counter));
        new_window.rerender(&self.interface_context, &self.textbuffer_context, &self.interface_theme, &mut self.resource_manager);

        self.window_counter += 1;
        self.windows.push(new_window);
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
            for action in self.windows[index].handle_input(&self.interface_context, &self.textbuffer_context, &self.interface_theme, &mut self.resource_manager, &mut self.language_manager, &mut self.theme_name) {
                match action {

                    Action::CloseWindow => {
                        self.windows[index].close();
                        self.windows.remove(index);
                        continue 'handle;
                    },

                    Action::NewInterface => {
                        if let Status::Error(error) = self.new_editor() {
                            self.windows[index].set_error_state(error);
                            self.windows[index].rerender(&self.interface_context, &self.textbuffer_context, &self.interface_theme, &self.resource_manager);
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

                        let theme_file = format_shared!("/home/.config/poet/themes/{}.data", &self.theme_name);
                        let theme_map = display!(read_map(&theme_file));
                        let theme = display!(theme_map.index(&identifier!("interface")));

                        let interface_context = display!(InterfaceContext::temp());
                        let interface_theme = InterfaceTheme::load(theme, &self.theme_name);

                        self.interface_context = interface_context;
                        self.interface_theme = interface_theme;

                        force_update = true;
                        force_rerender = true;

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
            let resource_manager = &self.resource_manager;
            let interface_theme = &self.interface_theme;
            self.windows.iter_mut().for_each(|window| window.update_layout(interface_context, textbuffer_context, resource_manager, interface_theme));
        }

        if force_reallocate {
            let interface_context = &self.interface_context;
            self.windows.iter_mut().for_each(|window| window.reallocate(interface_context));
        }

        if force_rerender {
            let interface_context = &self.interface_context;
            let textbuffer_context = &self.textbuffer_context;
            let interface_theme = &self.interface_theme;
            let resource_manager = &self.resource_manager;
            self.windows.iter_mut().for_each(|window| window.rerender(interface_context, textbuffer_context, interface_theme, resource_manager));
        }

        self.windows.iter_mut().for_each(|window| window.display());
    }

    pub fn close(&self) {
    }
}

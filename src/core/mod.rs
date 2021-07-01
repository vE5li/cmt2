use seamonkey::*;
use context::{ Context, Action };
use window::PoetWindow;

pub struct Instance<'i> {
    windows: Vec<PoetWindow<'i>>,
    context: Context,
}

impl<'i> Instance<'i> {

    pub fn new(arguments: &Vec<String>) -> Self {

        let configuration_directory = SharedString::from("/home/.config/poet/");
        let context = display!(Context::new(&configuration_directory));

        Self {
            windows: Vec::new(),
            context: context,
        }
    }

    pub fn new_editor(&mut self) -> Status<()> {
        let mut new_window = confirm!(PoetWindow::editor());
        new_window.rerender(&self.context);

        self.windows.push(new_window);
        return success!(());
    }

    pub fn has_open_windows(&self) -> bool {
        return !self.windows.is_empty();
    }

    pub fn handle_input(&mut self) {
        let mut index = 0;
        let mut force_rerender = false;

        'handle: while index < self.windows.len() {
            self.windows[index].display();

            for action in self.windows[index].handle_input(&self.context) {
                match action {

                    Action::CloseWindow => {
                        self.windows[index].close();
                        self.windows.remove(index);
                        continue 'handle;
                    },

                    Action::NewEditor => {
                        display!(self.new_editor());
                    },

                    Action::ZoomIn => {
                        self.context.zoom_in();
                        //self.check_selection_gaps();
                        force_rerender = true;
                    },

                    Action::ZoomOut => {
                        self.context.zoom_out();
                        force_rerender = true;
                    },

                    Action::ToggleAppendLines => {
                        panic!("implement");
                    },

                    Action::ToggleStatusBar => {
                        panic!("implement");
                    },

                    Action::ToggleLineNumbers => {
                        panic!("implement");
                    },

                    Action::ToggleSelectionLines => {
                        panic!("implement");
                    },

                    Action::ToggleHighlighting => {
                        panic!("implement");
                    },

                    Action::ToggleUnfocusedSelections => {
                        panic!("implement");
                    },

                    Action::ToggleFocusBar => {
                        panic!("implement");
                    },

                    Action::Quit => {
                        panic!("implement");
                    },

                    _unhandled => { },
                }
            }

            index += 1;
        }

        if force_rerender {
            let context = &self.context;
            self.windows.iter_mut().for_each(|window| window.rerender(context));
        }
    }

    pub fn close(&self) {
        self.context.safe();
    }
}

mod action;

use seamonkey::*;
use context::Context;
use window::PoetWindow;

pub use self::action::CoreAction;

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

            while let Some(action) = self.windows[index].handle_input(&self.context) {
                match action {
                    CoreAction::CloseWindow => {
                        self.windows.remove(index);
                        continue 'handle;
                    },
                    CoreAction::NewEditor => {
                        display!(self.new_editor());
                    },
                    CoreAction::ZoomIn => {
                        self.context.zoom_in();
                        //self.check_selection_gaps();
                        force_rerender = true;
                    },
                    CoreAction::ZoomOut => {
                        self.context.zoom_out();
                        force_rerender = true;
                    },
                    CoreAction::Quit => {
                        //TODO:
                        //self.windows.iter_mut().for_each(|window| window.close());
                    },
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

mod mode;
mod editor;

use kami::*;
use self::mode::PanelMode;
use self::editor::Editor;
use context::{ Context, Action };
use sfml::graphics::*;
use sfml::system::Vector2f;
use sfml::window::ContextSettings;
use graphics::RoundedRectangle;


pub struct Panel<'p> {
    mode: PanelMode,
    framebuffer: RenderTexture,
    surface: CustomShape<'p>,
    size: Vector2f,
}

impl<'p> Panel<'p> {

    pub fn new_editor(font_size: usize) -> Status<Self> {
        let editor = confirm!(Editor::new(font_size));
        let mut surface = CustomShape::new(Box::new(RoundedRectangle::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0)));

        success!(Self {
            mode: PanelMode::Editor(editor),
            framebuffer: RenderTexture::new(0, 0, false).unwrap(),
            surface: surface,
            size: Vector2f::new(0.0, 0.0),
        })
    }

    pub fn new_terminal() -> Status<Self> {
        let mut surface = CustomShape::new(Box::new(RoundedRectangle::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0)));

        success!(Self {
            mode: PanelMode::Terminal,
            framebuffer: RenderTexture::new(0, 0, false).unwrap(),
            surface: surface,
            size: Vector2f::new(0.0, 0.0),
        })
    }

    pub fn update(&mut self, context: &Context, focused: bool) {
        match &self.mode {
            PanelMode::Editor(editor) => editor.draw(&mut self.framebuffer, context, focused),
            PanelMode::Terminal => { },
        }
    }

    pub fn update_graphics(&mut self, context: &Context, focused: bool, size: Vector2f) {

        //let mut settings = ContextSettings::default();
        //let mut framebuffer = RenderTexture::with_settings(size.x.ceil() as u32, size.y.ceil() as u32, &settings).unwrap();

        let mut framebuffer = RenderTexture::new(size.x.ceil() as u32, size.y.ceil() as u32, false).unwrap();
        framebuffer.clear(context.theme.panel.background);
        self.framebuffer = framebuffer;

        let panel_radius = context.theme.panel.radius * context.font_size as f32;
        self.surface = CustomShape::new(Box::new(RoundedRectangle::new(size.x, size.y, panel_radius, panel_radius, panel_radius, panel_radius)));
        let texture_pointer = self.framebuffer.texture() as *const _;
        self.surface.set_texture(unsafe { &*texture_pointer }, false);
        self.surface.set_outline_thickness(0.0);

        match &mut self.mode {
            PanelMode::Editor(editor) => editor.update_graphics(context, size),
            PanelMode::Terminal => { },
        }

        self.update(context, focused);
        self.size = size;
    }

    pub fn update_position(&mut self, position: Vector2f) {
        self.surface.set_position(position);
    }

    pub fn handle_action(&mut self, context: &Context, action: Action) -> Status<bool> {
        let handled = match &mut self.mode {
            PanelMode::Editor(editor) => confirm!(editor.handle_action(context, action)),
            PanelMode::Terminal => false,
        };

        if handled {
            self.update(context, true);
        }

        return success!(handled);
    }

    pub fn add_character(&mut self, context: &Context, character: Character) {
        match &mut self.mode {
            PanelMode::Editor(editor) => editor.add_character(context, character),
            PanelMode::Terminal => { },
        };
        self.update(context, true);
    }

    pub fn draw(&mut self, window: &mut RenderWindow) {
        window.draw(&self.surface);
    }
}

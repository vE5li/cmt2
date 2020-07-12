use sfml::graphics::{ Color, TextStyle };

#[derive(Clone, Debug)]
pub struct PanelTheme {
    pub border: Color,
    pub background: Color,
    pub text: Color,
    pub comment: Color,
    pub string: Color,
    pub character: Color,
    pub integer: Color,
    pub float: Color,
    pub keyword: Color,
    pub operator: Color,
    pub identifier: Color,
    pub type_identifier: Color,
    pub error: Color,
    pub gap: f32,
    pub radius: f32,
    pub left_offset: f32,
    pub top_offset: f32,
    pub right_offset: f32,
    pub style: TextStyle,
}

impl PanelTheme {

    pub fn new() -> Self {
        Self {
            border: Color::rgb(60, 60, 60),
            background: Color::rgb(35, 35, 35),
            text: Color::rgb(160, 160, 160),
            comment: Color::rgb(100, 100, 100),
            string: Color::rgb(100, 150, 140),
            character: Color::rgb(35, 155, 140),
            integer: Color::rgb(45, 110, 135),
            float: Color::rgb(45, 110, 135),
            keyword: Color::rgb(145, 100, 145),
            operator: Color::rgb(130, 130, 130),
            identifier: Color::rgb(160, 160, 160),
            type_identifier: Color::rgb(210, 100, 150),
            error: Color::rgb(160, 60, 60),
            radius: 0.2,
            gap: 0.5,
            left_offset: 0.4,
            top_offset: 0.4,
            right_offset: 0.4,
            style: TextStyle::REGULAR,
        }
    }
}

#[derive(Clone, Debug)]
pub struct LineNumberTheme {
    pub background: Color,
    pub text: Color,
    pub width: f32,
    pub offset: f32,
    pub gap: f32,
    pub radius: f32,
    pub text_offset: f32,
    pub style: TextStyle,
}

impl LineNumberTheme {

    pub fn new() -> Self {
        Self {
            background: Color::rgb(45, 45, 45),
            text: Color::rgb(100, 100, 100),
            width: 5.0,
            offset: 0.0,
            gap: 0.0,
            radius: 0.0,
            text_offset: 0.5,
            style: TextStyle::REGULAR,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DialogueTheme {
    pub background: Color,
    pub focused: Color,
    pub text: Color,
    pub ghost: Color,
    pub focused_text: Color,
    pub focused_ghost: Color,
    pub style: TextStyle,
    pub ghost_style: TextStyle,
    pub height: f32,
}

impl DialogueTheme {

    pub fn new() -> Self {
        Self {
            background: Color::rgb(45, 45, 45),
            focused: Color::rgb(70, 70, 70),
            ghost: Color::rgb(70, 70, 70),
            text: Color::rgb(90, 90, 90),
            focused_text: Color::rgb(130, 130, 130),
            focused_ghost: Color::rgb(100, 100, 100),
            style: TextStyle::REGULAR,
            ghost_style: TextStyle::ITALIC,
            height: 1.5,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FocusBarTheme {
    pub background: Color,
    pub height: f32,
}

impl FocusBarTheme {

    pub fn new() -> Self {
        Self {
            background: Color::rgb(130, 80, 100),
            height: 0.5,
        }
    }
}

#[derive(Clone, Debug)]
pub struct StatusBarTheme {
    pub background: Color,
    pub text: Color,
    pub height: f32,
    pub offset: f32,
    pub style: TextStyle,
}

impl StatusBarTheme {

    pub fn new() -> Self {
        Self {
            background: Color::rgb(130, 80, 100),
            text: Color::rgb(100, 100, 100),
            height: 1.5,
            offset: 2.0,
            style: TextStyle::REGULAR,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SelectionTheme {
    pub background: Color,
    pub text: Color,
    pub new_background: Color,
    pub new_text: Color,
    pub line: Color,
    pub radius: f32,
    pub style: TextStyle,
}

impl SelectionTheme {

    pub fn new() -> Self {
        Self {
            background: Color::rgb(115, 115, 115),
            text: Color::rgb(35, 35, 35),
            new_background: Color::rgb(145, 105, 130),
            new_text: Color::rgb(35, 35, 35),
            line: Color::rgb(50, 50, 50),
            radius: 0.05,
            style: TextStyle::BOLD,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Theme {
    pub panel: PanelTheme,
    pub line_number: LineNumberTheme,
    pub dialogue: DialogueTheme,
    pub status_bar: StatusBarTheme,
    pub focus_bar: FocusBarTheme,
    pub selection: SelectionTheme,
}

impl Theme {

    pub fn new() -> Self {
        Self {
            panel: PanelTheme::new(),
            line_number: LineNumberTheme::new(),
            dialogue: DialogueTheme::new(),
            status_bar: StatusBarTheme::new(),
            focus_bar: FocusBarTheme::new(),
            selection: SelectionTheme::new(),
        }
    }
}

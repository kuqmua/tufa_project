#[derive(PartialEq)]
pub enum DrawerPosition {
    Left,
    Right,
}

impl DrawerPosition {
    pub fn get_style(&self) -> DrawerPositionStyle {
        match *self {
            DrawerPosition::Left => DrawerPositionStyle {
                translate_sign: std::string::String::from("-"),
                left_value: std::string::String::from("0"),
                right_value: std::string::String::from("auto"),
            },
            DrawerPosition::Right => DrawerPositionStyle {
                translate_sign: std::string::String::from("+"),
                left_value: std::string::String::from("auto"),
                right_value: std::string::String::from("0"),
            },
        }
    }
}

pub struct DrawerPositionStyle {
    pub translate_sign: std::string::String, //maybe change later
    pub left_value: std::string::String,
    pub right_value: std::string::String,
}

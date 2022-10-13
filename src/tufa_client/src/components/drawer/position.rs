#[derive(Debug, PartialEq)]
pub enum DrawerPosition {
    Left,
    Right,
}

impl DrawerPosition {
    pub fn get_style(&self) -> DrawerPositionStyle {
        match *self {
            DrawerPosition::Left => DrawerPositionStyle {
                translate_sign: String::from("-"),
                left_value: String::from("0"),
                right_value: String::from("auto"),
            },
            DrawerPosition::Right => DrawerPositionStyle {
                translate_sign: String::from("+"),
                left_value: String::from("auto"),
                right_value: String::from("0"),
            },
        }
    }
}

#[derive(Debug)]
pub struct DrawerPositionStyle {
    pub translate_sign: String, //maybe change later
    pub left_value: String,
    pub right_value: String,
}

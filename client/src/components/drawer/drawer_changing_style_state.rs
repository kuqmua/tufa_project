#[derive(PartialEq)]
pub enum DrawerChangingStyleState {
    Initial,
    OpenedBeforeTimeout,
    OpenedAfterTimeout,
    ClosedBeforeTimeout,
}

impl Clone for DrawerChangingStyleState {
    fn clone(&self) -> Self {
        match *self {
            DrawerChangingStyleState::Initial => DrawerChangingStyleState::Initial,
            DrawerChangingStyleState::OpenedBeforeTimeout => {
                DrawerChangingStyleState::OpenedBeforeTimeout
            }
            DrawerChangingStyleState::OpenedAfterTimeout => {
                DrawerChangingStyleState::OpenedAfterTimeout
            }
            DrawerChangingStyleState::ClosedBeforeTimeout => {
                DrawerChangingStyleState::ClosedBeforeTimeout
            }
        }
    }
}

impl DrawerChangingStyleState {
    pub fn get_value(&self, translate_sign: std::string::String) -> DrawerChangingStyle {
        match *self {
            DrawerChangingStyleState::Initial => DrawerChangingStyle {
                display: std::string::String::from("none"),
                transform: format!("translate3d({}100%, 0, 0)", translate_sign),
                webkit_transform: format!("translate3d({}100%, 0, 0)", translate_sign),
                opacity: std::string::String::from(""),
            },
            DrawerChangingStyleState::OpenedBeforeTimeout => DrawerChangingStyle {
                display: std::string::String::from("block"),
                transform: format!("translate3d({}100%, 0, 0)", translate_sign),
                webkit_transform: format!("translate3d({}100%, 0, 0)", translate_sign),
                opacity: std::string::String::from(""),
            },
            DrawerChangingStyleState::OpenedAfterTimeout => DrawerChangingStyle {
                display: std::string::String::from("block"),
                transform: std::string::String::from("translate3d(0, 0, 0)"),
                webkit_transform: std::string::String::from("translate3d(0, 0, 0)"),
                opacity: std::string::String::from("0.5"),
            },
            DrawerChangingStyleState::ClosedBeforeTimeout => DrawerChangingStyle {
                display: std::string::String::from("block"),
                transform: format!("translate3d({}100%, 0, 0)", translate_sign),
                webkit_transform: format!("translate3d({}100%, 0, 0)", translate_sign),
                opacity: std::string::String::from(""),
            },
        }
    }
}

#[derive(PartialEq)]
pub struct DrawerChangingStyle {
    pub display: std::string::String,
    pub transform: std::string::String,
    pub webkit_transform: std::string::String,
    pub opacity: std::string::String,
}

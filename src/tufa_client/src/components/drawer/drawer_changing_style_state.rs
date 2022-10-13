#[derive(Debug, PartialEq)]
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
    pub fn get_value(&self, translate_sign: String) -> DrawerChangingStyle {
        match *self {
            DrawerChangingStyleState::Initial => DrawerChangingStyle {
                display: String::from("none"),
                transform: format!("translate3d({}100%, 0, 0)", translate_sign),
                webkit_transform: format!("translate3d({}100%, 0, 0)", translate_sign),
                opacity: String::from(""),
            },
            DrawerChangingStyleState::OpenedBeforeTimeout => DrawerChangingStyle {
                display: String::from("block"),
                transform: format!("translate3d({}100%, 0, 0)", translate_sign),
                webkit_transform: format!("translate3d({}100%, 0, 0)", translate_sign),
                opacity: String::from(""),
            },
            DrawerChangingStyleState::OpenedAfterTimeout => DrawerChangingStyle {
                display: String::from("block"),
                transform: String::from("translate3d(0, 0, 0)"),
                webkit_transform: String::from("translate3d(0, 0, 0)"),
                opacity: String::from("0.5"),
            },
            DrawerChangingStyleState::ClosedBeforeTimeout => DrawerChangingStyle {
                display: String::from("block"),
                transform: format!("translate3d({}100%, 0, 0)", translate_sign),
                webkit_transform: format!("translate3d({}100%, 0, 0)", translate_sign),
                opacity: String::from(""),
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct DrawerChangingStyle {
    pub display: String,
    pub transform: String,
    pub webkit_transform: String,
    pub opacity: String,
}

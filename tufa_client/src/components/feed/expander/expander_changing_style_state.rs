#[derive(PartialEq)]
pub enum ExpanderChangingStyleState {
    Initial,
    OpenedBeforeTimeout,
    OpenedAfterTimeout,
    ClosedBeforeTimeout,
}

impl Clone for ExpanderChangingStyleState {
    fn clone(&self) -> Self {
        match *self {
            ExpanderChangingStyleState::Initial => ExpanderChangingStyleState::Initial,
            ExpanderChangingStyleState::OpenedBeforeTimeout => {
                ExpanderChangingStyleState::OpenedBeforeTimeout
            }
            ExpanderChangingStyleState::OpenedAfterTimeout => {
                ExpanderChangingStyleState::OpenedAfterTimeout
            }
            ExpanderChangingStyleState::ClosedBeforeTimeout => {
                ExpanderChangingStyleState::ClosedBeforeTimeout
            }
        }
    }
}

impl ExpanderChangingStyleState {
    pub fn get_value(&self) -> ExpanderChangingStyle {
        match *self {
            ExpanderChangingStyleState::Initial => ExpanderChangingStyle {
                display: std::string::String::from("none"),
                transform: std::string::String::from("translate3d(0, 100%, 0)"),
                webkit_transform: std::string::String::from("translate3d(0, 100%, 0)"),
                opacity: std::string::String::from(""),
            },
            ExpanderChangingStyleState::OpenedBeforeTimeout => ExpanderChangingStyle {
                display: std::string::String::from("block"),
                transform: std::string::String::from("translate3d(0, 100%, 0)"),
                webkit_transform: std::string::String::from("translate3d(0, 100%, 0)"),
                opacity: std::string::String::from(""),
            },
            ExpanderChangingStyleState::OpenedAfterTimeout => ExpanderChangingStyle {
                display: std::string::String::from("block"),
                transform: std::string::String::from("translate3d(0, 0, 0)"),
                webkit_transform: std::string::String::from("translate3d(0, 0, 0)"),
                opacity: std::string::String::from("0.5"),
            },
            ExpanderChangingStyleState::ClosedBeforeTimeout => ExpanderChangingStyle {
                display: std::string::String::from("block"),
                transform: std::string::String::from("translate3d(0, 100%, 0)"),
                webkit_transform: std::string::String::from("translate3d(0, 100%, 0)"),
                opacity: std::string::String::from(""),
            },
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct ExpanderChangingStyle {
    pub display: std::string::String,
    pub transform: std::string::String,
    pub webkit_transform: std::string::String,
    pub opacity: std::string::String,
}

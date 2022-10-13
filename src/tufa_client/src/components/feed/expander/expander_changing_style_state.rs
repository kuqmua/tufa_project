#[derive(Debug, PartialEq)]
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
                display: String::from("none"),
                transform: String::from("translate3d(0, 100%, 0)"),
                webkit_transform: String::from("translate3d(0, 100%, 0)"),
                opacity: String::from(""),
            },
            ExpanderChangingStyleState::OpenedBeforeTimeout => ExpanderChangingStyle {
                display: String::from("block"),
                transform: String::from("translate3d(0, 100%, 0)"),
                webkit_transform: String::from("translate3d(0, 100%, 0)"),
                opacity: String::from(""),
            },
            ExpanderChangingStyleState::OpenedAfterTimeout => ExpanderChangingStyle {
                display: String::from("block"),
                transform: String::from("translate3d(0, 0, 0)"),
                webkit_transform: String::from("translate3d(0, 0, 0)"),
                opacity: String::from("0.5"),
            },
            ExpanderChangingStyleState::ClosedBeforeTimeout => ExpanderChangingStyle {
                display: String::from("block"),
                transform: String::from("translate3d(0, 100%, 0)"),
                webkit_transform: String::from("translate3d(0, 100%, 0)"),
                opacity: String::from(""),
            },
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExpanderChangingStyle {
    pub display: String,
    pub transform: String,
    pub webkit_transform: String,
    pub opacity: String,
}

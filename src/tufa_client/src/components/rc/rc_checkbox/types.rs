use std::fmt;
// use yew_stdweb::ChangeData;
use web_sys::KeyboardEvent;
use yew::Callback;
// use yew::Event;
use yew::FocusEvent;
use yew::MouseEvent;
use yew::NodeRef;
use yew::Properties;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InputType {
    Button,
    Checkbox,
    Color,
    Date,
    DatetimeLocal,
    Email,
    File,
    Hidden,
    Image,
    Month,
    Number,
    Password,
    Radio,
    Range,
    Reset,
    Search,
    Submit,
    Tel,
    Text,
    Time,
    Url,
    Week,
}

impl fmt::Display for InputType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InputType::Button => write!(f, "button"),
            InputType::Checkbox => write!(f, "checkbox"),
            InputType::Color => write!(f, "color"),
            InputType::Date => write!(f, "date"),
            InputType::DatetimeLocal => write!(f, "datetime-local"),
            InputType::Email => write!(f, "email"),
            InputType::File => write!(f, "file"),
            InputType::Hidden => write!(f, "hidden"),
            InputType::Image => write!(f, "image"),
            InputType::Month => write!(f, "month"),
            InputType::Number => write!(f, "number"),
            InputType::Password => write!(f, "password"),
            InputType::Radio => write!(f, "radio"),
            InputType::Range => write!(f, "range"),
            InputType::Reset => write!(f, "reset"),
            InputType::Search => write!(f, "search"),
            InputType::Submit => write!(f, "submit"),
            InputType::Tel => write!(f, "tel"),
            InputType::Text => write!(f, "text"),
            InputType::Time => write!(f, "time"),
            InputType::Url => write!(f, "url"),
            InputType::Week => write!(f, "week"),
        }
    }
}

#[derive(Debug, PartialEq, Properties, Clone)]
pub struct RcCheckBoxProps {
    pub prefix_cls: Option<String>,
    pub class_name: Option<String>,
    pub style: Option<String>, //React.CSSProperties
    pub name: Option<String>,
    pub id: Option<String>,
    pub type_handle: Option<InputType>,
    pub title: Option<String>,
    pub default_checked: Option<()>, //?: number | boolean;
    pub checked: Option<()>,         //?: number | boolean;
    pub disabled: Option<()>,
    pub on_focus: Option<Callback<FocusEvent>>,
    pub on_blur: Option<Callback<FocusEvent>>,
    // pub on_change: Option<Callback<Event>>,
    pub on_click: Option<Callback<MouseEvent>>,
    pub on_key_down: Option<Callback<KeyboardEvent>>,
    pub on_key_press: Option<Callback<KeyboardEvent>>,
    pub on_key_up: Option<Callback<KeyboardEvent>>,
    pub tab_index: Option<i32>, //?: string | number;
    pub read_only: Option<()>,
    pub required: Option<()>,
    pub auto_focus: Option<()>,
    pub value: Option<String>,
}

#[derive(Debug, PartialEq, Properties, Clone)]
pub struct CustomCheckBoxProps {
    pub prefix_cls: Option<String>,
    pub class_name: Option<String>,
    pub style: Option<String>, //React.CSSProperties
    pub name: Option<String>,
    pub id: Option<String>,
    pub type_handle: Option<InputType>,
    pub title: Option<String>,
    pub default_checked: Option<()>, //?: number | boolean;
    pub checked: Option<()>,         //?: number | boolean;
    pub disabled: Option<()>,
    pub on_focus: Option<Callback<FocusEvent>>,
    pub on_blur: Option<Callback<FocusEvent>>,
    pub on_change: Option<Callback<()>>, //Event//ChangeData
    pub on_click: Option<Callback<MouseEvent>>,
    pub on_key_down: Option<Callback<KeyboardEvent>>,
    pub on_key_press: Option<Callback<KeyboardEvent>>,
    pub on_key_up: Option<Callback<KeyboardEvent>>,
    pub tab_index: Option<i32>, //?: string | number;
    pub read_only: Option<()>,
    pub required: Option<()>,
    pub auto_focus: Option<()>,
    pub value: Option<String>,
    pub reference: NodeRef,
}

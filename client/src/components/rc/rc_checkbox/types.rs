use std::fmt;
// use yew_stdweb::ChangeData;
use web_sys::KeyboardEvent;
use yew::Callback;
// use yew::Event;
use yew::FocusEvent;
use yew::MouseEvent;
use yew::NodeRef;
use yew::Properties;

#[derive(PartialEq, Eq, Clone)]
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputType::Button => write!(formatter, "button"),
            InputType::Checkbox => write!(formatter, "checkbox"),
            InputType::Color => write!(formatter, "color"),
            InputType::Date => write!(formatter, "date"),
            InputType::DatetimeLocal => write!(formatter, "datetime-local"),
            InputType::Email => write!(formatter, "email"),
            InputType::File => write!(formatter, "file"),
            InputType::Hidden => write!(formatter, "hidden"),
            InputType::Image => write!(formatter, "image"),
            InputType::Month => write!(formatter, "month"),
            InputType::Number => write!(formatter, "number"),
            InputType::Password => write!(formatter, "password"),
            InputType::Radio => write!(formatter, "radio"),
            InputType::Range => write!(formatter, "range"),
            InputType::Reset => write!(formatter, "reset"),
            InputType::Search => write!(formatter, "search"),
            InputType::Submit => write!(formatter, "submit"),
            InputType::Tel => write!(formatter, "tel"),
            InputType::Text => write!(formatter, "text"),
            InputType::Time => write!(formatter, "time"),
            InputType::Url => write!(formatter, "url"),
            InputType::Week => write!(formatter, "week"),
        }
    }
}

#[derive(PartialEq, Eq, Properties, Clone)]
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

#[derive(PartialEq, Eq, Properties, Clone)]
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

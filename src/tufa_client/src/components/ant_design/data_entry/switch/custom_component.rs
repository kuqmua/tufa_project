use web_sys::KeyboardEvent;
use web_sys::MouseEvent;
use yew::function_component;
use yew::html;
use yew::use_state;
use yew::Callback;
use yew::Html;
use yew::NodeRef;
use yew::Properties;

#[derive(Debug, PartialEq, Properties, Clone)]
pub struct CustomSwitchProps {
    pub class: Option<String>,
    pub disabled: Option<()>,
    pub checked_children: Option<Html>,   //todo
    pub unchecked_children: Option<Html>, //todo
    pub on_change: Option<Callback<(bool, MouseOrKeyboardEvent)>>,
    pub on_key_down: Option<Callback<KeyboardEvent>>,
    pub on_click: Option<Callback<(bool, MouseEvent)>>,
    pub tab_index: Option<i32>,
    pub checked: Option<()>,
    pub default_checked: Option<()>,
    pub loading_icon: Option<Html>, //todo
    pub style: Option<String>,
    pub title: Option<String>,
    pub reference: NodeRef,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MouseOrKeyboardEvent {
    MouseEvent(MouseEvent),
    KeyboardEvent(KeyboardEvent),
}

#[function_component(CustomSwitch)]
pub fn custom_switch(prop: &CustomSwitchProps) -> Html {
    let props = prop.clone();
    let class = match props.class.clone() {
        None => String::from(""),
        Some(cn) => cn,
    };
    let disabled = props.disabled.clone().is_some();
    let checked_children = match props.checked_children {
        None => html! {},
        Some(cc) => cc,
    };
    let unchecked_children = match props.unchecked_children {
        None => html! {},
        Some(uc) => uc,
    };
    let checked = props.checked.is_some();
    let default_checked = props.default_checked.is_some();
    let loading_icon = match props.loading_icon.clone() {
        None => html! {},
        Some(li) => li,
    };
    let tab_index = match props.tab_index {
        None => String::from(""),
        Some(ti) => ti.to_string(),
    };
    let style = match props.style.clone() {
        None => String::from(""),
        Some(s) => s,
    };
    let title = match props.title.clone() {
        None => String::from(""),
        Some(t) => t,
    };
    let inner_checked = use_state(|| match (checked, default_checked) {
        (true, true) => checked,
        (true, false) => checked,
        (false, true) => default_checked,
        (false, false) => false,
    });
    let inner_checked_first_cloned = inner_checked.clone();
    let trigger_change = move |e: (bool, MouseOrKeyboardEvent)| {
        let mut merged_checked = *inner_checked_first_cloned;
        if !disabled {
            merged_checked = e.0;
            inner_checked_first_cloned.set(merged_checked);
            if let Some(on_change) = props.on_change {
                on_change.emit((merged_checked, e.1));
            };
        }
        merged_checked
    };
    let trigger_change_cloned = trigger_change.clone();
    let on_internal_key_down = move |e: KeyboardEvent| {
        let trigger_change_cloned_cloned = trigger_change_cloned.clone();
        let code = e.code();
        if code == *"ArrowLeft" {
            //todo maybe few same codes
            trigger_change_cloned_cloned((false, MouseOrKeyboardEvent::KeyboardEvent(e.clone())));
        } else if code == *"ArrowRight" {
            //todo maybe few same codes
            trigger_change_cloned_cloned((true, MouseOrKeyboardEvent::KeyboardEvent(e.clone())));
        }
        if let Some(on_key_down) = props.on_key_down.clone() {
            on_key_down.emit(e);
        }
    };
    let inner_checked_cloned = inner_checked.clone();
    let trigger_change_second_cloned = trigger_change;
    let on_internal_click = move |e: MouseEvent| {
        let trigger_change_second_cloned_cloned = trigger_change_second_cloned.clone();
        let trigger_change_input = (
            !*inner_checked_cloned.clone(),
            MouseOrKeyboardEvent::MouseEvent(e.clone()),
        );
        let ret = trigger_change_second_cloned_cloned(trigger_change_input);
        if let Some(on_click) = props.on_click.clone() {
            on_click.emit((ret, e));
        }
    };
    let on_internal_click_cloned = on_internal_click;
    let switch_class_name = match (*inner_checked, disabled) {
        (true, true) => format!(
            "ant-switch ant-switch-checked ant-switch-disabled {}",
            class
        ),
        (true, false) => format!("ant-switch ant-switch-checked {}", class),
        (false, true) => format!("ant-switch ant-switch-disabled {}", class),
        (false, false) => format!("ant-switch {}", class),
    };
    let inner_checked_third = *inner_checked;
    html! {
      <button
        tab_index={tab_index}
        style={style}
        title={title}
        // {...restProps} //some dynamic unhandled props
        type="button"
        role="switch"
        aria-checked={inner_checked_third.clone().to_string()}
        disabled={disabled}
        class={switch_class_name.clone()}
        ref={props.reference.clone()}
        onkeydown={on_internal_key_down}
        onclick={on_internal_click_cloned}
        ant-click-animating={"true"}
      >
        {loading_icon.clone()}
        <span class={String::from("ant-switch-inner")}>
        if *inner_checked {
            {checked_children.clone()}
        }
        else {
            {unchecked_children.clone()}
        }
        </span>
        <div class="ant-click-animating-node"></div>
      </button>
    }
}

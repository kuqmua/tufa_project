use crate::components::rc::rc_checkbox::types::InputType;
use crate::components::rc::rc_checkbox::types::RcCheckBoxProps;
// use gloo::console::log;
// use web_sys::Event;
use web_sys::FocusEvent;
use web_sys::KeyboardEvent;
use web_sys::MouseEvent;
use yew::function_component;
use yew::html;
// use yew::use_state;
use yew::Callback;

#[function_component(RcCheckBox)]
pub fn rc_checkbox(props: &RcCheckBoxProps) -> Html {
    let prefix_cls = match props.prefix_cls.clone() {
        None => String::from("rc-checkbox"),
        Some(pc) => pc,
    };
    let class_name = match props.class_name.clone() {
        None => String::from(""),
        Some(cn) => cn,
    };
    let style = match props.style.clone() {
        None => String::from(""),
        Some(s) => s,
    };
    let type_handle = match props.type_handle.clone() {
        None => InputType::Checkbox,
        Some(t) => t,
    };
    let title = match props.title.clone() {
        None => String::from(""),
        Some(t) => t,
    };
    let default_checked = props.default_checked.is_some();
    let checked = match props.checked {
        None => default_checked,
        Some(_) => true,
    };
    let name = match props.name.clone() {
        None => String::from(""),
        Some(n) => n,
    };
    let id = match props.id.clone() {
        None => String::from(""),
        Some(i) => i,
    };
    let required = props.required.clone().is_some();
    let read_only = props.read_only.clone().is_some();
    let disabled = props.disabled.clone().is_some();
    let tab_index = match props.tab_index {
        None => String::from(""),
        Some(ti) => ti.to_string(),
    };
    // let checked_state = use_state(|| checked);
    let on_focus = match props.on_focus.clone() {
        None => Callback::from(|_: FocusEvent| {}),
        Some(of) => of,
    };
    let on_blur = match props.on_blur.clone() {
        None => Callback::from(|_: FocusEvent| {}),
        Some(ob) => ob,
    };
    // let on_change = match props.on_change.clone() {
    //     None => Callback::from(|_: Event| {}),
    //     Some(oc) => oc,
    // };
    let on_click = match props.on_click.clone() {
        None => Callback::from(|_: MouseEvent| {}),
        Some(okd) => okd,
    };
    let on_key_down = match props.on_key_down.clone() {
        None => Callback::from(|_: KeyboardEvent| {}),
        Some(okd) => okd,
    };
    let on_key_press = match props.on_key_press.clone() {
        None => Callback::from(|_: KeyboardEvent| {}),
        Some(okp) => okp,
    };
    let on_key_up = match props.on_key_up.clone() {
        None => Callback::from(|_: KeyboardEvent| {}),
        Some(oku) => oku,
    };
    let value = match props.value.clone() {
        None => String::from(""),
        Some(v) => v,
    };
    let auto_focus = props.auto_focus.clone().is_some();
    // let focus = || {
    //   this.input.focus();
    // };
    // let blur = || {
    //   this.input.blur();
    // };
    // let checked_state_cloned = checked_state;
    // let disabled_cloned = props.disabled;
    // let on_change_cloned: Option<Callback<Event>> = props.on_change.clone();
    // let checked_cloned = props.checked;
    // let handle_change = move |e: Event| {
    //     match e.target() {
    //         None => (),
    //         Some(event_target) => {
    //             log!("checck", event_target);
    //         }
    //     };
    //     if disabled_cloned.is_some() {
    //         return;
    //     }
    //     if checked_cloned.is_none() {
    //         // checked_state_cloned.set(e.target.checked);
    //     }
    //     if let Some(on_change_handle) = on_change_cloned.clone() {
    //         //   onChange({
    //         //     target: {
    //         //       ...this.props,
    //         //       checked: e.target.checked,
    //         //     },
    //         //     stopPropagation() {
    //         //       e.stopPropagation();
    //         //     },
    //         //     preventDefault() {
    //         //       e.preventDefault();
    //         //     },
    //         //     nativeEvent: e.nativeEvent,
    //         //   });
    //     }
    // };
    // let save_input = |node| {
    //     this.input = node;
    // };
    //     const globalProps = Object.keys(others).reduce((prev, key) => {
    //       if (key.substr(0, 5) === 'aria-' || key.substr(0, 5) === 'data-' || key === 'role') {
    //         // eslint-disable-next-line no-param-reassign
    //         prev[key] = others[key];
    //       }
    //       return prev;
    //     }, {});
    let class_string = match (checked, disabled) {
        (true, true) => format!(
            "{} {} {}-checked {}-disabled",
            prefix_cls, class_name, prefix_cls, prefix_cls
        ),
        (true, false) => format!("{} {} {}-checked", prefix_cls, class_name, prefix_cls),
        (false, true) => format!("{} {} {}-disabled", prefix_cls, class_name, prefix_cls),
        (false, false) => format!("{} {}", prefix_cls, class_name),
    };
    html! {
      <span class={class_string} style={style}>
        <input
          name={name}
          id={id}
          type={format!("{}", type_handle)}
          title={title}
          required={required}
          readonly={read_only}
          disabled={disabled}
          tabIndex={tab_index}
          className={format!("{}-input", prefix_cls)}
          checked={checked}
          onclick={on_click}
          onfocus={on_focus}
          onblur={on_blur}
          onkeyup={on_key_up}
          onkeydown={on_key_down}
          onkeypress={on_key_press}
        //   onchange={handle_change}
          autofocus={auto_focus}
        //   ref={this.save_input}
          value={value}
        //   {...globalProps}
        />
        <span class={format!("{}-inner", prefix_cls)} />
      </span>
    }
}

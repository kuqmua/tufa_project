#[derive(Debug, PartialEq, Clone)]
pub struct Animation {
    pub appear: bool,
    pub enter: bool,
    pub leave: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AnimateProps {
    pub transition_name: bool,
    pub transition_appear: bool,
    pub animation: Animation,
    pub transition_enter: bool,
    pub transition_leave: bool,
}

pub fn is_appear_supported(props: AnimateProps) -> bool {
    return props.transition_name && props.transition_appear || props.animation.appear;
}
pub fn is_enter_supported(props: AnimateProps) -> bool {
    return props.transition_name && props.transition_enter || props.animation.enter;
}
pub fn is_leave_supported(props: AnimateProps) -> bool {
    return props.transition_name && props.transition_leave || props.animation.leave;
}
pub fn allow_appear_callback(props: AnimateProps) -> bool {
    return props.transition_appear || props.animation.appear;
}
pub fn allow_enter_callback(props: AnimateProps) -> bool {
    return props.transition_enter || props.animation.enter;
}
pub fn allow_eave_callback(props: AnimateProps) -> bool {
    return props.transition_leave || props.animation.leave;
}

use crate::components::ant_design::data_display::tooltip::placements::AutoAdjustOverflowHandle;
use crate::components::ant_design::data_display::tooltip::placements::PointsOffset;
use crate::components::ant_design::data_display::tooltip::placements::PointsValue;
use crate::components::ant_design::data_display::tooltip::placements::PositionType;
use crate::components::ant_design::navigation::dropdown;
use std::collections::HashMap;
use web_sys::FocusEvent;
use web_sys::KeyboardEvent;
use web_sys::MouseEvent;
use yew::function_component;
use yew::html;
use yew::use_state;
use yew::Callback;
use yew::Children;
use yew::Event;
use yew::Html;
use yew::NodeRef;
use yew::Properties;

// import * as React from 'react';
// import Trigger from 'rc-trigger';
// import type { AlignType } from 'rc-trigger/lib/interface';
// import classNames from 'classnames';
// import type { Placement, RenderDOMFunc } from './BaseSelect';

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DropdownMatchSelectWidth {
    //todo zero or one?
    Number(u8),
    Boolean(bool),
}

pub fn get_built_in_placements(
    dropdown_match_select_width: &DropdownMatchSelectWidth,
) -> HashMap<String, PointsOffset> {
    let adjust_x = match dropdown_match_select_width {
        DropdownMatchSelectWidth::Number(n) => *n,
        DropdownMatchSelectWidth::Boolean(b) => match b {
            true => 0,
            false => 1,
        },
    };
    HashMap::from([
        (
            PositionType::BottomLeft.get_string(),
            PointsOffset {
                points: [PointsValue::Tl, PointsValue::Bl],
                offset: [0, 4],
                overflow: Some(AutoAdjustOverflowHandle {
                    adjust_x,
                    adjust_y: 1,
                }),
                target_offset: None,
                ignore_shake: None,
            },
        ),
        (
            PositionType::BottomRight.get_string(),
            PointsOffset {
                points: [PointsValue::Tr, PointsValue::Br],
                offset: [0, 4],
                overflow: Some(AutoAdjustOverflowHandle {
                    adjust_x,
                    adjust_y: 1,
                }),
                target_offset: None,
                ignore_shake: None,
            },
        ),
        (
            PositionType::TopLeft.get_string(),
            PointsOffset {
                points: [PointsValue::Bl, PointsValue::Tl],
                offset: [0, -4],
                overflow: Some(AutoAdjustOverflowHandle {
                    adjust_x,
                    adjust_y: 1,
                }),
                target_offset: None,
                ignore_shake: None,
            },
        ),
        (
            PositionType::TopRight.get_string(),
            PointsOffset {
                points: [PointsValue::Br, PointsValue::Tr],
                offset: [0, -4],
                overflow: Some(AutoAdjustOverflowHandle {
                    adjust_x,
                    adjust_y: 1,
                }),
                target_offset: None,
                ignore_shake: None,
            },
        ),
    ])
}

#[derive(Debug, PartialEq, Clone)]
pub struct RefTriggerProps {
    pub get_popup_element: Callback<Html>, //() => HTMLDivElement
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Overflow {
    pub adjust_x: u8,
    pub adjust_y: u8,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AlignType {
    /**
     * move point of source node to align with point of target node.
     * Such as ['tr','cc'], align top right point of source node with center point of target node.
     * Point can be 't'(top), 'b'(bottom), 'c'(center), 'l'(left), 'r'(right) */
    pub points: Option<Vec<String>>,
    /**
     * offset source node by offset[0] in x and offset[1] in y.
     * If offset contains percentage string value, it is relative to sourceNode region.
     */
    pub offset: Option<Vec<i32>>,
    /**
     * offset target node by offset[0] in x and offset[1] in y.
     * If targetOffset contains percentage string value, it is relative to targetNode region.
     */
    pub target_offset: Option<Vec<i32>>,
    /**
     * If adjustX field is true, will adjust source node in x direction if source node is invisible.
     * If adjustY field is true, will adjust source node in y direction if source node is invisible.
     */
    pub overflow: Option<Overflow>,
    /**
     * Whether use css right instead of left to position
     */
    pub use_css_right: Option<bool>,
    /**
     * Whether use css bottom instead of top to position
     */
    pub use_css_bottom: Option<bool>,
    /**
     * Whether use css transform instead of left/top/right/bottom to position if browser supports.
     * Defaults to false.
     */
    pub use_css_transform: Option<bool>,
    pub ignore_shake: Option<bool>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SelectTriggerProps {
    pub prefix_cls: String,
    pub children: Children,
    pub disabled: Option<()>,
    pub visible: Option<()>,
    pub popup_element: Html, //React.ReactElement
    pub animation: Option<String>,
    pub transition_name: Option<String>,
    pub container_width: i32,
    //   pub placement?: Option<Placement>,
    pub dropdown_style: String, //React.CSSProperties,
    pub dropdown_class_name: String,
    pub direction: String,
    pub dropdown_match_select_width: Option<DropdownMatchSelectWidth>,
    pub dropdown_render: Option<Callback<Html>>, //(menu: React.ReactElement) => React.ReactElement
    pub get_popup_container: Option<Callback<()>>, //RenderDOMFunc
    pub dropdown_align: AlignType,
    pub empty: Option<()>,
    pub get_trigger_dom_node: Callback<()>, //() => HTMLElement
    pub on_popup_visible_change: Option<Callback<bool>>, //(visible: boolean) => void
    pub on_popup_mouse_enter: Callback<()>,
}

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct RefTriggerPropsAndSelectTriggerProps {
    pub get_popup_element: Callback<Html>, //() => HTMLDivElement
    //
    pub prefix_cls: String,
    pub children: Children,
    pub disabled: Option<()>,
    pub visible: Option<()>,
    pub popup_element: Html, //React.ReactElement
    pub animation: Option<String>,
    pub transition_name: Option<String>,
    pub container_width: i32,
    //   pub placement?: Option<Placement>,
    pub dropdown_style: String, //React.CSSProperties,
    pub dropdown_class_name: String,
    pub direction: String,
    pub dropdown_match_select_width: Option<DropdownMatchSelectWidth>,
    // pub dropdown_render: Option<Callback<Html>>, //(menu: React.ReactElement) => React.ReactElement //no need
    pub get_popup_container: Option<Callback<()>>, //RenderDOMFunc
    pub dropdown_align: AlignType,
    pub empty: Option<()>,
    pub get_trigger_dom_node: Callback<()>, //() => HTMLElement
    pub on_popup_visible_change: Option<Callback<bool>>, //(visible: boolean) => void
    pub on_popup_mouse_enter: Callback<()>,
    //
    pub reference: NodeRef,
}

#[function_component(SelectTrigger)]
pub fn select_trigger(props: &RefTriggerPropsAndSelectTriggerProps) -> Html {
    // let prefix_cls = match props.prefix_cls.clone() {
    //     None => String::from("ant-checkbox"),
    //     Some(pc) => pc,
    // };
    let dropdown_prefix_cls = format!("{}-dropdown", props.prefix_cls);
    let mut popup_node = props.popup_element.clone();
    // if let Some(dropdown_render) = props.dropdown_render.clone() {
    //     popup_node = dropdown_render(props.popup_element.clone());
    // }

    //   const builtInPlacements = React.useMemo(
    //     () => getBuiltInPlacements(dropdownMatchSelectWidth),
    //     [dropdownMatchSelectWidth],
    //   );

    //   // ===================== Motion ======================
    let merged_transition_name = match props.animation.clone() {
        Some(a) => format!("{}-{}", dropdown_prefix_cls, a),
        None => match props.transition_name.clone() {
            Some(tn) => tn,
            None => String::from(""),
        },
    };
    let merged_transition_name = match props.animation.clone() {
        None => match props.transition_name.clone() {
            Some(tn) => tn,
            None => String::from(""),
        },
        Some(a) => format!("{}-{}", dropdown_prefix_cls, a),
    };
    //   const mergedTransitionName = animation ? `${dropdownPrefixCls}-${animation}` : transitionName;

    //   // ======================= Ref =======================
    //   const popupRef = React.useRef<HTMLDivElement>(null);

    //   React.useImperativeHandle(ref, () => ({
    //     getPopupElement: () => popupRef.current,
    //   }));
    let mut popup_style = format!(
        "min-width: {}; {}",
        props.container_width.clone(),
        props.dropdown_style.clone()
    );
    //   const popupStyle: React.CSSProperties = {
    //     minWidth: containerWidth,
    //     ...dropdownStyle,
    //   };
    match props.dropdown_match_select_width.clone() {
        None => (),
        Some(dmsw) => match dmsw {
            DropdownMatchSelectWidth::Number(u) => {
                popup_style = format!("width: {}; {}", u, popup_style);
            }
            DropdownMatchSelectWidth::Boolean(_) => {
                popup_style = format!("width: {}; {}", props.container_width.clone(), popup_style);
            }
        },
    }

    //   if (typeof dropdownMatchSelectWidth === 'number') {
    //     popupStyle.width = dropdownMatchSelectWidth;
    //   } else if (dropdownMatchSelectWidth) {
    //     popupStyle.width = containerWidth;
    //   }
    html! {
            //     <Trigger
    //       {...restProps}
    //       showAction={onPopupVisibleChange ? ['click'] : []}
    //       hideAction={onPopupVisibleChange ? ['click'] : []}
    //       popupPlacement={placement || (direction === 'rtl' ? 'bottomRight' : 'bottomLeft')}
    //       builtinPlacements={builtInPlacements}
    //       prefixCls={dropdownPrefixCls}
    //       popupTransitionName={mergedTransitionName}
    //       popup={
    //         <div ref={popupRef} onMouseEnter={onPopupMouseEnter}>
    //           {popupNode}
    //         </div>
    //       }
    //       popupAlign={dropdownAlign}
    //       popupVisible={visible}
    //       getPopupContainer={getPopupContainer}
    //       popupClassName={classNames(dropdownClassName, {
    //         [`${dropdownPrefixCls}-empty`]: empty,
    //       })}
    //       popupStyle={popupStyle}
    //       getTriggerDOMNode={getTriggerDOMNode}
    //       onPopupVisibleChange={onPopupVisibleChange}
    //     >
    //       {children}
    //     </Trigger>
        }
}

// const SelectTrigger: React.RefForwardingComponent<RefTriggerProps, SelectTriggerProps> = (
//   props,
//   ref,
// ) => {
//   const {
//     prefixCls,
//     disabled,
//     visible,
//     children,
//     popupElement,
//     containerWidth,
//     animation,
//     transitionName,
//     dropdownStyle,
//     dropdownClassName,
//     direction = 'ltr',
//     placement,
//     dropdownMatchSelectWidth,
//     dropdownRender,
//     dropdownAlign,
//     getPopupContainer,
//     empty,
//     getTriggerDOMNode,
//     onPopupVisibleChange,
//     onPopupMouseEnter,
//     ...restProps
//   } = props;

//   const dropdownPrefixCls = `${prefixCls}-dropdown`;

//   let popupNode = popupElement;
//   if (dropdownRender) {
//     popupNode = dropdownRender(popupElement);
//   }

//   const builtInPlacements = React.useMemo(
//     () => getBuiltInPlacements(dropdownMatchSelectWidth),
//     [dropdownMatchSelectWidth],
//   );

//   // ===================== Motion ======================
//   const mergedTransitionName = animation ? `${dropdownPrefixCls}-${animation}` : transitionName;

//   // ======================= Ref =======================
//   const popupRef = React.useRef<HTMLDivElement>(null);

//   React.useImperativeHandle(ref, () => ({
//     getPopupElement: () => popupRef.current,
//   }));

//   const popupStyle: React.CSSProperties = {
//     minWidth: containerWidth,
//     ...dropdownStyle,
//   };

//   if (typeof dropdownMatchSelectWidth === 'number') {
//     popupStyle.width = dropdownMatchSelectWidth;
//   } else if (dropdownMatchSelectWidth) {
//     popupStyle.width = containerWidth;
//   }

//   return (
//     <Trigger
//       {...restProps}
//       showAction={onPopupVisibleChange ? ['click'] : []}
//       hideAction={onPopupVisibleChange ? ['click'] : []}
//       popupPlacement={placement || (direction === 'rtl' ? 'bottomRight' : 'bottomLeft')}
//       builtinPlacements={builtInPlacements}
//       prefixCls={dropdownPrefixCls}
//       popupTransitionName={mergedTransitionName}
//       popup={
//         <div ref={popupRef} onMouseEnter={onPopupMouseEnter}>
//           {popupNode}
//         </div>
//       }
//       popupAlign={dropdownAlign}
//       popupVisible={visible}
//       getPopupContainer={getPopupContainer}
//       popupClassName={classNames(dropdownClassName, {
//         [`${dropdownPrefixCls}-empty`]: empty,
//       })}
//       popupStyle={popupStyle}
//       getTriggerDOMNode={getTriggerDOMNode}
//       onPopupVisibleChange={onPopupVisibleChange}
//     >
//       {children}
//     </Trigger>
//   );
// };

// const RefSelectTrigger = React.forwardRef<RefTriggerProps, SelectTriggerProps>(SelectTrigger);
// RefSelectTrigger.displayName = 'SelectTrigger';

// export default RefSelectTrigger;

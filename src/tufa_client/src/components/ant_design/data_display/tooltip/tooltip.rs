use crate::components::ant_design::data_display::tooltip::placements::get_placements;
use crate::components::ant_design::general::button::Button;
use crate::components::ant_design::helpers::offset::Offset;
// use crate::components::ant_design::tooltip::placements::AdjustOverflow;
use crate::components::ant_design::data_display::tooltip::placements::AdjustOverflowOrBool;
use crate::components::ant_design::data_display::tooltip::placements::PlacementsConfig;
use crate::helpers::pseudo_css_wrapper::PseudoCssWrapper;
use std::collections::HashMap;
use yew::use_state;
use yew::{function_component, html, Callback, Children, Html, Properties};

// import * as React from 'react';
// import { polyfill } from 'react-lifecycles-compat';
// import RcTooltip from 'rc-tooltip';
// import classNames from 'classnames';
// import getPlacements, { AdjustOverflow, PlacementsConfig } from './placements';
// import { ConfigConsumer, ConfigConsumerProps } from '../config-provider';

// export { AdjustOverflow, PlacementsConfig };

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TooltipPlacement {
    Top,
    Left,
    Right,
    Bottom,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    LeftTop,
    LeftBottom,
    RightTop,
    RightBottom,
}

// export type TooltipPlacement =
//   | 'top'
//   | 'left'
//   | 'right'
//   | 'bottom'
//   | 'topLeft'
//   | 'topRight'
//   | 'bottomLeft'
//   | 'bottomRight'
//   | 'leftTop'
//   | 'leftBottom'
//   | 'rightTop'
//   | 'rightBottom';

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Overflow {
    pub adjust_x: bool,
    pub adjust_y: bool,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TooltipAlignConfig {
    pub points: Option<(String, String)>,
    pub offset: Option<Offset>,
    pub target_offset: Option<Offset>,
    pub overflow: Option<Overflow>,
    pub use_css_right: Option<()>,
    pub use_css_bottom: Option<()>,
    pub use_css_transform: Option<()>,
}

// // https://github.com/react-component/tooltip
// // https://github.com/yiminghe/dom-align
// export interface TooltipAlignConfig {
//   points?: [string, string];
//   offset?: [number | string, number | string];
//   targetOffset?: [number | string, number | string];
//   overflow?: { adjustX: boolean; adjustY: boolean };
//   useCssRight?: boolean;
//   useCssBottom?: boolean;
//   useCssTransform?: boolean;
// }

#[derive(Debug, PartialEq, Clone)]
pub struct AbstractTooltipProps {
    //not full
    pub style: Option<String>, // React.CSSProperties;
    pub class_name: Option<String>,
    pub color: Option<()>, // LiteralUnion<PresetColorType, string>;
    pub placement: Option<TooltipPlacement>,
    pub builtin_placements: Option<()>, // typeof Placements;
    pub open_class_name: Option<String>,
    pub arrow_point_at_center: Option<()>,
    pub auto_adjust_overflow: Option<AdjustOverflowOrBool>,
    pub get_popup_container: Option<Callback<()>>, // (triggerNode: HTMLElement) => HTMLElement;
    pub children: Children,
}

// export interface AbstractTooltipProps extends Partial<Omit<RcTooltipProps, 'children'>> {
//   style?: React.CSSProperties;
//   className?: string;
//   color?: LiteralUnion<PresetColorType, string>;
//   placement?: TooltipPlacement;
//   builtinPlacements?: typeof Placements;
//   openClassName?: string;
//   arrowPointAtCenter?: boolean;
//   autoAdjustOverflow?: boolean | AdjustOverflow;
//   getPopupContainer?: (triggerNode: HTMLElement) => HTMLElement;
//   children?: React.ReactNode;
// }

pub type RenderFunction = fn() -> Html;
// export type RenderFunction = () => React.ReactNode;

#[derive(Debug, PartialEq, Clone)]
pub enum AbstractTooltipPropsContent {
    ReactNode(Html),
    RenderFunction(RenderFunction),
}

#[derive(Debug, PartialEq, Clone)]
pub struct TooltipPropsWithOverlay {
    //not full
    pub style: Option<String>, // React.CSSProperties;
    pub class_name: Option<String>,
    pub color: Option<()>, // LiteralUnion<PresetColorType, string>;
    pub placement: Option<TooltipPlacement>,
    pub builtin_placements: Option<HashMap<String, PointsOffset>>, // typeof Placements;//object
    pub open_class_name: Option<String>,
    pub arrow_point_at_center: Option<()>,
    pub auto_adjust_overflow: Option<AdjustOverflowOrBool>,
    pub get_popup_container: Option<Callback<()>>, // (triggerNode: HTMLElement) => HTMLElement;
    pub children: Children,

    pub title: Option<AbstractTooltipPropsContent>,
    pub overlay: AbstractTooltipPropsContent,
    pub visible: Option<()>,
    pub on_visible_change: Option<fn(bool)>, //Callback<bool>
}

// export interface TooltipPropsWithOverlay extends AbstractTooltipProps {
//   title?: React.ReactNode | RenderFunction;
//   overlay: React.ReactNode | RenderFunction;
// }

#[derive(Debug, PartialEq, Clone)]
pub struct TooltipPropsWithTitle {
    //not full
    pub style: Option<String>, // React.CSSProperties;
    pub class_name: Option<String>,
    pub color: Option<()>, // LiteralUnion<PresetColorType, string>;
    pub placement: Option<TooltipPlacement>,
    pub builtin_placements: Option<HashMap<String, PointsOffset>>, // typeof Placements;//object
    pub open_class_name: Option<String>,
    pub arrow_point_at_center: Option<()>,
    pub auto_adjust_overflow: Option<AdjustOverflowOrBool>,
    pub get_popup_container: Option<Callback<()>>, // (triggerNode: HTMLElement) => HTMLElement;
    pub children: Children,

    pub title: AbstractTooltipPropsContent,
    pub overlay: Option<AbstractTooltipPropsContent>,
    pub visible: Option<()>,
    pub on_visible_change: Option<fn(bool)>, //Callback<bool>
}

// export interface TooltipPropsWithTitle extends AbstractTooltipProps {
//   title: React.ReactNode | RenderFunction;
//   overlay?: React.ReactNode | RenderFunction;
// }

#[derive(Debug, PartialEq, Clone)]
pub enum TooltipProps {
    WithTitle(TooltipPropsWithTitle),
    WithOverlay(TooltipPropsWithOverlay),
}

// export declare type TooltipProps = TooltipPropsWithTitle | TooltipPropsWithOverlay;

#[derive(Debug, PartialEq, Clone)]
pub struct SplittedObject {
    pub picked: PseudoCssWrapper,
    pub omitted: PseudoCssWrapper,
}

pub fn split_object(element: ElementType, omitted_keys_array: Vec<&str>) -> SplittedObject {
    let mut picked = PseudoCssWrapper {
        style: HashMap::<String, String>::new(),
    };
    let mut omitted = PseudoCssWrapper {
        style: HashMap::<String, String>::new(),
    };
    match element.clone().get_style_option() {
        None => (),
        Some(pseudo_css_wraper) => {
            for (style_key, style_value) in pseudo_css_wraper.style.clone() {
                for ommited_key in &omitted_keys_array {
                    let k = style_key.clone();
                    let v = style_value.clone();
                    match style_key == ommited_key.to_string() {
                        true => {
                            omitted.style.insert(k, v);
                        }
                        false => {
                            picked.style.insert(k, v);
                        }
                    }
                }
            }
        }
    }
    SplittedObject { picked, omitted }
}

// const splitObject = (obj: any, keys: string[]) => {
//   const picked: any = {};
//   const omitted: any = { ...obj };
//   keys.forEach(key => {
//     if (obj && key in obj) {
//       picked[key] = obj[key];
//       delete omitted[key];
//     }
//   });
//   return { picked, omitted };
// };
// const PresetColorRegex = new RegExp(`^(${PresetColorTypes.join('|')})(-inverse)?$`);

use crate::components::ant_design::general::button::ButtonProps;

use super::placements::PointsOffset;

#[derive(Debug, PartialEq, Clone)]
pub enum ElementType {
    Button(ButtonProps),
    // Switch(SwitchProps),
    // Checkbox(ChechboxProps),
    OtherDisabledCompatibleChildren(Html),
}

impl ElementType {
    pub fn get_style_option(&self) -> Option<PseudoCssWrapper> {
        match self {
            ElementType::Button(props) => props.style.clone(),
            // ElementType::Switch(props) => props.style.clone(),
            // ElementType::Checkbox(props) => props.style.clone(),
            ElementType::OtherDisabledCompatibleChildren(_) => None, //todo //props.style.clone()
        }
    }
}

pub fn get_disabled_compatible_children(element_type: ElementType, prefix_cls: String) -> Html {
    //Html //ElementType
    match element_type.clone() {
        ElementType::Button(props) => match props.disabled {
            None => html! {
              <Button
                disabled={props.disabled}
                ghost={props.ghost}
                href={props.href.clone()}
                html_type={props.html_type.clone()}
                icon={props.icon.clone()}
                loading={props.loading.clone()}
                shape={props.shape.clone()}
                size={props.size.clone()}
                target={props.target.clone()}
                button_type={props.button_type.clone()}
                on_click={props.on_click.clone()}
                block={props.block.clone()}
                placeholder={props.placeholder.clone()}
                style={props.style.clone()}
              />
            },
            Some(_) => {
                let overrided_props = props.clone();
                let block = overrided_props.block;
                let width = match block {
                    None => String::from("null"),
                    Some(_) => String::from("100%"),
                };
                let omitted_str_array = vec![
                    "position", "left", "right", "top", "bottom", "float", "display", "zIndex",
                ];
                // Pick some layout related style properties up to span
                // Prevent layout bugs like https://github.com/ant-design/ant-design/issues/5254
                // const { picked, omitted } = splitObject(element.props.style, [
                //   'position',
                //   'left',
                //   'right',
                //   'top',
                //   'bottom',
                //   'float',
                //   'display',
                //   'zIndex',
                // ]);
                let splitted_object = split_object(
                    ElementType::Button(overrided_props.clone()),
                    omitted_str_array,
                );
                // const splitObject = (obj: any, keys: string[]) => {
                //   const picked: any = {};
                //   const omitted: any = { ...obj };
                //   keys.forEach(key => {
                //     if (obj && key in obj) {
                //       picked[key] = obj[key];
                //       delete omitted[key];
                //     }
                //   });
                //   return { picked, omitted };
                // };
                let mut span_style = splitted_object.picked;
                span_style
                    .style
                    .insert(String::from("display"), String::from("inline-block"));
                span_style
                    .style
                    .insert(String::from("cursor"), String::from("not-allowed"));
                span_style
                    .style
                    .insert(String::from("width"), width.to_string());
                let mut button_style = splitted_object.omitted;
                button_style
                    .style
                    .insert(String::from("pointerEvents"), String::from("none"));
                // const buttonStyle = {
                //   ...omitted,
                //   pointerEvents: 'none',
                // };
                let child = html! {
                  <Button
                    disabled={props.disabled}
                    ghost={props.ghost}
                    href={props.href.clone()}
                    html_type={props.html_type.clone()}
                    icon={props.icon.clone()}
                    loading={props.loading.clone()}
                    shape={props.shape.clone()}
                    size={props.size.clone()}
                    target={props.target.clone()}
                    button_type={props.button_type.clone()}
                    on_click={props.on_click.clone()}
                    block={props.block.clone()}
                    placeholder={props.placeholder.clone()}
                    style={Some(button_style)}
                    //className: null,
                  />
                };
                let class = format!("{}-disabled-compatible-wrapper", prefix_cls); //todo //classNames(element.props.className, `${prefixCls}-disabled-compatible-wrapper`)
                let style = match overrided_props.style {
                    None => String::from(""),
                    Some(pseudo_css_wrapper) => pseudo_css_wrapper.to_string(),
                };
                html! {
                    <span
                      style={style}
                      class={class}
                    >
                      {child}
                    </span>
                }
            }
        },
        // ElementType::Switch(props) => match props.disabled {
        //     None => false,
        //     Some(_) => html!{
        //         <Switch

        //         />
        //       },
        // },
        // ElementType::Checkbox(props) => match props.disabled {
        //     None => false,
        //     Some(_) => html!{
        //         <Checkbox

        //         />
        //       },
        // },
        ElementType::OtherDisabledCompatibleChildren(html) => html,
    }
}

// // Fix Tooltip won't hide at disabled button
// // mouse events don't trigger at disabled button in Chrome
// // https://github.com/react-component/tooltip/issues/18
// function getDisabledCompatibleChildren(element: React.ReactElement<any>, prefixCls: string) {
//   const elementType = element.type as any;
//   if (
//     ((elementType.__ANT_BUTTON === true || element.type === 'button') && element.props.disabled) ||
//     (elementType.__ANT_SWITCH === true && (element.props.disabled || element.props.loading))
//   ) {
//     // Pick some layout related style properties up to span
//     // Prevent layout bugs like https://github.com/ant-design/ant-design/issues/5254
//     const { picked, omitted } = splitObject(element.props.style, [
//       'position',
//       'left',
//       'right',
//       'top',
//       'bottom',
//       'float',
//       'display',
//       'zIndex',
//     ]);
//     const spanStyle = {
//       display: 'inline-block', // default inline-block is important
//       ...picked,
//       cursor: 'not-allowed',
//       width: element.props.block ? '100%' : null,
//     };
//     const buttonStyle = {
//       ...omitted,
//       pointerEvents: 'none',
//     };
//     const child = cloneElement(element, {
//       style: buttonStyle,
//       className: null,
//     });
//     return (
//       <span
//         style={spanStyle}
//         className={classNames(element.props.className, `${prefixCls}-disabled-compatible-wrapper`)}
//       >
//         {child}
//       </span>
//     );
//   }
//   return element;
// }

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct TooltipPropsStruct {
    pub tooltip_props: TooltipProps,
}

#[derive(Debug, PartialEq, Clone)]
pub enum GetPlacementsTooltipValue {
    BuiltinPlacements(HashMap<String, PointsOffset>), //todo here some object
    GetPlacements(HashMap<String, PointsOffset>),
}

#[function_component(Tooltip)]
pub fn tooltip(props: &TooltipPropsStruct) -> Html {
    // class Tooltip extends React.Component<TooltipProps, any> {
    // let default_placement = TooltipPlacement::Top;
    // let default_transition_name = "zoom-big-fast";
    // let default_mouseEnterDelay = 0.1;
    // let default_mouseLeaveDelay = 0.1;
    // let default_arrowPointAtCenter = false;
    // let default_autoAdjustOverflow = true;

    let visible_state = use_state(|| false);
    let visible_state_cloned_first = visible_state;
    //todo props.defaultVisible
    match props.tooltip_props.clone() {
        TooltipProps::WithTitle(with_title) => match with_title.visible {
            Some(_) => visible_state_cloned_first.set(true),
            None => visible_state_cloned_first.set(false),
        },
        TooltipProps::WithOverlay(with_overlay) => match with_overlay.visible {
            Some(_) => visible_state_cloned_first.set(true),
            None => visible_state_cloned_first.set(false),
        },
    };
    //did not understand why this function here. its not used anythere../ ignored for now
    //     static getDerivedStateFromProps(nextProps: TooltipProps) {
    //       if ('visible' in nextProps) {
    //         return { visible: nextProps.visible };
    //       }
    //       return null;
    //     }

    //     private tooltip: typeof RcTooltip;

    //     constructor(props: TooltipProps) {
    //       super(props);

    //       this.state = {
    //         visible: !!props.visible || !!props.defaultVisible,
    //       };
    //     }
    let visible_prop = match props.tooltip_props.clone() {
        TooltipProps::WithTitle(props) => props.visible.clone(),
        TooltipProps::WithOverlay(props) => props.visible.clone(),
    };
    let on_visible_change_option_prop = match props.tooltip_props.clone() {
        TooltipProps::WithTitle(props) => props.on_visible_change.clone(),
        TooltipProps::WithOverlay(props) => props.on_visible_change.clone(),
    };
    let _on_visible_change = |visible: bool| {
        if let None = visible_prop {
            // this.setState({ visible: this.isNoTitle() ? false : visible });
        }
        let f = false;
        match (on_visible_change_option_prop, f) {
            //&& this.isNoTitle()
            (None, true) => (),
            (None, false) => (),
            (Some(_), true) => (),
            (Some(on_visible_change), false) => on_visible_change(visible),
        }
    };

    //     onVisibleChange = (visible: boolean) => {
    //       const { onVisibleChange } = this.props;
    //       if (!('visible' in this.props)) {
    //         this.setState({ visible: this.isNoTitle() ? false : visible });
    //       }
    //       if (onVisibleChange && !this.isNoTitle()) {
    //         onVisibleChange(visible);
    //       }
    //     };

    //     getPopupDomNode() {
    //       return this.tooltip.getPopupDomNode();
    //     }
    let builtin_placements = match props.tooltip_props.clone() {
        TooltipProps::WithTitle(props) => props.builtin_placements.clone(),
        TooltipProps::WithOverlay(props) => props.builtin_placements.clone(),
    };
    let arrow_point_at_center = match props.tooltip_props.clone() {
        TooltipProps::WithTitle(props) => props.arrow_point_at_center.clone(),
        TooltipProps::WithOverlay(props) => props.arrow_point_at_center.clone(),
    };
    let auto_adjust_overflow = match props.tooltip_props.clone() {
        TooltipProps::WithTitle(props) => props.auto_adjust_overflow.clone(),
        TooltipProps::WithOverlay(props) => props.auto_adjust_overflow.clone(),
    };
    let get_placements_tooltip = || -> GetPlacementsTooltipValue {
        match builtin_placements {
            Some(hs) => GetPlacementsTooltipValue::BuiltinPlacements(hs),
            None => {
                GetPlacementsTooltipValue::GetPlacements(get_placements(Some(PlacementsConfig {
                    arrow_width: None,
                    horizontal_arrow_shift: None,
                    vertical_arrow_shift: Some(8),
                    arrow_point_at_center: Some(arrow_point_at_center.is_some()),
                    auto_adjust_overflow: auto_adjust_overflow,
                })))
            }
        }
    };

    //     getPlacements() {
    //       const { builtinPlacements, arrowPointAtCenter, autoAdjustOverflow } = this.props;
    //       return (
    //         builtinPlacements ||
    //         getPlacements({
    //           arrowPointAtCenter,
    //           verticalArrowShift: 8,
    //           autoAdjustOverflow,
    //         })
    //       );
    //     }

    //     saveTooltip = (node: typeof RcTooltip) => {
    //       this.tooltip = node;
    //     };

    let _on_popup_align = |_dom_node: Html, align: PointsOffset| {
        //, align: any
        let placements = get_placements_tooltip();
        let points_placements_hashmap = match placements {
            GetPlacementsTooltipValue::BuiltinPlacements(hs) => hs,
            GetPlacementsTooltipValue::GetPlacements(hs) => hs,
        };
        let mut placement: Option<PointsOffset> = None;
        for (_key, value) in points_placements_hashmap {
            if align.points[0] == value.points[0] && align.points[1] == value.points[1] {
                placement = Some(value);
            }
        }
        if placement.is_none() {
            return;
        }
        //   const placement = Object.keys(placements).filter(
        //     key =>
        //       placements[key].points[0] === align.points[0] &&
        //       placements[key].points[1] === align.points[1],
        //   )[0];
        //   if (!placement) {
        //     return;
        //   }
        //   // 根据当前坐标设置动画点
        //   const rect = domNode.getBoundingClientRect();
        // let transform_origin_top = "50%";
        // let transform_origin_left = "50%";
        //   const transformOrigin = {
        //     top: '50%',
        //     left: '50%',
        //   };
        //   if (placement.indexOf('top') >= 0 || placement.indexOf('Bottom') >= 0) {
        //     transformOrigin.top = `${rect.height - align.offset[1]}px`;
        //   } else if (placement.indexOf('Top') >= 0 || placement.indexOf('bottom') >= 0) {
        //     transformOrigin.top = `${-align.offset[1]}px`;
        //   }
        //   if (placement.indexOf('left') >= 0 || placement.indexOf('Right') >= 0) {
        //     transformOrigin.left = `${rect.width - align.offset[0]}px`;
        //   } else if (placement.indexOf('right') >= 0 || placement.indexOf('Left') >= 0) {
        //     transformOrigin.left = `${-align.offset[0]}px`;
        //   }
        //   domNode.style.transformOrigin = `${transformOrigin.left} ${transformOrigin.top}`;
    };

    ///////////////////////////////////

    //     onPopupAlign = (domNode: HTMLElement, align: any) => {
    //       const placements: any = this.getPlacements();
    //       // 当前返回的位置
    //       const placement = Object.keys(placements).filter(
    //         key =>
    //           placements[key].points[0] === align.points[0] &&
    //           placements[key].points[1] === align.points[1],
    //       )[0];
    //       if (!placement) {
    //         return;
    //       }
    //       // 根据当前坐标设置动画点
    //       const rect = domNode.getBoundingClientRect();
    //       const transformOrigin = {
    //         top: '50%',
    //         left: '50%',
    //       };
    //       if (placement.indexOf('top') >= 0 || placement.indexOf('Bottom') >= 0) {
    //         transformOrigin.top = `${rect.height - align.offset[1]}px`;
    //       } else if (placement.indexOf('Top') >= 0 || placement.indexOf('bottom') >= 0) {
    //         transformOrigin.top = `${-align.offset[1]}px`;
    //       }
    //       if (placement.indexOf('left') >= 0 || placement.indexOf('Right') >= 0) {
    //         transformOrigin.left = `${rect.width - align.offset[0]}px`;
    //       } else if (placement.indexOf('right') >= 0 || placement.indexOf('Left') >= 0) {
    //         transformOrigin.left = `${-align.offset[0]}px`;
    //       }
    //       domNode.style.transformOrigin = `${transformOrigin.left} ${transformOrigin.top}`;
    //     };

    //     isNoTitle() {
    //       const { title, overlay } = this.props;
    //       return !title && !overlay && title !== 0; // overlay for old version compatibility
    //     }

    //     getOverlay() {
    //       const { title, overlay } = this.props;
    //       if (title === 0) {
    //         return title;
    //       }
    //       return overlay || title || '';
    //     }

    //     renderTooltip = ({
    //       getPopupContainer: getContextPopupContainer,
    //       getPrefixCls,
    //     }: ConfigConsumerProps) => {
    //       const { props, state } = this;
    //       const {
    //         prefixCls: customizePrefixCls,
    //         openClassName,
    //         getPopupContainer,
    //         getTooltipContainer,
    //       } = props;
    //       const children = props.children as React.ReactElement<any>;
    //       const prefixCls = getPrefixCls('tooltip', customizePrefixCls);
    //       let { visible } = state;
    //       // Hide tooltip when there is no title
    //       if (!('visible' in props) && this.isNoTitle()) {
    //         visible = false;
    //       }

    //       const child = getDisabledCompatibleChildren(
    //         React.isValidElement(children) ? children : <span>{children}</span>,
    //       );

    //       const childProps = child.props;
    //       const childCls = classNames(childProps.className, {
    //         [openClassName || `${prefixCls}-open`]: true,
    //       });

    //       return (
    //         <RcTooltip
    //           {...this.props}
    //           prefixCls={prefixCls}
    //           getTooltipContainer={getPopupContainer || getTooltipContainer || getContextPopupContainer}
    //           ref={this.saveTooltip}
    //           builtinPlacements={this.getPlacements()}
    //           overlay={this.getOverlay()}
    //           visible={visible}
    //           onVisibleChange={this.onVisibleChange}
    //           onPopupAlign={this.onPopupAlign}
    //         >
    //           {visible ? React.cloneElement(child, { className: childCls }) : child}
    //         </RcTooltip>
    //       );
    //     };

    //     render() {
    //       return <ConfigConsumer>{this.renderTooltip}</ConfigConsumer>;
    //     }
    //   }
    html! {}
}

// polyfill(Tooltip);

// export default Tooltip;

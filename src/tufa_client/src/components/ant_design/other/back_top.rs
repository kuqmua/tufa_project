// import VerticalAlignTopOutlined from '@ant-design/icons/VerticalAlignTopOutlined';
// import classNames from 'classnames';
// import CSSMotion from 'rc-motion';
// import addEventListener from 'rc-util/lib/Dom/addEventListener';
// import useMergedState from 'rc-util/lib/hooks/useMergedState';
// import omit from 'rc-util/lib/omit';
// import * as React from 'react';
// import { ConfigContext } from '../config-provider';
// import getScroll from '../_util/getScroll';
// import { cloneElement } from '../_util/reactNode';
// import scrollTo from '../_util/scrollTo';
// import { throttleByAnimationFrame } from '../_util/throttleByAnimationFrame';

// use colorsys::Hsl;
// use gloo::console::error;
// use tufa_common::helpers::numeric::Numeric;
use yew::{function_component, html, Properties};
// export interface BackTopProps {
//   visibilityHeight?: number;
//   onClick?: React.MouseEventHandler<HTMLElement>;
//   target?: () => HTMLElement | Window | Document;
//   prefixCls?: string;
//   children?: React.ReactNode;
//   className?: string;
//   style?: React.CSSProperties;
//   duration?: number;
//   visible?: boolean; // Only for test. Don't use it.
// }
#[derive(Properties, PartialEq)]
pub struct BackTopProps {
    // pub visibility_height: Option<u32>,
    // pub on_click: Option<Callback<MouseEvent>>,
    // // pub target?: () => HTMLElement | Window | Document;
    // pub prefix_cls: Option<String>,
    // pub children: Children,
    // pub class_name: Option<String>,
    // pub style: Option<String>,
    // pub duration: Option<u32>,
    // pub visible: Option<()>, // Only for test. Don't use it.
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SetVisible {
    pub value: bool,
}

#[function_component(BackTop)]
pub fn back_top(_props: &BackTopProps) -> Html {
    // let visibility_height =  props.visibility_height.unwrap_or(400);

    // let visible = use_state(|| false);
    // let setVisible = use_state(|| SetVisible { value: props.visible.is_some().clone()});

    // let render_children = |prefix_cls_handle: String, root_prefix_cls_handle: String| {
    //     let default_element = html!{
    //       <div className={format!("{}-content", prefix_cls_handle)}>
    //         <div className={format!("{}-icon", root_prefix_cls_handle)}>
    //         //   <VerticalAlignTopOutlined />
    //         </div>
    //       </div>
    //     };
    //     html!{
    //     //   <CSSMotion visible={visible} motionName={`${rootPrefixCls}-fade`}>
    //     //     {({ className: motionClassName }) =>
    //     //       cloneElement(children || defaultElement, ({ className }) => ({
    //     //         className: classNames(motionClassName, className),
    //     //       }))
    //     //     }
    //     //   </CSSMotion>
    //     }
    // };
    // let divProps = omit(props, [
    //     'prefixCls',
    //     'className',
    //     'children',
    //     'visibilityHeight',
    //     'target',
    //     'visible',
    // ]);
    // pub on_click: Option<Callback<MouseEvent>>,
    // pub duration: Option<u32>,

    html! {
    //   <div
    //     style={props.style.clone().unwrap_or(String::from(""))}
    //     // pub on_click: Option<Callback<MouseEvent>>,
    //     // pub duration: Option<u32>,
    //     // class={classString}
    //     // onclick={scrollToTop} ref={ref}
    //   >
    //     // {render_children(prefix_cls.clone(), root_prefix_cls.clone())}
    //   </div>
      <div class="ant-back-top">
        <div class="ant-back-top-content">
          <div class="ant-back-top-icon"></div>
        </div>
      </div>
    }
}
// const BackTop: React.FC<BackTopProps> = props => {
//   const [visible, setVisible] = useMergedState(false, {
//     value: props.visible,
//   });

//   const ref = React.createRef<HTMLDivElement>();
//   const scrollEvent = React.useRef<any>();

//   const getDefaultTarget = () =>
//     ref.current && ref.current.ownerDocument ? ref.current.ownerDocument : window;

//   const handleScroll = throttleByAnimationFrame(
//     (e: React.UIEvent<HTMLElement> | { target: any }) => {
//       const { visibilityHeight } = props;
//       const scrollTop = getScroll(e.target, true);
//       setVisible(scrollTop > visibilityHeight!);
//     },
//   );

//   const bindScrollEvent = () => {
//     const { target } = props;
//     const getTarget = target || getDefaultTarget;
//     const container = getTarget();
//     scrollEvent.current = addEventListener(container, 'scroll', (e: React.UIEvent<HTMLElement>) => {
//       handleScroll(e);
//     });
//     handleScroll({
//       target: container,
//     });
//   };

//   React.useEffect(() => {
//     bindScrollEvent();
//     return () => {
//       if (scrollEvent.current) {
//         scrollEvent.current.remove();
//       }
//       (handleScroll as any).cancel();
//     };
//   }, [props.target]);

//   const scrollToTop = (e: React.MouseEvent<HTMLDivElement>) => {
//     const { onClick, target, duration = 450 } = props;
//     scrollTo(0, {
//       getContainer: target || getDefaultTarget,
//       duration,
//     });
//     if (typeof onClick === 'function') {
//       onClick(e);
//     }
//   };

//   const renderChildren = ({
//     prefixCls,
//     rootPrefixCls,
//   }: {
//     prefixCls: string;
//     rootPrefixCls: string;
//   }) => {
//     const { children } = props;
//     const defaultElement = (
//       <div className={`${prefixCls}-content`}>
//         <div className={`${prefixCls}-icon`}>
//           <VerticalAlignTopOutlined />
//         </div>
//       </div>
//     );
//     return (
//       <CSSMotion visible={visible} motionName={`${rootPrefixCls}-fade`}>
//         {({ className: motionClassName }) =>
//           cloneElement(children || defaultElement, ({ className }) => ({
//             className: classNames(motionClassName, className),
//           }))
//         }
//       </CSSMotion>
//     );
//   };

//   const { getPrefixCls, direction } = React.useContext(ConfigContext);
//   const { prefixCls: customizePrefixCls, className = '' } = props;
//   const prefixCls = getPrefixCls('back-top', customizePrefixCls);
//   const rootPrefixCls = getPrefixCls();
//   const classString = classNames(
//     prefixCls,
//     {
//       [`${prefixCls}-rtl`]: direction === 'rtl',
//     },
//     className,
//   );

//   // fix https://fb.me/react-unknown-prop
//   const divProps = omit(props, [
//     'prefixCls',
//     'className',
//     'children',
//     'visibilityHeight',
//     'target',
//     'visible',
//   ]);

//   return (
//     <div {...divProps} className={classString} onClick={scrollToTop} ref={ref}>
//       {renderChildren({ prefixCls, rootPrefixCls })}
//     </div>
//   );
// };

// BackTop.defaultProps = {
//   visibilityHeight: 400,
// };

// export default React.memo(BackTop);

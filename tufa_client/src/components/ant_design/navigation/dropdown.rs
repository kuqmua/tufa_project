// // use yew::{function_component, html, Html, Properties};

// // #[derive(Properties, PartialEq)]
// // pub struct DropdownProps {
// //     pub disabled: bool, //	Whether the dropdown menu is disabled	boolean	-
// //                         //pub get_popup_container:  //	To set the container of the dropdown menu. The default is to create a div element in body, but you can reset it to the scrolling area and make a relative reposition. Example on CodePen.	Function(triggerNode)	() => document.body
// //                         //pub overlay: 	//The dropdown menu	Menu | () => Menu	-
// //                         //pub overlayClassName: //	Class name of the dropdown root element	string	-	3.11.0
// //                         //pub overlayStyle: 	Style of the dropdown root element	object	-	3.11.0
// //                         //pub placement:	Placement of popup menu: bottomLeft, bottomCenter, bottomRight, topLeft, topCenter or topRight	String	bottomLeft
// //                         //pub trigger:	The trigger mode which executes the dropdown action. Note that hover can't be used on touchscreens.	Array<click|hover|contextMenu>	['hover']
// //                         //pub visible:	Whether the dropdown menu is currently visible	boolean	-
// //                         //pub onVisibleChange:	Called when the visible state is changed.	Function(visible)
// // }

// // #[function_component(Dropdown)]
// // pub fn dropdown(props: &DropdownProps) -> Html {
// //     html! {}
// // }

// import EllipsisOutlined from '@ant-design/icons/EllipsisOutlined';
// import classNames from 'classnames';
// import * as React from 'react';
// import type { ButtonProps } from '../button';
// import Button from '../button';
// import type { ButtonHTMLType } from '../button/button';
// import type { ButtonGroupProps } from '../button/button-group';
// import { ConfigContext } from '../config-provider';
// import type { DropdownProps } from './dropdown';
// import Dropdown from './dropdown';

// const ButtonGroup = Button.Group;

// export type DropdownButtonType = 'default' | 'primary' | 'ghost' | 'dashed' | 'link' | 'text';

// export interface DropdownButtonProps extends ButtonGroupProps, DropdownProps {
//   type?: DropdownButtonType;
//   htmlType?: ButtonHTMLType;
//   disabled?: boolean;
//   loading?: ButtonProps['loading'];
//   onClick?: React.MouseEventHandler<HTMLButtonElement>;
//   icon?: React.ReactNode;
//   href?: string;
//   children?: React.ReactNode;
//   title?: string;
//   buttonsRender?: (buttons: React.ReactNode[]) => React.ReactNode[];
// }

// interface DropdownButtonInterface extends React.FC<DropdownButtonProps> {
//   __ANT_BUTTON: boolean;
// }

// const DropdownButton: DropdownButtonInterface = props => {
//   const {
//     getPopupContainer: getContextPopupContainer,
//     getPrefixCls,
//     direction,
//   } = React.useContext(ConfigContext);

//   const {
//     prefixCls: customizePrefixCls,
//     type = 'default',
//     disabled,
//     loading,
//     onClick,
//     htmlType,
//     children,
//     className,
//     overlay,
//     trigger,
//     align,
//     visible,
//     onVisibleChange,
//     placement,
//     getPopupContainer,
//     href,
//     icon = <EllipsisOutlined />,
//     title,
//     buttonsRender = (buttons: React.ReactNode[]) => buttons,
//     mouseEnterDelay,
//     mouseLeaveDelay,
//     overlayClassName,
//     overlayStyle,
//     destroyPopupOnHide,
//     ...restProps
//   } = props;

//   const prefixCls = getPrefixCls('dropdown-button', customizePrefixCls);
//   const dropdownProps = {
//     align,
//     overlay,
//     disabled,
//     trigger: disabled ? [] : trigger,
//     onVisibleChange,
//     getPopupContainer: getPopupContainer || getContextPopupContainer,
//     mouseEnterDelay,
//     mouseLeaveDelay,
//     overlayClassName,
//     overlayStyle,
//     destroyPopupOnHide,
//   } as DropdownProps;

//   if ('visible' in props) {
//     dropdownProps.visible = visible;
//   }

//   if ('placement' in props) {
//     dropdownProps.placement = placement;
//   } else {
//     dropdownProps.placement = direction === 'rtl' ? 'bottomLeft' : 'bottomRight';
//   }

//   const leftButton = (
//     <Button
//       type={type}
//       disabled={disabled}
//       loading={loading}
//       onClick={onClick}
//       htmlType={htmlType}
//       href={href}
//       title={title}
//     >
//       {children}
//     </Button>
//   );

//   const rightButton = <Button type={type} icon={icon} />;

//   const [leftButtonToRender, rightButtonToRender] = buttonsRender!([leftButton, rightButton]);

//   return (
//     <ButtonGroup {...restProps} className={classNames(prefixCls, className)}>
//       {leftButtonToRender}
//       <Dropdown {...dropdownProps}>{rightButtonToRender}</Dropdown>
//     </ButtonGroup>
//   );
// };

// DropdownButton.__ANT_BUTTON = true;

// export default DropdownButton;

// import RightOutlined from '@ant-design/icons/RightOutlined';
// import classNames from 'classnames';
// import RcDropdown from 'rc-dropdown';
// import useEvent from 'rc-util/lib/hooks/useEvent';
// import useMergedState from 'rc-util/lib/hooks/useMergedState';
// import * as React from 'react';
// import { ConfigContext } from '../config-provider';
// import { OverrideProvider } from '../menu/OverrideContext';
// import getPlacements from '../_util/placements';
// import { cloneElement } from '../_util/reactNode';
// import { tuple } from '../_util/type';
// import warning from '../_util/warning';
// import DropdownButton from './dropdown-button';

// const Placements = tuple(
//   'topLeft',
//   'topCenter',
//   'topRight',
//   'bottomLeft',
//   'bottomCenter',
//   'bottomRight',
//   'top',
//   'bottom',
// );

// type Placement = typeof Placements[number];

// type OverlayFunc = () => React.ReactElement;

// type Align = {
//   points?: [string, string];
//   offset?: [number, number];
//   targetOffset?: [number, number];
//   overflow?: {
//     adjustX?: boolean;
//     adjustY?: boolean;
//   };
//   useCssRight?: boolean;
//   useCssBottom?: boolean;
//   useCssTransform?: boolean;
// };

// export type DropdownArrowOptions = {
//   pointAtCenter?: boolean;
// };

// export interface DropdownProps {
//   autoFocus?: boolean;
//   arrow?: boolean | DropdownArrowOptions;
//   trigger?: ('click' | 'hover' | 'contextMenu')[];
//   overlay: React.ReactElement | OverlayFunc;
//   onVisibleChange?: (visible: boolean) => void;
//   visible?: boolean;
//   disabled?: boolean;
//   destroyPopupOnHide?: boolean;
//   align?: Align;
//   getPopupContainer?: (triggerNode: HTMLElement) => HTMLElement;
//   prefixCls?: string;
//   className?: string;
//   transitionName?: string;
//   placement?: Placement;
//   overlayClassName?: string;
//   overlayStyle?: React.CSSProperties;
//   forceRender?: boolean;
//   mouseEnterDelay?: number;
//   mouseLeaveDelay?: number;
//   openClassName?: string;
//   children?: React.ReactNode;
// }

// interface DropdownInterface extends React.FC<DropdownProps> {
//   Button: typeof DropdownButton;
// }

// const Dropdown: DropdownInterface = props => {
//   const {
//     getPopupContainer: getContextPopupContainer,
//     getPrefixCls,
//     direction,
//   } = React.useContext(ConfigContext);

//   const getTransitionName = () => {
//     const rootPrefixCls = getPrefixCls();
//     const { placement = '', transitionName } = props;
//     if (transitionName !== undefined) {
//       return transitionName;
//     }
//     if (placement.indexOf('top') >= 0) {
//       return `${rootPrefixCls}-slide-down`;
//     }
//     return `${rootPrefixCls}-slide-up`;
//   };

//   const getPlacement = () => {
//     const { placement } = props;
//     if (!placement) {
//       return direction === 'rtl' ? ('bottomRight' as Placement) : ('bottomLeft' as Placement);
//     }

//     if (placement.includes('Center')) {
//       const newPlacement = placement.slice(0, placement.indexOf('Center'));
//       warning(
//         !placement.includes('Center'),
//         'Dropdown',
//         `You are using '${placement}' placement in Dropdown, which is deprecated. Try to use '${newPlacement}' instead.`,
//       );
//       return newPlacement;
//     }

//     return placement;
//   };

//   const {
//     arrow,
//     prefixCls: customizePrefixCls,
//     children,
//     trigger,
//     disabled,
//     getPopupContainer,
//     overlayClassName,
//     visible,
//     onVisibleChange,
//   } = props;

//   const prefixCls = getPrefixCls('dropdown', customizePrefixCls);
//   const child = React.Children.only(children) as React.ReactElement<any>;

//   const dropdownTrigger = cloneElement(child, {
//     className: classNames(
//       `${prefixCls}-trigger`,
//       {
//         [`${prefixCls}-rtl`]: direction === 'rtl',
//       },
//       child.props.className,
//     ),
//     disabled,
//   });

//   const triggerActions = disabled ? [] : trigger;
//   let alignPoint;
//   if (triggerActions && triggerActions.indexOf('contextMenu') !== -1) {
//     alignPoint = true;
//   }

//   // =========================== Visible ============================
//   const [mergedVisible, setVisible] = useMergedState(false, {
//     value: visible,
//   });

//   const onInnerVisibleChange = useEvent((nextVisible: boolean) => {
//     onVisibleChange?.(nextVisible);
//     setVisible(nextVisible);
//   });

//   // =========================== Overlay ============================
//   const overlayClassNameCustomized = classNames(overlayClassName, {
//     [`${prefixCls}-rtl`]: direction === 'rtl',
//   });

//   const builtinPlacements = getPlacements({
//     arrowPointAtCenter: typeof arrow === 'object' && arrow.pointAtCenter,
//     autoAdjustOverflow: true,
//   });

//   const onMenuClick = React.useCallback(() => {
//     setVisible(false);
//   }, []);

//   const renderOverlay = () => {
//     // rc-dropdown already can process the function of overlay, but we have check logic here.
//     // So we need render the element to check and pass back to rc-dropdown.
//     const { overlay } = props;

//     let overlayNode;
//     if (typeof overlay === 'function') {
//       overlayNode = (overlay as OverlayFunc)();
//     } else {
//       overlayNode = overlay;
//     }
//     overlayNode = React.Children.only(
//       typeof overlayNode === 'string' ? <span>{overlayNode}</span> : overlayNode,
//     );

//     return (
//       <OverrideProvider
//         prefixCls={`${prefixCls}-menu`}
//         expandIcon={
//           <span className={`${prefixCls}-menu-submenu-arrow`}>
//             <RightOutlined className={`${prefixCls}-menu-submenu-arrow-icon`} />
//           </span>
//         }
//         mode="vertical"
//         selectable={false}
//         onClick={onMenuClick}
//         validator={({ mode }) => {
//           // Warning if use other mode
//           warning(
//             !mode || mode === 'vertical',
//             'Dropdown',
//             `mode="${mode}" is not supported for Dropdown's Menu.`,
//           );
//         }}
//       >
//         {overlayNode}
//       </OverrideProvider>
//     );
//   };

//   // ============================ Render ============================
//   return (
//     <RcDropdown
//       alignPoint={alignPoint}
//       {...props}
//       visible={mergedVisible}
//       builtinPlacements={builtinPlacements}
//       arrow={!!arrow}
//       overlayClassName={overlayClassNameCustomized}
//       prefixCls={prefixCls}
//       getPopupContainer={getPopupContainer || getContextPopupContainer}
//       transitionName={getTransitionName()}
//       trigger={triggerActions}
//       overlay={renderOverlay}
//       placement={getPlacement()}
//       onVisibleChange={onInnerVisibleChange}
//     >
//       {dropdownTrigger}
//     </RcDropdown>
//   );
// };

// Dropdown.Button = DropdownButton;

// Dropdown.defaultProps = {
//   mouseEnterDelay: 0.15,
//   mouseLeaveDelay: 0.1,
// };

// export default Dropdown;

// import Dropdown from './dropdown';

// export type {
//   DropdownProps,
//   // typo, but we need to support it for backwards compatibility
//   // https://github.com/ant-design/ant-design/pull/35161
//   DropdownProps as DropDownProps,
// } from './dropdown';
// export type { DropdownButtonProps, DropdownButtonType } from './dropdown-button';
// export default Dropdown;

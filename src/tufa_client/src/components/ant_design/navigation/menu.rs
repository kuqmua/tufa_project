// import { createContext } from 'react';
// import type { DirectionType } from '../config-provider';

// export type MenuTheme = 'light' | 'dark';

// export interface MenuContextProps {
//   prefixCls: string;
//   inlineCollapsed: boolean;
//   antdMenuTheme?: MenuTheme;
//   direction?: DirectionType;
//   firstLevel: boolean;
//   /** @private Internal Usage. Safe to remove */
//   disableMenuItemTitleTooltip?: boolean;
// }

// const MenuContext = createContext<MenuContextProps>({
//   prefixCls: '',
//   firstLevel: true,
//   inlineCollapsed: false,
// });

// export default MenuContext;

// import classNames from 'classnames';
// import { Divider } from 'rc-menu';
// import * as React from 'react';
// import { ConfigContext } from '../config-provider';

// export interface MenuDividerProps extends React.HTMLAttributes<HTMLLIElement> {
//   className?: string;
//   prefixCls?: string;
//   style?: React.CSSProperties;
//   dashed?: boolean;
// }

// const MenuDivider: React.FC<MenuDividerProps> = ({
//   prefixCls: customizePrefixCls,
//   className,
//   dashed,
//   ...restProps
// }) => {
//   const { getPrefixCls } = React.useContext(ConfigContext);

//   const prefixCls = getPrefixCls('menu', customizePrefixCls);
//   const classString = classNames(
//     {
//       [`${prefixCls}-item-divider-dashed`]: !!dashed,
//     },
//     className,
//   );

//   return <Divider className={classString} {...restProps} />;
// };

// export default MenuDivider;

// import classNames from 'classnames';
// import type { MenuItemProps as RcMenuItemProps } from 'rc-menu';
// import { Item } from 'rc-menu';
// import toArray from 'rc-util/lib/Children/toArray';
// import * as React from 'react';
// import type { SiderContextProps } from '../layout/Sider';
// import { SiderContext } from '../layout/Sider';
// import type { TooltipProps } from '../tooltip';
// import Tooltip from '../tooltip';
// import { cloneElement, isValidElement } from '../_util/reactNode';
// import type { MenuContextProps } from './MenuContext';
// import MenuContext from './MenuContext';

// export interface MenuItemProps extends Omit<RcMenuItemProps, 'title'> {
//   icon?: React.ReactNode;
//   danger?: boolean;
//   title?: React.ReactNode;
// }

// export default class MenuItem extends React.Component<MenuItemProps> {
//   static contextType = MenuContext;

//   context: MenuContextProps;

//   renderItemChildren(inlineCollapsed: boolean) {
//     const { prefixCls, firstLevel } = this.context;
//     const { icon, children } = this.props;

//     const wrapNode = <span className={`${prefixCls}-title-content`}>{children}</span>;
//     // inline-collapsed.md demo 依赖 span 来隐藏文字,有 icon 属性，则内部包裹一个 span
//     // ref: https://github.com/ant-design/ant-design/pull/23456
//     if (!icon || (isValidElement(children) && children.type === 'span')) {
//       if (children && inlineCollapsed && firstLevel && typeof children === 'string') {
//         return <div className={`${prefixCls}-inline-collapsed-noicon`}>{children.charAt(0)}</div>;
//       }
//     }
//     return wrapNode;
//   }

//   renderItem = ({ siderCollapsed }: SiderContextProps) => {
//     const { prefixCls, firstLevel, inlineCollapsed, direction, disableMenuItemTitleTooltip } =
//       this.context;
//     const { className, children } = this.props;
//     const { title, icon, danger, ...rest } = this.props;

//     let tooltipTitle = title;
//     if (typeof title === 'undefined') {
//       tooltipTitle = firstLevel ? children : '';
//     } else if (title === false) {
//       tooltipTitle = '';
//     }
//     const tooltipProps: TooltipProps = {
//       title: tooltipTitle,
//     };

//     if (!siderCollapsed && !inlineCollapsed) {
//       tooltipProps.title = null;
//       // Reset `visible` to fix control mode tooltip display not correct
//       // ref: https://github.com/ant-design/ant-design/issues/16742
//       tooltipProps.visible = false;
//     }
//     const childrenLength = toArray(children).length;

//     let returnNode = (
//       <Item
//         {...rest}
//         className={classNames(
//           {
//             [`${prefixCls}-item-danger`]: danger,
//             [`${prefixCls}-item-only-child`]: (icon ? childrenLength + 1 : childrenLength) === 1,
//           },
//           className,
//         )}
//         title={typeof title === 'string' ? title : undefined}
//       >
//         {cloneElement(icon, {
//           className: classNames(
//             isValidElement(icon) ? icon.props?.className : '',
//             `${prefixCls}-item-icon`,
//           ),
//         })}
//         {this.renderItemChildren(inlineCollapsed)}
//       </Item>
//     );

//     if (!disableMenuItemTitleTooltip) {
//       returnNode = (
//         <Tooltip
//           {...tooltipProps}
//           placement={direction === 'rtl' ? 'left' : 'right'}
//           overlayClassName={`${prefixCls}-inline-collapsed-tooltip`}
//         >
//           {returnNode}
//         </Tooltip>
//       );
//     }

//     return returnNode;
//   };

//   render() {
//     return <SiderContext.Consumer>{this.renderItem}</SiderContext.Consumer>;
//   }
// }

// import * as React from 'react';
// import type { MenuProps } from '.';

// // Used for Dropdown only
// export interface OverrideContextProps {
//   prefixCls?: string;
//   expandIcon?: React.ReactNode;
//   mode?: MenuProps['mode'];
//   selectable?: boolean;
//   validator?: (menuProps: Pick<MenuProps, 'mode'>) => void;
//   onClick?: () => void;
// }

// /** @private Internal Usage. Only used for Dropdown component. Do not use this in your production. */
// const OverrideContext = React.createContext<OverrideContextProps | null>(null);

// /** @private Internal Usage. Only used for Dropdown component. Do not use this in your production. */
// export const OverrideProvider = ({
//   children,
//   ...restProps
// }: OverrideContextProps & { children: React.ReactNode }) => {
//   const override = React.useContext(OverrideContext);

//   const context = React.useMemo(
//     () => ({
//       ...override,
//       ...restProps,
//     }),
//     [
//       override,
//       restProps.prefixCls,
//       // restProps.expandIcon, Not mark as deps since this is a ReactNode
//       restProps.mode,
//       restProps.selectable,
//       // restProps.validator, Not mark as deps since this is a function
//     ],
//   );

//   return <OverrideContext.Provider value={context}>{children}</OverrideContext.Provider>;
// };

// export default OverrideContext;

// import classNames from 'classnames';
// import { SubMenu as RcSubMenu, useFullPath } from 'rc-menu';
// import omit from 'rc-util/lib/omit';
// import * as React from 'react';
// import { cloneElement, isValidElement } from '../_util/reactNode';
// import type { MenuTheme } from './MenuContext';
// import MenuContext from './MenuContext';

// interface TitleEventEntity {
//   key: string;
//   domEvent: React.MouseEvent<HTMLElement> | React.KeyboardEvent<HTMLElement>;
// }

// export interface SubMenuProps {
//   className?: string;
//   disabled?: boolean;
//   level?: number;
//   title?: React.ReactNode;
//   icon?: React.ReactNode;
//   style?: React.CSSProperties;
//   onTitleClick?: (e: TitleEventEntity) => void;
//   onTitleMouseEnter?: (e: TitleEventEntity) => void;
//   onTitleMouseLeave?: (e: TitleEventEntity) => void;
//   popupOffset?: [number, number];
//   popupClassName?: string;
//   children?: React.ReactNode;
//   theme?: MenuTheme;
// }

// function SubMenu(props: SubMenuProps) {
//   const { popupClassName, icon, title, theme } = props;
//   const context = React.useContext(MenuContext);
//   const { prefixCls, inlineCollapsed, antdMenuTheme } = context;

//   const parentPath = useFullPath();

//   let titleNode: React.ReactNode;

//   if (!icon) {
//     titleNode =
//       inlineCollapsed && !parentPath.length && title && typeof title === 'string' ? (
//         <div className={`${prefixCls}-inline-collapsed-noicon`}>{title.charAt(0)}</div>
//       ) : (
//         <span className={`${prefixCls}-title-content`}>{title}</span>
//       );
//   } else {
//     // inline-collapsed.md demo 依赖 span 来隐藏文字,有 icon 属性，则内部包裹一个 span
//     // ref: https://github.com/ant-design/ant-design/pull/23456
//     const titleIsSpan = isValidElement(title) && title.type === 'span';
//     titleNode = (
//       <>
//         {cloneElement(icon, {
//           className: classNames(
//             isValidElement(icon) ? icon.props?.className : '',
//             `${prefixCls}-item-icon`,
//           ),
//         })}
//         {titleIsSpan ? title : <span className={`${prefixCls}-title-content`}>{title}</span>}
//       </>
//     );
//   }

//   const contextValue = React.useMemo(
//     () => ({
//       ...context,
//       firstLevel: false,
//     }),
//     [context],
//   );

//   return (
//     <MenuContext.Provider value={contextValue}>
//       <RcSubMenu
//         {...omit(props, ['icon'])}
//         title={titleNode}
//         popupClassName={classNames(
//           prefixCls,
//           `${prefixCls}-${theme || antdMenuTheme}`,
//           popupClassName,
//         )}
//       />
//     </MenuContext.Provider>
//   );
// }

// export default SubMenu;

// import EllipsisOutlined from '@ant-design/icons/EllipsisOutlined';
// import classNames from 'classnames';
// import type { MenuProps as RcMenuProps, MenuRef } from 'rc-menu';
// import RcMenu, { ItemGroup } from 'rc-menu';
// import useEvent from 'rc-util/lib/hooks/useEvent';
// import omit from 'rc-util/lib/omit';
// import * as React from 'react';
// import { forwardRef } from 'react';
// import { ConfigContext } from '../config-provider';
// import type { SiderContextProps } from '../layout/Sider';
// import { SiderContext } from '../layout/Sider';
// import collapseMotion from '../_util/motion';
// import { cloneElement } from '../_util/reactNode';
// import warning from '../_util/warning';
// import type { ItemType } from './hooks/useItems';
// import useItems from './hooks/useItems';
// import MenuContext, { MenuTheme } from './MenuContext';
// import MenuDivider from './MenuDivider';
// import Item, { MenuItemProps } from './MenuItem';
// import OverrideContext from './OverrideContext';
// import SubMenu, { SubMenuProps } from './SubMenu';

// export { MenuItemGroupProps } from 'rc-menu';
// export { MenuDividerProps } from './MenuDivider';
// export { MenuTheme, SubMenuProps, MenuItemProps };

// export type MenuMode = 'vertical' | 'vertical-left' | 'vertical-right' | 'horizontal' | 'inline';

// export interface MenuProps extends Omit<RcMenuProps, 'items'> {
//   theme?: MenuTheme;
//   inlineIndent?: number;

//   // >>>>> Private
//   /**
//    * @private Internal Usage. Not promise crash if used in production. Connect with chenshuai2144
//    *   for removing.
//    */
//   _internalDisableMenuItemTitleTooltip?: boolean;

//   items?: ItemType[];
// }

// type InternalMenuProps = MenuProps &
//   SiderContextProps & {
//     collapsedWidth?: string | number;
//   };

// const InternalMenu = forwardRef<MenuRef, InternalMenuProps>((props, ref) => {
//   const override = React.useContext(OverrideContext) || {};

//   const { getPrefixCls, getPopupContainer, direction } = React.useContext(ConfigContext);

//   const rootPrefixCls = getPrefixCls();

//   const {
//     prefixCls: customizePrefixCls,
//     className,
//     theme = 'light',
//     expandIcon,
//     _internalDisableMenuItemTitleTooltip,
//     inlineCollapsed,
//     siderCollapsed,
//     items,
//     children,
//     mode,
//     selectable,
//     onClick,
//     ...restProps
//   } = props;

//   const passedProps = omit(restProps, ['collapsedWidth']);

//   // ========================= Items ===========================
//   const mergedChildren = useItems(items) || children;

//   // ======================== Warning ==========================
//   warning(
//     !('inlineCollapsed' in props && mode !== 'inline'),
//     'Menu',
//     '`inlineCollapsed` should only be used when `mode` is inline.',
//   );

//   warning(
//     !(props.siderCollapsed !== undefined && 'inlineCollapsed' in props),
//     'Menu',
//     '`inlineCollapsed` not control Menu under Sider. Should set `collapsed` on Sider instead.',
//   );

//   warning(
//     'items' in props && !children,
//     'Menu',
//     '`children` will be removed in next major version. Please use `items` instead.',
//   );

//   override.validator?.({ mode });

//   // ========================== Click ==========================
//   // Tell dropdown that item clicked
//   const onItemClick = useEvent<Required<MenuProps>['onClick']>((...args) => {
//     onClick?.(...args);
//     override?.onClick?.();
//   });

//   // ========================== Mode ===========================
//   const mergedMode = override.mode || mode;

//   // ======================= Selectable ========================
//   const mergedSelectable = selectable ?? override.selectable;

//   // ======================== Collapsed ========================
//   // Inline Collapsed
//   const mergedInlineCollapsed = React.useMemo(() => {
//     if (siderCollapsed !== undefined) {
//       return siderCollapsed;
//     }
//     return inlineCollapsed;
//   }, [inlineCollapsed, siderCollapsed]);

//   const defaultMotions = {
//     horizontal: { motionName: `${rootPrefixCls}-slide-up` },
//     inline: collapseMotion,
//     other: { motionName: `${rootPrefixCls}-zoom-big` },
//   };

//   const prefixCls = getPrefixCls('menu', customizePrefixCls || override.prefixCls);
//   const menuClassName = classNames(`${prefixCls}-${theme}`, className);

//   // ====================== Expand Icon ========================
//   let mergedExpandIcon: MenuProps[`expandIcon`];
//   if (typeof expandIcon === 'function') {
//     mergedExpandIcon = expandIcon;
//   } else {
//     mergedExpandIcon = cloneElement(expandIcon || override.expandIcon, {
//       className: `${prefixCls}-submenu-expand-icon`,
//     });
//   }

//   // ======================== Context ==========================
//   const contextValue = React.useMemo(
//     () => ({
//       prefixCls,
//       inlineCollapsed: mergedInlineCollapsed || false,
//       antdMenuTheme: theme,
//       direction,
//       firstLevel: true,
//       disableMenuItemTitleTooltip: _internalDisableMenuItemTitleTooltip,
//     }),
//     [prefixCls, mergedInlineCollapsed, theme, direction, _internalDisableMenuItemTitleTooltip],
//   );

//   // ========================= Render ==========================
//   return (
//     <OverrideContext.Provider value={null}>
//       <MenuContext.Provider value={contextValue}>
//         <RcMenu
//           getPopupContainer={getPopupContainer}
//           overflowedIndicator={<EllipsisOutlined />}
//           overflowedIndicatorPopupClassName={`${prefixCls}-${theme}`}
//           mode={mergedMode}
//           selectable={mergedSelectable}
//           onClick={onItemClick}
//           {...passedProps}
//           inlineCollapsed={mergedInlineCollapsed}
//           className={menuClassName}
//           prefixCls={prefixCls}
//           direction={direction}
//           defaultMotions={defaultMotions}
//           expandIcon={mergedExpandIcon}
//           ref={ref}
//         >
//           {mergedChildren}
//         </RcMenu>
//       </MenuContext.Provider>
//     </OverrideContext.Provider>
//   );
// });

// // We should keep this as ref-able
// class Menu extends React.Component<MenuProps, {}> {
//   static Divider = MenuDivider;

//   static Item = Item;

//   static SubMenu = SubMenu;

//   static ItemGroup = ItemGroup;

//   menu: MenuRef | null;

//   focus = (options?: FocusOptions) => {
//     this.menu?.focus(options);
//   };

//   render() {
//     return (
//       <SiderContext.Consumer>
//         {(context: SiderContextProps) => (
//           <InternalMenu
//             ref={node => {
//               this.menu = node;
//             }}
//             {...this.props}
//             {...context}
//           />
//         )}
//       </SiderContext.Consumer>
//     );
//   }
// }

// export default Menu;

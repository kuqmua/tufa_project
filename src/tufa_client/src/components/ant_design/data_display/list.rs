// import classNames from 'classnames';
// import type {
//   CSSProperties,
//   FC,
//   ForwardRefExoticComponent,
//   ForwardRefRenderFunction,
//   HTMLAttributes,
//   ReactElement,
//   ReactNode,
// } from 'react';
// import React, { Children, forwardRef, useContext } from 'react';
// import { ConfigContext } from '../config-provider';
// import { Col } from '../grid';
// import { cloneElement } from '../_util/reactNode';
// import { ListContext } from './index';

// export interface ListItemProps extends HTMLAttributes<HTMLDivElement> {
//   className?: string;
//   children?: ReactNode;
//   prefixCls?: string;
//   style?: CSSProperties;
//   extra?: ReactNode;
//   actions?: ReactNode[];
//   colStyle?: CSSProperties;
// }

// export interface ListItemMetaProps {
//   avatar?: ReactNode;
//   className?: string;
//   children?: ReactNode;
//   description?: ReactNode;
//   prefixCls?: string;
//   style?: CSSProperties;
//   title?: ReactNode;
// }

// export const Meta: FC<ListItemMetaProps> = ({
//   prefixCls: customizePrefixCls,
//   className,
//   avatar,
//   title,
//   description,
//   ...others
// }) => {
//   const { getPrefixCls } = useContext(ConfigContext);

//   const prefixCls = getPrefixCls('list', customizePrefixCls);
//   const classString = classNames(`${prefixCls}-item-meta`, className);

//   const content = (
//     <div className={`${prefixCls}-item-meta-content`}>
//       {title && <h4 className={`${prefixCls}-item-meta-title`}>{title}</h4>}
//       {description && <div className={`${prefixCls}-item-meta-description`}>{description}</div>}
//     </div>
//   );

//   return (
//     <div {...others} className={classString}>
//       {avatar && <div className={`${prefixCls}-item-meta-avatar`}>{avatar}</div>}
//       {(title || description) && content}
//     </div>
//   );
// };

// export interface ListItemTypeProps
//   extends ForwardRefExoticComponent<ListItemProps & React.RefAttributes<HTMLElement>> {
//   Meta: typeof Meta;
// }

// const InternalItem: ForwardRefRenderFunction<HTMLDivElement, ListItemProps> = (
//   { prefixCls: customizePrefixCls, children, actions, extra, className, colStyle, ...others },
//   ref,
// ) => {
//   const { grid, itemLayout } = useContext(ListContext);
//   const { getPrefixCls } = useContext(ConfigContext);

//   const isItemContainsTextNodeAndNotSingular = () => {
//     let result;
//     Children.forEach(children, (element: ReactElement<any>) => {
//       if (typeof element === 'string') {
//         result = true;
//       }
//     });
//     return result && Children.count(children) > 1;
//   };

//   const isFlexMode = () => {
//     if (itemLayout === 'vertical') {
//       return !!extra;
//     }
//     return !isItemContainsTextNodeAndNotSingular();
//   };

//   const prefixCls = getPrefixCls('list', customizePrefixCls);
//   const actionsContent = actions && actions.length > 0 && (
//     <ul className={`${prefixCls}-item-action`} key="actions">
//       {actions.map((action: ReactNode, i: number) => (
//         // eslint-disable-next-line react/no-array-index-key
//         <li key={`${prefixCls}-item-action-${i}`}>
//           {action}
//           {i !== actions.length - 1 && <em className={`${prefixCls}-item-action-split`} />}
//         </li>
//       ))}
//     </ul>
//   );
//   const Element = grid ? 'div' : 'li';
//   const itemChildren = (
//     <Element
//       {...(others as any)} // `li` element `onCopy` prop args is not same as `div`
//       {...(!grid ? { ref } : {})}
//       className={classNames(
//         `${prefixCls}-item`,
//         {
//           [`${prefixCls}-item-no-flex`]: !isFlexMode(),
//         },
//         className,
//       )}
//     >
//       {itemLayout === 'vertical' && extra
//         ? [
//             <div className={`${prefixCls}-item-main`} key="content">
//               {children}
//               {actionsContent}
//             </div>,
//             <div className={`${prefixCls}-item-extra`} key="extra">
//               {extra}
//             </div>,
//           ]
//         : [children, actionsContent, cloneElement(extra, { key: 'extra' })]}
//     </Element>
//   );

//   return grid ? (
//     <Col ref={ref} flex={1} style={colStyle}>
//       {itemChildren}
//     </Col>
//   ) : (
//     itemChildren
//   );
// };
// const Item = forwardRef(InternalItem) as ListItemTypeProps;

// Item.Meta = Meta;

// export default Item;

// import classNames from 'classnames';
// import * as React from 'react';
// import type { RenderEmptyHandler } from '../config-provider';
// import { ConfigContext } from '../config-provider';
// import defaultRenderEmpty from '../config-provider/defaultRenderEmpty';
// import { Row } from '../grid';
// import useBreakpoint from '../grid/hooks/useBreakpoint';
// import type { PaginationConfig } from '../pagination';
// import Pagination from '../pagination';
// import type { SpinProps } from '../spin';
// import Spin from '../spin';
// import type { Breakpoint } from '../_util/responsiveObserve';
// import { responsiveArray } from '../_util/responsiveObserve';
// import Item from './Item';

// export { ListItemMetaProps, ListItemProps } from './Item';

// export type ColumnCount = number;

// export type ColumnType = 'gutter' | 'column' | 'xs' | 'sm' | 'md' | 'lg' | 'xl' | 'xxl';

// export interface ListGridType {
//   gutter?: number;
//   column?: ColumnCount;
//   xs?: ColumnCount;
//   sm?: ColumnCount;
//   md?: ColumnCount;
//   lg?: ColumnCount;
//   xl?: ColumnCount;
//   xxl?: ColumnCount;
// }

// export type ListSize = 'small' | 'default' | 'large';

// export type ListItemLayout = 'horizontal' | 'vertical';

// export interface ListProps<T> {
//   bordered?: boolean;
//   className?: string;
//   style?: React.CSSProperties;
//   children?: React.ReactNode;
//   dataSource?: T[];
//   extra?: React.ReactNode;
//   grid?: ListGridType;
//   id?: string;
//   itemLayout?: ListItemLayout;
//   loading?: boolean | SpinProps;
//   loadMore?: React.ReactNode;
//   pagination?: PaginationConfig | false;
//   prefixCls?: string;
//   rowKey?: ((item: T) => React.Key) | keyof T;
//   renderItem?: (item: T, index: number) => React.ReactNode;
//   size?: ListSize;
//   split?: boolean;
//   header?: React.ReactNode;
//   footer?: React.ReactNode;
//   locale?: ListLocale;
// }

// export interface ListLocale {
//   emptyText: React.ReactNode;
// }

// export interface ListConsumerProps {
//   grid?: any;
//   itemLayout?: string;
// }

// export const ListContext = React.createContext<ListConsumerProps>({});

// export const ListConsumer = ListContext.Consumer;

// function List<T>({
//   pagination = false as ListProps<any>['pagination'],
//   prefixCls: customizePrefixCls,
//   bordered = false,
//   split = true,
//   className,
//   children,
//   itemLayout,
//   loadMore,
//   grid,
//   dataSource = [],
//   size,
//   header,
//   footer,
//   loading = false,
//   rowKey,
//   renderItem,
//   locale,
//   ...rest
// }: ListProps<T>) {
//   const paginationObj = pagination && typeof pagination === 'object' ? pagination : {};

//   const [paginationCurrent, setPaginationCurrent] = React.useState(
//     paginationObj.defaultCurrent || 1,
//   );
//   const [paginationSize, setPaginationSize] = React.useState(paginationObj.defaultPageSize || 10);

//   const { getPrefixCls, renderEmpty, direction } = React.useContext(ConfigContext);

//   const defaultPaginationProps = {
//     current: 1,
//     total: 0,
//   };

//   const listItemsKeys: { [index: number]: React.Key } = {};

//   const triggerPaginationEvent = (eventName: string) => (page: number, pageSize: number) => {
//     setPaginationCurrent(page);
//     setPaginationSize(pageSize);
//     if (pagination && (pagination as any)[eventName]) {
//       (pagination as any)[eventName](page, pageSize);
//     }
//   };

//   const onPaginationChange = triggerPaginationEvent('onChange');

//   const onPaginationShowSizeChange = triggerPaginationEvent('onShowSizeChange');

//   const renderInnerItem = (item: T, index: number) => {
//     if (!renderItem) return null;

//     let key;

//     if (typeof rowKey === 'function') {
//       key = rowKey(item);
//     } else if (rowKey) {
//       key = item[rowKey];
//     } else {
//       key = (item as any).key;
//     }

//     if (!key) {
//       key = `list-item-${index}`;
//     }

//     listItemsKeys[index] = key;

//     return renderItem(item, index);
//   };

//   const isSomethingAfterLastItem = () => !!(loadMore || pagination || footer);

//   const renderEmptyFunc = (prefixCls: string, renderEmptyHandler: RenderEmptyHandler) => (
//     <div className={`${prefixCls}-empty-text`}>
//       {(locale && locale.emptyText) || renderEmptyHandler('List')}
//     </div>
//   );

//   const prefixCls = getPrefixCls('list', customizePrefixCls);
//   let loadingProp = loading;
//   if (typeof loadingProp === 'boolean') {
//     loadingProp = {
//       spinning: loadingProp,
//     };
//   }
//   const isLoading = loadingProp && loadingProp.spinning;

//   // large => lg
//   // small => sm
//   let sizeCls = '';
//   switch (size) {
//     case 'large':
//       sizeCls = 'lg';
//       break;
//     case 'small':
//       sizeCls = 'sm';
//       break;
//     default:
//       break;
//   }

//   const classString = classNames(
//     prefixCls,
//     {
//       [`${prefixCls}-vertical`]: itemLayout === 'vertical',
//       [`${prefixCls}-${sizeCls}`]: sizeCls,
//       [`${prefixCls}-split`]: split,
//       [`${prefixCls}-bordered`]: bordered,
//       [`${prefixCls}-loading`]: isLoading,
//       [`${prefixCls}-grid`]: !!grid,
//       [`${prefixCls}-something-after-last-item`]: isSomethingAfterLastItem(),
//       [`${prefixCls}-rtl`]: direction === 'rtl',
//     },
//     className,
//   );

//   const paginationProps = {
//     ...defaultPaginationProps,
//     total: dataSource.length,
//     current: paginationCurrent,
//     pageSize: paginationSize,
//     ...(pagination || {}),
//   };

//   const largestPage = Math.ceil(paginationProps.total / paginationProps.pageSize);
//   if (paginationProps.current > largestPage) {
//     paginationProps.current = largestPage;
//   }
//   const paginationContent = pagination ? (
//     <div className={`${prefixCls}-pagination`}>
//       <Pagination
//         {...paginationProps}
//         onChange={onPaginationChange}
//         onShowSizeChange={onPaginationShowSizeChange}
//       />
//     </div>
//   ) : null;

//   let splitDataSource = [...dataSource];
//   if (pagination) {
//     if (dataSource.length > (paginationProps.current - 1) * paginationProps.pageSize) {
//       splitDataSource = [...dataSource].splice(
//         (paginationProps.current - 1) * paginationProps.pageSize,
//         paginationProps.pageSize,
//       );
//     }
//   }

//   const needResponsive = Object.keys(grid || {}).some(key =>
//     ['xs', 'sm', 'md', 'lg', 'xl', 'xxl'].includes(key),
//   );
//   const screens = useBreakpoint(needResponsive);
//   const currentBreakpoint = React.useMemo(() => {
//     for (let i = 0; i < responsiveArray.length; i += 1) {
//       const breakpoint: Breakpoint = responsiveArray[i];
//       if (screens[breakpoint]) {
//         return breakpoint;
//       }
//     }
//     return undefined;
//   }, [screens]);

//   const colStyle = React.useMemo(() => {
//     if (!grid) {
//       return undefined;
//     }
//     const columnCount =
//       currentBreakpoint && grid[currentBreakpoint] ? grid[currentBreakpoint] : grid.column;
//     if (columnCount) {
//       return {
//         width: `${100 / columnCount}%`,
//         maxWidth: `${100 / columnCount}%`,
//       };
//     }
//   }, [grid?.column, currentBreakpoint]);

//   let childrenContent = isLoading && <div style={{ minHeight: 53 }} />;
//   if (splitDataSource.length > 0) {
//     const items = splitDataSource.map((item: T, index: number) => renderInnerItem(item, index));
//     const childrenList = React.Children.map(items, (child: React.ReactNode, index: number) => (
//       <div key={listItemsKeys[index]} style={colStyle}>
//         {child}
//       </div>
//     ));
//     childrenContent = grid ? (
//       <Row gutter={grid.gutter}>{childrenList}</Row>
//     ) : (
//       <ul className={`${prefixCls}-items`}>{items}</ul>
//     );
//   } else if (!children && !isLoading) {
//     childrenContent = renderEmptyFunc(prefixCls, renderEmpty || defaultRenderEmpty);
//   }

//   const paginationPosition = paginationProps.position || 'bottom';
//   const contextValue = React.useMemo(
//     () => ({ grid, itemLayout }),
//     [JSON.stringify(grid), itemLayout],
//   );

//   return (
//     <ListContext.Provider value={contextValue}>
//       <div className={classString} {...rest}>
//         {(paginationPosition === 'top' || paginationPosition === 'both') && paginationContent}
//         {header && <div className={`${prefixCls}-header`}>{header}</div>}
//         <Spin {...loadingProp}>
//           {childrenContent}
//           {children}
//         </Spin>
//         {footer && <div className={`${prefixCls}-footer`}>{footer}</div>}
//         {loadMore ||
//           ((paginationPosition === 'bottom' || paginationPosition === 'both') && paginationContent)}
//       </div>
//     </ListContext.Provider>
//   );
// }

// List.Item = Item;

// export default List;

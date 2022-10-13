// import classNames from 'classnames';
// import * as React from 'react';
// import type { KeyWiseTransferItem } from '.';
// import Pagination from '../pagination';
// import type { ElementOf } from '../_util/type';
// import { tuple } from '../_util/type';
// import type { PaginationType } from './interface';
// import type { RenderedItem, TransferListProps } from './list';
// import ListItem from './ListItem';

// export const OmitProps = tuple('handleFilter', 'handleClear', 'checkedKeys');
// export type OmitProp = ElementOf<typeof OmitProps>;
// type PartialTransferListProps<RecordType> = Omit<TransferListProps<RecordType>, OmitProp>;

// export interface TransferListBodyProps<RecordType> extends PartialTransferListProps<RecordType> {
//   filteredItems: RecordType[];
//   filteredRenderItems: RenderedItem<RecordType>[];
//   selectedKeys: string[];
// }

// function parsePagination(pagination?: PaginationType) {
//   if (!pagination) {
//     return null;
//   }

//   const defaultPagination = {
//     pageSize: 10,
//     simple: true,
//     showSizeChanger: false,
//     showLessItems: false,
//   };

//   if (typeof pagination === 'object') {
//     return {
//       ...defaultPagination,
//       ...pagination,
//     };
//   }

//   return defaultPagination;
// }

// interface TransferListBodyState {
//   current: number;
// }

// class ListBody<RecordType extends KeyWiseTransferItem> extends React.Component<
//   TransferListBodyProps<RecordType>,
//   TransferListBodyState
// > {
//   state = {
//     current: 1,
//   };

//   static getDerivedStateFromProps<T>(
//     { filteredRenderItems, pagination }: TransferListBodyProps<T>,
//     { current }: TransferListBodyState,
//   ) {
//     const mergedPagination = parsePagination(pagination);
//     if (mergedPagination) {
//       // Calculate the page number
//       const maxPageCount = Math.ceil(filteredRenderItems.length / mergedPagination.pageSize);

//       if (current > maxPageCount) {
//         return { current: maxPageCount };
//       }
//     }

//     return null;
//   }

//   onItemSelect = (item: RecordType) => {
//     const { onItemSelect, selectedKeys } = this.props;
//     const checked = selectedKeys.indexOf(item.key) >= 0;
//     onItemSelect(item.key, !checked);
//   };

//   onItemRemove = (item: RecordType) => {
//     const { onItemRemove } = this.props;
//     onItemRemove?.([item.key]);
//   };

//   onPageChange = (current: number) => {
//     this.setState({ current });
//   };

//   getItems = () => {
//     const { current } = this.state;
//     const { pagination, filteredRenderItems } = this.props;

//     const mergedPagination = parsePagination(pagination);

//     let displayItems = filteredRenderItems;

//     if (mergedPagination) {
//       displayItems = filteredRenderItems.slice(
//         (current - 1) * mergedPagination.pageSize,
//         current * mergedPagination.pageSize,
//       );
//     }

//     return displayItems;
//   };

//   render() {
//     const { current } = this.state;
//     const {
//       prefixCls,
//       onScroll,
//       filteredRenderItems,
//       selectedKeys,
//       disabled: globalDisabled,
//       showRemove,
//       pagination,
//     } = this.props;

//     const mergedPagination = parsePagination(pagination);
//     let paginationNode: React.ReactNode = null;

//     if (mergedPagination) {
//       paginationNode = (
//         <Pagination
//           simple={mergedPagination.simple}
//           showSizeChanger={mergedPagination.showSizeChanger}
//           showLessItems={mergedPagination.showLessItems}
//           size="small"
//           disabled={globalDisabled}
//           className={`${prefixCls}-pagination`}
//           total={filteredRenderItems.length}
//           pageSize={mergedPagination.pageSize}
//           current={current}
//           onChange={this.onPageChange}
//         />
//       );
//     }

//     return (
//       <>
//         <ul
//           className={classNames(`${prefixCls}-content`, {
//             [`${prefixCls}-content-show-remove`]: showRemove,
//           })}
//           onScroll={onScroll}
//         >
//           {this.getItems().map(({ renderedEl, renderedText, item }: RenderedItem<RecordType>) => {
//             const { disabled } = item;
//             const checked = selectedKeys.indexOf(item.key) >= 0;

//             return (
//               <ListItem
//                 disabled={globalDisabled || disabled}
//                 key={item.key}
//                 item={item}
//                 renderedText={renderedText}
//                 renderedEl={renderedEl}
//                 checked={checked}
//                 prefixCls={prefixCls}
//                 onClick={this.onItemSelect}
//                 onRemove={this.onItemRemove}
//                 showRemove={showRemove}
//               />
//             );
//           })}
//         </ul>

//         {paginationNode}
//       </>
//     );
//   }
// }

// export default ListBody;

// import DeleteOutlined from '@ant-design/icons/DeleteOutlined';
// import classNames from 'classnames';
// import * as React from 'react';
// import type { KeyWiseTransferItem, TransferLocale } from '.';
// import Checkbox from '../checkbox';
// import LocaleReceiver from '../locale-provider/LocaleReceiver';
// import defaultLocale from '../locale/default';
// import TransButton from '../_util/transButton';

// type ListItemProps<RecordType> = {
//   renderedText?: string | number;
//   renderedEl: React.ReactNode;
//   disabled?: boolean;
//   checked?: boolean;
//   prefixCls: string;
//   onClick: (item: RecordType) => void;
//   onRemove?: (item: RecordType) => void;
//   item: RecordType;
//   showRemove?: boolean;
// };

// const ListItem = <RecordType extends KeyWiseTransferItem>(props: ListItemProps<RecordType>) => {
//   const {
//     renderedText,
//     renderedEl,
//     item,
//     checked,
//     disabled,
//     prefixCls,
//     onClick,
//     onRemove,
//     showRemove,
//   } = props;

//   const className = classNames({
//     [`${prefixCls}-content-item`]: true,
//     [`${prefixCls}-content-item-disabled`]: disabled || item.disabled,
//     [`${prefixCls}-content-item-checked`]: checked,
//   });

//   let title: string | undefined;
//   if (typeof renderedText === 'string' || typeof renderedText === 'number') {
//     title = String(renderedText);
//   }

//   return (
//     <LocaleReceiver componentName="Transfer" defaultLocale={defaultLocale.Transfer}>
//       {(transferLocale: TransferLocale) => {
//         const liProps: React.HTMLAttributes<HTMLLIElement> = { className, title };
//         const labelNode = <span className={`${prefixCls}-content-item-text`}>{renderedEl}</span>;

//         // Show remove
//         if (showRemove) {
//           return (
//             <li {...liProps}>
//               {labelNode}
//               <TransButton
//                 disabled={disabled || item.disabled}
//                 className={`${prefixCls}-content-item-remove`}
//                 aria-label={transferLocale.remove}
//                 onClick={() => {
//                   onRemove?.(item);
//                 }}
//               >
//                 <DeleteOutlined />
//               </TransButton>
//             </li>
//           );
//         }

//         // Default click to select
//         liProps.onClick = disabled || item.disabled ? undefined : () => onClick(item);
//         return (
//           <li {...liProps}>
//             <Checkbox
//               className={`${prefixCls}-checkbox`}
//               checked={checked}
//               disabled={disabled || item.disabled}
//             />
//             {labelNode}
//           </li>
//         );
//       }}
//     </LocaleReceiver>
//   );
// };

// export default React.memo(ListItem);

// import classNames from 'classnames';
// import * as React from 'react';
// import type { ConfigConsumerProps, RenderEmptyHandler } from '../config-provider';
// import { ConfigConsumer } from '../config-provider';
// import defaultRenderEmpty from '../config-provider/defaultRenderEmpty';
// import { FormItemInputContext } from '../form/context';
// import LocaleReceiver from '../locale-provider/LocaleReceiver';
// import defaultLocale from '../locale/default';
// import type { InputStatus } from '../_util/statusUtils';
// import { getMergedStatus, getStatusClassNames } from '../_util/statusUtils';
// import warning from '../_util/warning';
// import type { PaginationType } from './interface';
// import type { TransferListProps } from './list';
// import List from './list';
// import type { TransferListBodyProps } from './ListBody';
// import Operation from './operation';
// import Search from './search';

// export { TransferListProps } from './list';
// export { TransferOperationProps } from './operation';
// export { TransferSearchProps } from './search';

// export type TransferDirection = 'left' | 'right';

// export interface RenderResultObject {
//   label: React.ReactElement;
//   value: string;
// }

// export type RenderResult = React.ReactElement | RenderResultObject | string | null;

// export interface TransferItem {
//   key?: string;
//   title?: string;
//   description?: string;
//   disabled?: boolean;
//   [name: string]: any;
// }

// export type KeyWise<T> = T & { key: string };

// export type KeyWiseTransferItem = KeyWise<TransferItem>;

// type TransferRender<RecordType> = (item: RecordType) => RenderResult;

// export interface ListStyle {
//   direction: TransferDirection;
// }

// export type SelectAllLabel =
//   | React.ReactNode
//   | ((info: { selectedCount: number; totalCount: number }) => React.ReactNode);

// export interface TransferLocale {
//   titles: React.ReactNode[];
//   notFoundContent?: React.ReactNode | React.ReactNode[];
//   searchPlaceholder: string;
//   itemUnit: string;
//   itemsUnit: string;
//   remove: string;
//   selectAll: string;
//   selectCurrent: string;
//   selectInvert: string;
//   removeAll: string;
//   removeCurrent: string;
// }

// export interface TransferProps<RecordType> {
//   prefixCls?: string;
//   className?: string;
//   disabled?: boolean;
//   dataSource: RecordType[];
//   targetKeys?: string[];
//   selectedKeys?: string[];
//   render?: TransferRender<RecordType>;
//   onChange?: (targetKeys: string[], direction: TransferDirection, moveKeys: string[]) => void;
//   onSelectChange?: (sourceSelectedKeys: string[], targetSelectedKeys: string[]) => void;
//   style?: React.CSSProperties;
//   listStyle?: ((style: ListStyle) => React.CSSProperties) | React.CSSProperties;
//   operationStyle?: React.CSSProperties;
//   titles?: React.ReactNode[];
//   operations?: string[];
//   showSearch?: boolean;
//   filterOption?: (inputValue: string, item: RecordType) => boolean;
//   locale?: Partial<TransferLocale>;
//   footer?: (
//     props: TransferListProps<RecordType>,
//     info?: {
//       direction: TransferDirection;
//     },
//   ) => React.ReactNode;
//   rowKey?: (record: RecordType) => string;
//   onSearch?: (direction: TransferDirection, value: string) => void;
//   onScroll?: (direction: TransferDirection, e: React.SyntheticEvent<HTMLUListElement>) => void;
//   children?: (props: TransferListBodyProps<RecordType>) => React.ReactNode;
//   showSelectAll?: boolean;
//   selectAllLabels?: SelectAllLabel[];
//   oneWay?: boolean;
//   pagination?: PaginationType;
//   status?: InputStatus;
// }

// interface TransferState {
//   sourceSelectedKeys: string[];
//   targetSelectedKeys: string[];
// }

// class Transfer<RecordType extends TransferItem = TransferItem> extends React.Component<
//   TransferProps<RecordType>,
//   TransferState
// > {
//   // For high-level customized Transfer @dqaria
//   static List = List;

//   static Operation = Operation;

//   static Search = Search;

//   static defaultProps = {
//     dataSource: [],
//     locale: {},
//     showSearch: false,
//     listStyle: () => {},
//   };

//   static getDerivedStateFromProps<T>({
//     selectedKeys,
//     targetKeys,
//     pagination,
//     children,
//   }: TransferProps<T>) {
//     if (selectedKeys) {
//       const mergedTargetKeys = targetKeys || [];
//       return {
//         sourceSelectedKeys: selectedKeys.filter(key => !mergedTargetKeys.includes(key)),
//         targetSelectedKeys: selectedKeys.filter(key => mergedTargetKeys.includes(key)),
//       };
//     }

//     warning(
//       !pagination || !children,
//       'Transfer',
//       '`pagination` not support customize render list.',
//     );

//     return null;
//   }

//   separatedDataSource: {
//     leftDataSource: RecordType[];
//     rightDataSource: RecordType[];
//   } | null = null;

//   constructor(props: TransferProps<RecordType>) {
//     super(props);

//     const { selectedKeys = [], targetKeys = [] } = props;
//     this.state = {
//       sourceSelectedKeys: selectedKeys.filter(key => targetKeys.indexOf(key) === -1),
//       targetSelectedKeys: selectedKeys.filter(key => targetKeys.indexOf(key) > -1),
//     };
//   }

//   setStateKeys = (
//     direction: TransferDirection,
//     keys: string[] | ((prevKeys: string[]) => string[]),
//   ) => {
//     if (direction === 'left') {
//       this.setState(({ sourceSelectedKeys }) => ({
//         sourceSelectedKeys: typeof keys === 'function' ? keys(sourceSelectedKeys || []) : keys,
//       }));
//     } else {
//       this.setState(({ targetSelectedKeys }) => ({
//         targetSelectedKeys: typeof keys === 'function' ? keys(targetSelectedKeys || []) : keys,
//       }));
//     }
//   };

//   getTitles(transferLocale: TransferLocale): React.ReactNode[] {
//     return this.props.titles ?? transferLocale.titles;
//   }

//   getLocale = (transferLocale: TransferLocale, renderEmpty: RenderEmptyHandler) => ({
//     ...transferLocale,
//     notFoundContent: renderEmpty('Transfer'),
//     ...this.props.locale,
//   });

//   moveTo = (direction: TransferDirection) => {
//     const { targetKeys = [], dataSource = [], onChange } = this.props;
//     const { sourceSelectedKeys, targetSelectedKeys } = this.state;
//     const moveKeys = direction === 'right' ? sourceSelectedKeys : targetSelectedKeys;
//     // filter the disabled options
//     const newMoveKeys = moveKeys.filter(
//       (key: string) => !dataSource.some(data => !!(key === data.key && data.disabled)),
//     );
//     // move items to target box
//     const newTargetKeys =
//       direction === 'right'
//         ? newMoveKeys.concat(targetKeys)
//         : targetKeys.filter(targetKey => newMoveKeys.indexOf(targetKey) === -1);

//     // empty checked keys
//     const oppositeDirection = direction === 'right' ? 'left' : 'right';
//     this.setStateKeys(oppositeDirection, []);
//     this.handleSelectChange(oppositeDirection, []);

//     onChange?.(newTargetKeys, direction, newMoveKeys);
//   };

//   moveToLeft = () => this.moveTo('left');

//   moveToRight = () => this.moveTo('right');

//   onItemSelectAll = (direction: TransferDirection, selectedKeys: string[], checkAll: boolean) => {
//     this.setStateKeys(direction, prevKeys => {
//       let mergedCheckedKeys = [];
//       if (checkAll) {
//         // Merge current keys with origin key
//         mergedCheckedKeys = Array.from(new Set([...prevKeys, ...selectedKeys]));
//       } else {
//         // Remove current keys from origin keys
//         mergedCheckedKeys = prevKeys.filter((key: string) => selectedKeys.indexOf(key) === -1);
//       }

//       this.handleSelectChange(direction, mergedCheckedKeys);

//       return mergedCheckedKeys;
//     });
//   };

//   onLeftItemSelectAll = (selectedKeys: string[], checkAll: boolean) =>
//     this.onItemSelectAll('left', selectedKeys, checkAll);

//   onRightItemSelectAll = (selectedKeys: string[], checkAll: boolean) =>
//     this.onItemSelectAll('right', selectedKeys, checkAll);

//   handleFilter = (direction: TransferDirection, e: React.ChangeEvent<HTMLInputElement>) => {
//     const { onSearch } = this.props;
//     const { value } = e.target;
//     onSearch?.(direction, value);
//   };

//   handleLeftFilter = (e: React.ChangeEvent<HTMLInputElement>) => this.handleFilter('left', e);

//   handleRightFilter = (e: React.ChangeEvent<HTMLInputElement>) => this.handleFilter('right', e);

//   handleClear = (direction: TransferDirection) => {
//     const { onSearch } = this.props;
//     onSearch?.(direction, '');
//   };

//   handleLeftClear = () => this.handleClear('left');

//   handleRightClear = () => this.handleClear('right');

//   onItemSelect = (direction: TransferDirection, selectedKey: string, checked: boolean) => {
//     const { sourceSelectedKeys, targetSelectedKeys } = this.state;
//     const holder = direction === 'left' ? [...sourceSelectedKeys] : [...targetSelectedKeys];
//     const index = holder.indexOf(selectedKey);
//     if (index > -1) {
//       holder.splice(index, 1);
//     }
//     if (checked) {
//       holder.push(selectedKey);
//     }
//     this.handleSelectChange(direction, holder);

//     if (!this.props.selectedKeys) {
//       this.setStateKeys(direction, holder);
//     }
//   };

//   onLeftItemSelect = (selectedKey: string, checked: boolean) =>
//     this.onItemSelect('left', selectedKey, checked);

//   onRightItemSelect = (selectedKey: string, checked: boolean) =>
//     this.onItemSelect('right', selectedKey, checked);

//   onRightItemRemove = (selectedKeys: string[]) => {
//     const { targetKeys = [], onChange } = this.props;

//     this.setStateKeys('right', []);

//     onChange?.(
//       targetKeys.filter(key => !selectedKeys.includes(key)),
//       'left',
//       [...selectedKeys],
//     );
//   };

//   handleScroll = (direction: TransferDirection, e: React.SyntheticEvent<HTMLUListElement>) => {
//     const { onScroll } = this.props;
//     onScroll?.(direction, e);
//   };

//   handleLeftScroll = (e: React.SyntheticEvent<HTMLUListElement>) => this.handleScroll('left', e);

//   handleRightScroll = (e: React.SyntheticEvent<HTMLUListElement>) => this.handleScroll('right', e);

//   handleSelectChange(direction: TransferDirection, holder: string[]) {
//     const { sourceSelectedKeys, targetSelectedKeys } = this.state;
//     const { onSelectChange } = this.props;
//     if (!onSelectChange) {
//       return;
//     }

//     if (direction === 'left') {
//       onSelectChange(holder, targetSelectedKeys);
//     } else {
//       onSelectChange(sourceSelectedKeys, holder);
//     }
//   }

//   // eslint-disable-next-line class-methods-use-this
//   handleListStyle = (
//     listStyle: TransferProps<RecordType>['listStyle'],
//     direction: TransferDirection,
//   ) => {
//     if (typeof listStyle === 'function') {
//       return listStyle({ direction });
//     }
//     return listStyle;
//   };

//   separateDataSource() {
//     const { dataSource, rowKey, targetKeys = [] } = this.props;

//     const leftDataSource: KeyWise<RecordType>[] = [];
//     const rightDataSource: KeyWise<RecordType>[] = new Array(targetKeys.length);
//     dataSource.forEach((record: KeyWise<RecordType>) => {
//       if (rowKey) {
//         record = {
//           ...record,
//           key: rowKey(record),
//         };
//       }

//       // rightDataSource should be ordered by targetKeys
//       // leftDataSource should be ordered by dataSource
//       const indexOfKey = targetKeys.indexOf(record.key);
//       if (indexOfKey !== -1) {
//         rightDataSource[indexOfKey] = record;
//       } else {
//         leftDataSource.push(record);
//       }
//     });

//     return {
//       leftDataSource,
//       rightDataSource,
//     };
//   }

//   renderTransfer = (transferLocale: TransferLocale) => (
//     <ConfigConsumer>
//       {({ getPrefixCls, renderEmpty, direction }: ConfigConsumerProps) => (
//         <FormItemInputContext.Consumer>
//           {({ hasFeedback, status: contextStatus }) => {
//             const {
//               prefixCls: customizePrefixCls,
//               className,
//               disabled,
//               operations = [],
//               showSearch,
//               footer,
//               style,
//               listStyle,
//               operationStyle,
//               filterOption,
//               render,
//               children,
//               showSelectAll,
//               oneWay,
//               pagination,
//               status: customStatus,
//             } = this.props;
//             const prefixCls = getPrefixCls('transfer', customizePrefixCls);
//             const locale = this.getLocale(transferLocale, renderEmpty || defaultRenderEmpty);
//             const { sourceSelectedKeys, targetSelectedKeys } = this.state;
//             const mergedStatus = getMergedStatus(contextStatus, customStatus);

//             const mergedPagination = !children && pagination;

//             const { leftDataSource, rightDataSource } = this.separateDataSource();
//             const leftActive = targetSelectedKeys.length > 0;
//             const rightActive = sourceSelectedKeys.length > 0;

//             const cls = classNames(
//               prefixCls,
//               {
//                 [`${prefixCls}-disabled`]: disabled,
//                 [`${prefixCls}-customize-list`]: !!children,
//                 [`${prefixCls}-rtl`]: direction === 'rtl',
//               },
//               getStatusClassNames(prefixCls, mergedStatus, hasFeedback),
//               className,
//             );

//             const titles = this.getTitles(locale);
//             const selectAllLabels = this.props.selectAllLabels || [];
//             return (
//               <div className={cls} style={style}>
//                 <List<KeyWise<RecordType>>
//                   prefixCls={`${prefixCls}-list`}
//                   titleText={titles[0]}
//                   dataSource={leftDataSource}
//                   filterOption={filterOption}
//                   style={this.handleListStyle(listStyle, 'left')}
//                   checkedKeys={sourceSelectedKeys}
//                   handleFilter={this.handleLeftFilter}
//                   handleClear={this.handleLeftClear}
//                   onItemSelect={this.onLeftItemSelect}
//                   onItemSelectAll={this.onLeftItemSelectAll}
//                   render={render}
//                   showSearch={showSearch}
//                   renderList={children}
//                   footer={footer}
//                   onScroll={this.handleLeftScroll}
//                   disabled={disabled}
//                   direction={direction === 'rtl' ? 'right' : 'left'}
//                   showSelectAll={showSelectAll}
//                   selectAllLabel={selectAllLabels[0]}
//                   pagination={mergedPagination}
//                   {...locale}
//                 />
//                 <Operation
//                   className={`${prefixCls}-operation`}
//                   rightActive={rightActive}
//                   rightArrowText={operations[0]}
//                   moveToRight={this.moveToRight}
//                   leftActive={leftActive}
//                   leftArrowText={operations[1]}
//                   moveToLeft={this.moveToLeft}
//                   style={operationStyle}
//                   disabled={disabled}
//                   direction={direction}
//                   oneWay={oneWay}
//                 />
//                 <List<KeyWise<RecordType>>
//                   prefixCls={`${prefixCls}-list`}
//                   titleText={titles[1]}
//                   dataSource={rightDataSource}
//                   filterOption={filterOption}
//                   style={this.handleListStyle(listStyle, 'right')}
//                   checkedKeys={targetSelectedKeys}
//                   handleFilter={this.handleRightFilter}
//                   handleClear={this.handleRightClear}
//                   onItemSelect={this.onRightItemSelect}
//                   onItemSelectAll={this.onRightItemSelectAll}
//                   onItemRemove={this.onRightItemRemove}
//                   render={render}
//                   showSearch={showSearch}
//                   renderList={children}
//                   footer={footer}
//                   onScroll={this.handleRightScroll}
//                   disabled={disabled}
//                   direction={direction === 'rtl' ? 'left' : 'right'}
//                   showSelectAll={showSelectAll}
//                   selectAllLabel={selectAllLabels[1]}
//                   showRemove={oneWay}
//                   pagination={mergedPagination}
//                   {...locale}
//                 />
//               </div>
//             );
//           }}
//         </FormItemInputContext.Consumer>
//       )}
//     </ConfigConsumer>
//   );

//   render() {
//     return (
//       <LocaleReceiver componentName="Transfer" defaultLocale={defaultLocale.Transfer}>
//         {this.renderTransfer}
//       </LocaleReceiver>
//     );
//   }
// }

// export default Transfer;

// export type PaginationType =
//   | boolean
//   | {
//       pageSize?: number;
//       simple?: boolean;
//       showSizeChanger?: boolean;
//       showLessItems?: boolean;
//     };

// import DownOutlined from '@ant-design/icons/DownOutlined';
// import classNames from 'classnames';
// import omit from 'rc-util/lib/omit';
// import * as React from 'react';
// import Checkbox from '../checkbox';
// import Dropdown from '../dropdown';
// import Menu from '../menu';
// import { isValidElement } from '../_util/reactNode';
// import type {
//   KeyWiseTransferItem,
//   RenderResult,
//   RenderResultObject,
//   SelectAllLabel,
//   TransferDirection,
//   TransferLocale,
// } from './index';
// import type { PaginationType } from './interface';
// import type { TransferListBodyProps } from './ListBody';
// import DefaultListBody, { OmitProps } from './ListBody';
// import Search from './search';

// const defaultRender = () => null;

// function isRenderResultPlainObject(result: RenderResult): result is RenderResultObject {
//   return !!(
//     result &&
//     !isValidElement(result) &&
//     Object.prototype.toString.call(result) === '[object Object]'
//   );
// }

// function getEnabledItemKeys<RecordType extends KeyWiseTransferItem>(items: RecordType[]) {
//   return items.filter(data => !data.disabled).map(data => data.key);
// }

// export interface RenderedItem<RecordType> {
//   renderedText: string;
//   renderedEl: React.ReactNode;
//   item: RecordType;
// }

// type RenderListFunction<T> = (props: TransferListBodyProps<T>) => React.ReactNode;

// export interface TransferListProps<RecordType> extends TransferLocale {
//   prefixCls: string;
//   titleText: React.ReactNode;
//   dataSource: RecordType[];
//   filterOption?: (filterText: string, item: RecordType) => boolean;
//   style?: React.CSSProperties;
//   checkedKeys: string[];
//   handleFilter: (e: React.ChangeEvent<HTMLInputElement>) => void;
//   onItemSelect: (key: string, check: boolean) => void;
//   onItemSelectAll: (dataSource: string[], checkAll: boolean) => void;
//   onItemRemove?: (keys: string[]) => void;
//   handleClear: () => void;
//   /** Render item */
//   render?: (item: RecordType) => RenderResult;
//   showSearch?: boolean;
//   searchPlaceholder: string;
//   itemUnit: string;
//   itemsUnit: string;
//   renderList?: RenderListFunction<RecordType>;
//   footer?: (
//     props: TransferListProps<RecordType>,
//     info?: { direction: TransferDirection },
//   ) => React.ReactNode;
//   onScroll: (e: React.UIEvent<HTMLUListElement>) => void;
//   disabled?: boolean;
//   direction: TransferDirection;
//   showSelectAll?: boolean;
//   selectAllLabel?: SelectAllLabel;
//   showRemove?: boolean;
//   pagination?: PaginationType;
// }

// interface TransferListState {
//   /** Filter input value */
//   filterValue: string;
// }

// export default class TransferList<
//   RecordType extends KeyWiseTransferItem,
// > extends React.PureComponent<TransferListProps<RecordType>, TransferListState> {
//   static defaultProps = {
//     dataSource: [],
//     titleText: '',
//     showSearch: false,
//   };

//   timer: number;

//   triggerScrollTimer: number;

//   defaultListBodyRef = React.createRef<DefaultListBody<RecordType>>();

//   constructor(props: TransferListProps<RecordType>) {
//     super(props);
//     this.state = {
//       filterValue: '',
//     };
//   }

//   componentWillUnmount() {
//     clearTimeout(this.triggerScrollTimer);
//   }

//   getCheckStatus(filteredItems: RecordType[]) {
//     const { checkedKeys } = this.props;
//     if (checkedKeys.length === 0) {
//       return 'none';
//     }
//     if (filteredItems.every(item => checkedKeys.indexOf(item.key) >= 0 || !!item.disabled)) {
//       return 'all';
//     }
//     return 'part';
//   }

//   // ================================ Item ================================
//   getFilteredItems(
//     dataSource: RecordType[],
//     filterValue: string,
//   ): { filteredItems: RecordType[]; filteredRenderItems: RenderedItem<RecordType>[] } {
//     const filteredItems: RecordType[] = [];
//     const filteredRenderItems: RenderedItem<RecordType>[] = [];

//     dataSource.forEach(item => {
//       const renderedItem = this.renderItem(item);
//       const { renderedText } = renderedItem;

//       // Filter skip
//       if (filterValue && !this.matchFilter(renderedText, item)) {
//         return null;
//       }

//       filteredItems.push(item);
//       filteredRenderItems.push(renderedItem);
//     });

//     return { filteredItems, filteredRenderItems };
//   }

//   // =============================== Filter ===============================
//   handleFilter = (e: React.ChangeEvent<HTMLInputElement>) => {
//     const { handleFilter } = this.props;
//     const {
//       target: { value: filterValue },
//     } = e;
//     this.setState({ filterValue });
//     handleFilter(e);
//   };

//   handleClear = () => {
//     const { handleClear } = this.props;
//     this.setState({ filterValue: '' });
//     handleClear();
//   };

//   matchFilter = (text: string, item: RecordType) => {
//     const { filterValue } = this.state;
//     const { filterOption } = this.props;
//     if (filterOption) {
//       return filterOption(filterValue, item);
//     }
//     return text.indexOf(filterValue) >= 0;
//   };

//   // =============================== Render ===============================
//   renderListBody = (
//     renderList: RenderListFunction<RecordType> | undefined,
//     props: TransferListBodyProps<RecordType>,
//   ) => {
//     let bodyContent: React.ReactNode = renderList ? renderList(props) : null;
//     const customize: boolean = !!bodyContent;
//     if (!customize) {
//       bodyContent = <DefaultListBody ref={this.defaultListBodyRef} {...props} />;
//     }
//     return {
//       customize,
//       bodyContent,
//     };
//   };

//   getListBody(
//     prefixCls: string,
//     searchPlaceholder: string,
//     filterValue: string,
//     filteredItems: RecordType[],
//     notFoundContent: React.ReactNode | React.ReactNode,
//     filteredRenderItems: RenderedItem<RecordType>[],
//     checkedKeys: string[],
//     renderList?: RenderListFunction<RecordType>,
//     showSearch?: boolean,
//     disabled?: boolean,
//   ): React.ReactNode {
//     const search = showSearch ? (
//       <div className={`${prefixCls}-body-search-wrapper`}>
//         <Search
//           prefixCls={`${prefixCls}-search`}
//           onChange={this.handleFilter}
//           handleClear={this.handleClear}
//           placeholder={searchPlaceholder}
//           value={filterValue}
//           disabled={disabled}
//         />
//       </div>
//     ) : null;

//     const { bodyContent, customize } = this.renderListBody(renderList, {
//       ...omit(this.props, OmitProps),
//       filteredItems,
//       filteredRenderItems,
//       selectedKeys: checkedKeys,
//     });

//     const getNotFoundContent = () => {
//       const contentIndex = this.props.direction === 'left' ? 0 : 1;
//       return Array.isArray(notFoundContent) ? notFoundContent[contentIndex] : notFoundContent;
//     };

//     let bodyNode: React.ReactNode;
//     // We should wrap customize list body in a classNamed div to use flex layout.
//     if (customize) {
//       bodyNode = <div className={`${prefixCls}-body-customize-wrapper`}>{bodyContent}</div>;
//     } else {
//       bodyNode = filteredItems.length ? (
//         bodyContent
//       ) : (
//         <div className={`${prefixCls}-body-not-found`}>{getNotFoundContent()}</div>
//       );
//     }

//     return (
//       <div
//         className={classNames(
//           showSearch ? `${prefixCls}-body ${prefixCls}-body-with-search` : `${prefixCls}-body`,
//         )}
//       >
//         {search}
//         {bodyNode}
//       </div>
//     );
//   }

//   getCheckBox({
//     filteredItems,
//     onItemSelectAll,
//     disabled,
//     prefixCls,
//   }: {
//     filteredItems: RecordType[];
//     onItemSelectAll: (dataSource: string[], checkAll: boolean) => void;
//     disabled?: boolean;
//     prefixCls?: string;
//   }): false | JSX.Element {
//     const checkStatus = this.getCheckStatus(filteredItems);
//     const checkedAll = checkStatus === 'all';
//     const checkAllCheckbox = (
//       <Checkbox
//         disabled={disabled}
//         checked={checkedAll}
//         indeterminate={checkStatus === 'part'}
//         className={`${prefixCls}-checkbox`}
//         onChange={() => {
//           // Only select enabled items
//           onItemSelectAll(
//             filteredItems.filter(item => !item.disabled).map(({ key }) => key),
//             !checkedAll,
//           );
//         }}
//       />
//     );

//     return checkAllCheckbox;
//   }

//   renderItem = (item: RecordType): RenderedItem<RecordType> => {
//     const { render = defaultRender } = this.props;
//     const renderResult = render(item);
//     const isRenderResultPlain = isRenderResultPlainObject(renderResult);
//     return {
//       renderedText: isRenderResultPlain
//         ? (renderResult as RenderResultObject).value
//         : (renderResult as string),
//       renderedEl: isRenderResultPlain ? (renderResult as RenderResultObject).label : renderResult,
//       item,
//     };
//   };

//   getSelectAllLabel = (selectedCount: number, totalCount: number): React.ReactNode => {
//     const { itemsUnit, itemUnit, selectAllLabel } = this.props;
//     if (selectAllLabel) {
//       return typeof selectAllLabel === 'function'
//         ? selectAllLabel({ selectedCount, totalCount })
//         : selectAllLabel;
//     }
//     const unit = totalCount > 1 ? itemsUnit : itemUnit;
//     return (
//       <>
//         {(selectedCount > 0 ? `${selectedCount}/` : '') + totalCount} {unit}
//       </>
//     );
//   };

//   render() {
//     const { filterValue } = this.state;
//     const {
//       prefixCls,
//       dataSource,
//       titleText,
//       checkedKeys,
//       disabled,
//       footer,
//       showSearch,
//       style,
//       searchPlaceholder,
//       notFoundContent,
//       selectAll,
//       selectCurrent,
//       selectInvert,
//       removeAll,
//       removeCurrent,
//       renderList,
//       onItemSelectAll,
//       onItemRemove,
//       showSelectAll = true,
//       showRemove,
//       pagination,
//       direction,
//     } = this.props;

//     // Custom Layout
//     const footerDom =
//       footer && (footer.length < 2 ? footer(this.props) : footer(this.props, { direction }));

//     const listCls = classNames(prefixCls, {
//       [`${prefixCls}-with-pagination`]: !!pagination,
//       [`${prefixCls}-with-footer`]: !!footerDom,
//     });

//     // ====================== Get filtered, checked item list ======================

//     const { filteredItems, filteredRenderItems } = this.getFilteredItems(dataSource, filterValue);

//     // ================================= List Body =================================

//     const listBody = this.getListBody(
//       prefixCls,
//       searchPlaceholder,
//       filterValue,
//       filteredItems,
//       notFoundContent,
//       filteredRenderItems,
//       checkedKeys,
//       renderList,
//       showSearch,
//       disabled,
//     );

//     // ================================ List Footer ================================
//     const listFooter = footerDom ? <div className={`${prefixCls}-footer`}>{footerDom}</div> : null;

//     const checkAllCheckbox =
//       !showRemove &&
//       !pagination &&
//       this.getCheckBox({ filteredItems, onItemSelectAll, disabled, prefixCls });

//     let menu: React.ReactElement | null = null;
//     if (showRemove) {
//       const items = [
//         /* Remove Current Page */
//         pagination
//           ? {
//               key: 'removeCurrent',
//               onClick: () => {
//                 const pageKeys = getEnabledItemKeys(
//                   (this.defaultListBodyRef.current?.getItems() || []).map(entity => entity.item),
//                 );
//                 onItemRemove?.(pageKeys);
//               },
//               label: removeCurrent,
//             }
//           : null,

//         /* Remove All */
//         {
//           key: 'removeAll',
//           onClick: () => {
//             onItemRemove?.(getEnabledItemKeys(filteredItems));
//           },
//           label: removeAll,
//         },
//       ].filter(item => item);

//       menu = <Menu items={items} />;
//     } else {
//       const items = [
//         {
//           key: 'selectAll',
//           onClick: () => {
//             const keys = getEnabledItemKeys(filteredItems);
//             onItemSelectAll(keys, keys.length !== checkedKeys.length);
//           },
//           label: selectAll,
//         },
//         pagination
//           ? {
//               key: 'selectCurrent',
//               onClick: () => {
//                 const pageItems = this.defaultListBodyRef.current?.getItems() || [];
//                 onItemSelectAll(getEnabledItemKeys(pageItems.map(entity => entity.item)), true);
//               },
//               label: selectCurrent,
//             }
//           : null,

//         {
//           key: 'selectInvert',
//           onClick: () => {
//             let availableKeys: string[];
//             if (pagination) {
//               availableKeys = getEnabledItemKeys(
//                 (this.defaultListBodyRef.current?.getItems() || []).map(entity => entity.item),
//               );
//             } else {
//               availableKeys = getEnabledItemKeys(filteredItems);
//             }

//             const checkedKeySet = new Set(checkedKeys);
//             const newCheckedKeys: string[] = [];
//             const newUnCheckedKeys: string[] = [];

//             availableKeys.forEach(key => {
//               if (checkedKeySet.has(key)) {
//                 newUnCheckedKeys.push(key);
//               } else {
//                 newCheckedKeys.push(key);
//               }
//             });

//             onItemSelectAll(newCheckedKeys, true);
//             onItemSelectAll(newUnCheckedKeys, false);
//           },
//           label: selectInvert,
//         },
//       ];

//       menu = <Menu items={items} />;
//     }

//     const dropdown = (
//       <Dropdown className={`${prefixCls}-header-dropdown`} overlay={menu} disabled={disabled}>
//         <DownOutlined />
//       </Dropdown>
//     );

//     // ================================== Render ===================================
//     return (
//       <div className={listCls} style={style}>
//         {/* Header */}
//         <div className={`${prefixCls}-header`}>
//           {showSelectAll ? (
//             <>
//               {checkAllCheckbox}
//               {dropdown}
//             </>
//           ) : null}
//           <span className={`${prefixCls}-header-selected`}>
//             {this.getSelectAllLabel(checkedKeys.length, filteredItems.length)}
//           </span>

//           <span className={`${prefixCls}-header-title`}>{titleText}</span>
//         </div>

//         {/* Body */}
//         {listBody}

//         {/* Footer */}
//         {listFooter}
//       </div>
//     );
//   }
// }

// import LeftOutlined from '@ant-design/icons/LeftOutlined';
// import RightOutlined from '@ant-design/icons/RightOutlined';
// import * as React from 'react';
// import Button from '../button';
// import type { DirectionType } from '../config-provider';

// export interface TransferOperationProps {
//   className?: string;
//   leftArrowText?: string;
//   rightArrowText?: string;
//   moveToLeft?: React.MouseEventHandler<HTMLButtonElement>;
//   moveToRight?: React.MouseEventHandler<HTMLButtonElement>;
//   leftActive?: boolean;
//   rightActive?: boolean;
//   style?: React.CSSProperties;
//   disabled?: boolean;
//   direction?: DirectionType;
//   oneWay?: boolean;
// }

// const Operation = ({
//   disabled,
//   moveToLeft,
//   moveToRight,
//   leftArrowText = '',
//   rightArrowText = '',
//   leftActive,
//   rightActive,
//   className,
//   style,
//   direction,
//   oneWay,
// }: TransferOperationProps) => (
//   <div className={className} style={style}>
//     <Button
//       type="primary"
//       size="small"
//       disabled={disabled || !rightActive}
//       onClick={moveToRight}
//       icon={direction !== 'rtl' ? <RightOutlined /> : <LeftOutlined />}
//     >
//       {rightArrowText}
//     </Button>
//     {!oneWay && (
//       <Button
//         type="primary"
//         size="small"
//         disabled={disabled || !leftActive}
//         onClick={moveToLeft}
//         icon={direction !== 'rtl' ? <LeftOutlined /> : <RightOutlined />}
//       >
//         {leftArrowText}
//       </Button>
//     )}
//   </div>
// );

// export default Operation;

// import SearchOutlined from '@ant-design/icons/SearchOutlined';
// import * as React from 'react';

// import Input from '../input';

// export interface TransferSearchProps {
//   prefixCls?: string;
//   placeholder?: string;
//   onChange?: (e: React.FormEvent<HTMLElement>) => void;
//   handleClear?: () => void;
//   value?: string;
//   disabled?: boolean;
// }

// export default function Search(props: TransferSearchProps) {
//   const { placeholder = '', value, prefixCls, disabled, onChange, handleClear } = props;

//   const handleChange = React.useCallback(
//     (e: React.ChangeEvent<HTMLInputElement>) => {
//       onChange?.(e);
//       if (e.target.value === '') {
//         handleClear?.();
//       }
//     },
//     [onChange],
//   );

//   return (
//     <Input
//       placeholder={placeholder}
//       className={prefixCls}
//       value={value}
//       onChange={handleChange}
//       disabled={disabled}
//       allowClear
//       prefix={<SearchOutlined />}
//     />
//   );
// }

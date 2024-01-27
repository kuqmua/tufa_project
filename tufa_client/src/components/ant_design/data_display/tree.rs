// import FileOutlined from '@ant-design/icons/FileOutlined';
// import FolderOpenOutlined from '@ant-design/icons/FolderOpenOutlined';
// import FolderOutlined from '@ant-design/icons/FolderOutlined';
// import classNames from 'classnames';
// import type RcTree from 'rc-tree';
// import type { BasicDataNode } from 'rc-tree';
// import type { DataNode, EventDataNode, Key } from 'rc-tree/lib/interface';
// import { conductExpandParent } from 'rc-tree/lib/util';
// import { convertDataToEntities, convertTreeToData } from 'rc-tree/lib/utils/treeUtil';
// import * as React from 'react';
// import { ConfigContext } from '../config-provider';

// import type { AntdTreeNodeAttribute, TreeProps } from './Tree';
// import Tree from './Tree';
// import { calcRangeKeys, convertDirectoryKeysToNodes } from './utils/dictUtil';

// export type ExpandAction = false | 'click' | 'doubleClick';

// export interface DirectoryTreeProps<T extends BasicDataNode = DataNode> extends TreeProps<T> {
//   expandAction?: ExpandAction;
// }

// type DirectoryTreeCompoundedComponent = (<T extends BasicDataNode | DataNode = DataNode>(
//   props: React.PropsWithChildren<DirectoryTreeProps<T>> & { ref?: React.Ref<RcTree> },
// ) => React.ReactElement) & {
//   defaultProps: Partial<React.PropsWithChildren<DirectoryTreeProps<any>>>;
//   displayName?: string;
// };

// export interface DirectoryTreeState {
//   expandedKeys?: Key[];
//   selectedKeys?: Key[];
// }

// function getIcon(props: AntdTreeNodeAttribute): React.ReactNode {
//   const { isLeaf, expanded } = props;
//   if (isLeaf) {
//     return <FileOutlined />;
//   }
//   return expanded ? <FolderOpenOutlined /> : <FolderOutlined />;
// }

// function getTreeData({ treeData, children }: DirectoryTreeProps) {
//   return treeData || convertTreeToData(children);
// }

// const DirectoryTree: React.ForwardRefRenderFunction<RcTree, DirectoryTreeProps> = (
//   { defaultExpandAll, defaultExpandParent, defaultExpandedKeys, ...props },
//   ref,
// ) => {
//   // Shift click usage
//   const lastSelectedKey = React.useRef<Key>();

//   const cachedSelectedKeys = React.useRef<Key[]>();

//   const treeRef = React.createRef<RcTree>();

//   React.useImperativeHandle(ref, () => treeRef.current!);

//   const getInitExpandedKeys = () => {
//     const { keyEntities } = convertDataToEntities(getTreeData(props));

//     let initExpandedKeys: any;

//     // Expanded keys
//     if (defaultExpandAll) {
//       initExpandedKeys = Object.keys(keyEntities);
//     } else if (defaultExpandParent) {
//       initExpandedKeys = conductExpandParent(
//         props.expandedKeys || defaultExpandedKeys || [],
//         keyEntities,
//       );
//     } else {
//       initExpandedKeys = props.expandedKeys || defaultExpandedKeys;
//     }
//     return initExpandedKeys;
//   };

//   const [selectedKeys, setSelectedKeys] = React.useState(
//     props.selectedKeys || props.defaultSelectedKeys || [],
//   );
//   const [expandedKeys, setExpandedKeys] = React.useState(getInitExpandedKeys());

//   React.useEffect(() => {
//     if ('selectedKeys' in props) {
//       setSelectedKeys(props.selectedKeys!);
//     }
//   }, [props.selectedKeys]);

//   React.useEffect(() => {
//     if ('expandedKeys' in props) {
//       setExpandedKeys(props.expandedKeys);
//     }
//   }, [props.expandedKeys]);

//   const onExpand = (
//     keys: Key[],
//     info: {
//       node: EventDataNode<any>;
//       expanded: boolean;
//       nativeEvent: MouseEvent;
//     },
//   ) => {
//     if (!('expandedKeys' in props)) {
//       setExpandedKeys(keys);
//     }
//     // Call origin function
//     return props.onExpand?.(keys, info);
//   };

//   const onSelect = (
//     keys: Key[],
//     event: {
//       event: 'select';
//       selected: boolean;
//       node: any;
//       selectedNodes: DataNode[];
//       nativeEvent: MouseEvent;
//     },
//   ) => {
//     const { multiple } = props;
//     const { node, nativeEvent } = event;
//     const { key = '' } = node;

//     const treeData = getTreeData(props);
//     // const newState: DirectoryTreeState = {};

//     // We need wrap this event since some value is not same
//     const newEvent: any = {
//       ...event,
//       selected: true, // Directory selected always true
//     };

//     // Windows / Mac single pick
//     const ctrlPick: boolean = nativeEvent?.ctrlKey || nativeEvent?.metaKey;
//     const shiftPick: boolean = nativeEvent?.shiftKey;

//     // Generate new selected keys
//     let newSelectedKeys: Key[];
//     if (multiple && ctrlPick) {
//       // Control click
//       newSelectedKeys = keys;
//       lastSelectedKey.current = key;
//       cachedSelectedKeys.current = newSelectedKeys;
//       newEvent.selectedNodes = convertDirectoryKeysToNodes(treeData, newSelectedKeys);
//     } else if (multiple && shiftPick) {
//       // Shift click
//       newSelectedKeys = Array.from(
//         new Set([
//           ...(cachedSelectedKeys.current || []),
//           ...calcRangeKeys({
//             treeData,
//             expandedKeys,
//             startKey: key,
//             endKey: lastSelectedKey.current,
//           }),
//         ]),
//       );
//       newEvent.selectedNodes = convertDirectoryKeysToNodes(treeData, newSelectedKeys);
//     } else {
//       // Single click
//       newSelectedKeys = [key];
//       lastSelectedKey.current = key;
//       cachedSelectedKeys.current = newSelectedKeys;
//       newEvent.selectedNodes = convertDirectoryKeysToNodes(treeData, newSelectedKeys);
//     }

//     props.onSelect?.(newSelectedKeys, newEvent);
//     if (!('selectedKeys' in props)) {
//       setSelectedKeys(newSelectedKeys);
//     }
//   };
//   const { getPrefixCls, direction } = React.useContext(ConfigContext);

//   const { prefixCls: customizePrefixCls, className, ...otherProps } = props;

//   const prefixCls = getPrefixCls('tree', customizePrefixCls);
//   const connectClassName = classNames(
//     `${prefixCls}-directory`,
//     {
//       [`${prefixCls}-directory-rtl`]: direction === 'rtl',
//     },
//     className,
//   );

//   return (
//     <Tree
//       icon={getIcon}
//       ref={treeRef}
//       blockNode
//       {...otherProps}
//       prefixCls={prefixCls}
//       className={connectClassName}
//       expandedKeys={expandedKeys}
//       selectedKeys={selectedKeys}
//       onSelect={onSelect}
//       onExpand={onExpand}
//     />
//   );
// };

// const ForwardDirectoryTree = React.forwardRef(
//   DirectoryTree,
// ) as unknown as DirectoryTreeCompoundedComponent;

// if (process.env.NODE_ENV !== 'production') {
//   ForwardDirectoryTree.displayName = 'DirectoryTree';
// }

// ForwardDirectoryTree.defaultProps = {
//   showIcon: true,
//   expandAction: 'click' as DirectoryTreeProps['expandAction'],
// };

// export default ForwardDirectoryTree;

// import HolderOutlined from '@ant-design/icons/HolderOutlined';
// import classNames from 'classnames';
// import type { BasicDataNode, TreeProps as RcTreeProps } from 'rc-tree';
// import RcTree, { TreeNode } from 'rc-tree';
// import type { DataNode, Key } from 'rc-tree/lib/interface';
// import * as React from 'react';
// import { ConfigContext } from '../config-provider';
// import collapseMotion from '../_util/motion';
// import DirectoryTree from './DirectoryTree';
// import dropIndicatorRender from './utils/dropIndicator';
// import renderSwitcherIcon from './utils/iconUtil';

// export type SwitcherIcon = React.ReactNode | ((props: { expanded: boolean }) => React.ReactNode);

// export interface AntdTreeNodeAttribute {
//   eventKey: string;
//   prefixCls: string;
//   className: string;
//   expanded: boolean;
//   selected: boolean;
//   checked: boolean;
//   halfChecked: boolean;
//   children: React.ReactNode;
//   title: React.ReactNode;
//   pos: string;
//   dragOver: boolean;
//   dragOverGapTop: boolean;
//   dragOverGapBottom: boolean;
//   isLeaf: boolean;
//   selectable: boolean;
//   disabled: boolean;
//   disableCheckbox: boolean;
// }

// export interface AntTreeNodeProps {
//   className?: string;
//   checkable?: boolean;
//   disabled?: boolean;
//   disableCheckbox?: boolean;
//   title?: string | React.ReactNode;
//   key?: Key;
//   eventKey?: string;
//   isLeaf?: boolean;
//   checked?: boolean;
//   expanded?: boolean;
//   loading?: boolean;
//   selected?: boolean;
//   selectable?: boolean;
//   icon?: ((treeNode: AntdTreeNodeAttribute) => React.ReactNode) | React.ReactNode;
//   children?: React.ReactNode;
//   [customProp: string]: any;
// }

// export interface AntTreeNode extends React.Component<AntTreeNodeProps, {}> {}

// export interface AntTreeNodeBaseEvent {
//   node: AntTreeNode;
//   nativeEvent: MouseEvent;
// }

// export interface AntTreeNodeCheckedEvent extends AntTreeNodeBaseEvent {
//   event: 'check';
//   checked?: boolean;
//   checkedNodes?: AntTreeNode[];
// }

// export interface AntTreeNodeSelectedEvent extends AntTreeNodeBaseEvent {
//   event: 'select';
//   selected?: boolean;
//   selectedNodes?: DataNode[];
// }

// export interface AntTreeNodeExpandedEvent extends AntTreeNodeBaseEvent {
//   expanded?: boolean;
// }

// export interface AntTreeNodeMouseEvent {
//   node: AntTreeNode;
//   event: React.DragEvent<HTMLElement>;
// }

// export interface AntTreeNodeDragEnterEvent extends AntTreeNodeMouseEvent {
//   expandedKeys: Key[];
// }

// export interface AntTreeNodeDropEvent {
//   node: AntTreeNode;
//   dragNode: AntTreeNode;
//   dragNodesKeys: Key[];
//   dropPosition: number;
//   dropToGap?: boolean;
//   event: React.MouseEvent<HTMLElement>;
// }

// // [Legacy] Compatible for v3
// export type TreeNodeNormal = DataNode;

// type DraggableFn = (node: AntTreeNode) => boolean;
// interface DraggableConfig {
//   icon?: React.ReactNode | false;
//   nodeDraggable?: DraggableFn;
// }

// export interface TreeProps<T extends BasicDataNode = DataNode>
//   extends Omit<
//     RcTreeProps<T>,
//     'prefixCls' | 'showLine' | 'direction' | 'draggable' | 'icon' | 'switcherIcon'
//   > {
//   showLine?: boolean | { showLeafIcon: boolean };
//   className?: string;
//   /** 是否支持多选 */
//   multiple?: boolean;
//   /** 是否自动展开父节点 */
//   autoExpandParent?: boolean;
//   /** Checkable状态下节点选择完全受控（父子节点选中状态不再关联） */
//   checkStrictly?: boolean;
//   /** 是否支持选中 */
//   checkable?: boolean;
//   /** 是否禁用树 */
//   disabled?: boolean;
//   /** 默认展开所有树节点 */
//   defaultExpandAll?: boolean;
//   /** 默认展开对应树节点 */
//   defaultExpandParent?: boolean;
//   /** 默认展开指定的树节点 */
//   defaultExpandedKeys?: Key[];
//   /** （受控）展开指定的树节点 */
//   expandedKeys?: Key[];
//   /** （受控）选中复选框的树节点 */
//   checkedKeys?: Key[] | { checked: Key[]; halfChecked: Key[] };
//   /** 默认选中复选框的树节点 */
//   defaultCheckedKeys?: Key[];
//   /** （受控）设置选中的树节点 */
//   selectedKeys?: Key[];
//   /** 默认选中的树节点 */
//   defaultSelectedKeys?: Key[];
//   selectable?: boolean;
//   /** 点击树节点触发 */
//   filterAntTreeNode?: (node: AntTreeNode) => boolean;
//   loadedKeys?: Key[];
//   /** 设置节点可拖拽（IE>8） */
//   draggable?: DraggableFn | boolean | DraggableConfig;
//   style?: React.CSSProperties;
//   showIcon?: boolean;
//   icon?:
//     | ((nodeProps: AntdTreeNodeAttribute) => React.ReactNode)
//     | React.ReactNode
//     | RcTreeProps<T>['icon'];
//   switcherIcon?: SwitcherIcon | RcTreeProps<T>['switcherIcon'];
//   prefixCls?: string;
//   children?: React.ReactNode;
//   blockNode?: boolean;
// }

// type CompoundedComponent = (<T extends BasicDataNode | DataNode = DataNode>(
//   props: React.PropsWithChildren<TreeProps<T>> & { ref?: React.Ref<RcTree> },
// ) => React.ReactElement) & {
//   defaultProps: Partial<React.PropsWithChildren<TreeProps<any>>>;
//   TreeNode: typeof TreeNode;
//   DirectoryTree: typeof DirectoryTree;
// };

// const Tree = React.forwardRef<RcTree, TreeProps>((props, ref) => {
//   const { getPrefixCls, direction, virtual } = React.useContext(ConfigContext);
//   const {
//     prefixCls: customizePrefixCls,
//     className,
//     showIcon,
//     showLine,
//     switcherIcon,
//     blockNode,
//     children,
//     checkable,
//     selectable,
//     draggable,
//   } = props;
//   const prefixCls = getPrefixCls('tree', customizePrefixCls);
//   const newProps = {
//     ...props,
//     showLine: Boolean(showLine),
//     dropIndicatorRender,
//   };

//   const draggableConfig = React.useMemo(() => {
//     if (!draggable) {
//       return false;
//     }

//     let mergedDraggable: DraggableConfig = {};
//     switch (typeof draggable) {
//       case 'function':
//         mergedDraggable.nodeDraggable = draggable;
//         break;

//       case 'object':
//         mergedDraggable = { ...draggable };
//         break;

//       default:
//       // Do nothing
//     }

//     if (mergedDraggable.icon !== false) {
//       mergedDraggable.icon = mergedDraggable.icon || <HolderOutlined />;
//     }

//     return mergedDraggable;
//   }, [draggable]);

//   return (
//     <RcTree
//       itemHeight={20}
//       ref={ref}
//       virtual={virtual}
//       {...newProps}
//       prefixCls={prefixCls}
//       className={classNames(
//         {
//           [`${prefixCls}-icon-hide`]: !showIcon,
//           [`${prefixCls}-block-node`]: blockNode,
//           [`${prefixCls}-unselectable`]: !selectable,
//           [`${prefixCls}-rtl`]: direction === 'rtl',
//         },
//         className,
//       )}
//       direction={direction}
//       checkable={checkable ? <span className={`${prefixCls}-checkbox-inner`} /> : checkable}
//       selectable={selectable}
//       switcherIcon={(nodeProps: AntTreeNodeProps) =>
//         renderSwitcherIcon(prefixCls, switcherIcon, showLine, nodeProps)
//       }
//       draggable={draggableConfig as any}
//     >
//       {children}
//     </RcTree>
//   );
// }) as unknown as CompoundedComponent;

// Tree.TreeNode = TreeNode;

// Tree.DirectoryTree = DirectoryTree;

// Tree.defaultProps = {
//   checkable: false,
//   selectable: true,
//   showIcon: false,
//   motion: {
//     ...collapseMotion,
//     motionAppear: false,
//   },
//   blockNode: false,
// };

// export default Tree;

// import Tree from './Tree';

// export { DataNode, EventDataNode } from 'rc-tree/lib/interface';
// export { DirectoryTreeProps, ExpandAction as DirectoryTreeExpandAction } from './DirectoryTree';
// export {
//   AntdTreeNodeAttribute,
//   AntTreeNode,
//   AntTreeNodeCheckedEvent,
//   AntTreeNodeExpandedEvent,
//   AntTreeNodeMouseEvent,
//   AntTreeNodeProps,
//   AntTreeNodeSelectedEvent,
//   TreeProps,
// } from './Tree';

// export default Tree;

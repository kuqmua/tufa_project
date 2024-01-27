// import * as React from 'react';
// import type { UploadProps } from './interface';
// import Upload from './Upload';

// export type DraggerProps = UploadProps & { height?: number };

// const InternalDragger: React.ForwardRefRenderFunction<unknown, DraggerProps> = (
//   { style, height, ...restProps },
//   ref,
// ) => <Upload ref={ref} {...restProps} type="drag" style={{ ...style, height }} />;

// const Dragger = React.forwardRef(InternalDragger) as React.FC<DraggerProps>;

// if (process.env.NODE_ENV !== 'production') {
//   Dragger.displayName = 'Dragger';
// }

// export default Dragger;

// import classNames from 'classnames';
// import type { UploadProps as RcUploadProps } from 'rc-upload';
// import RcUpload from 'rc-upload';
// import useMergedState from 'rc-util/lib/hooks/useMergedState';
// import * as React from 'react';
// import { ConfigContext } from '../config-provider';
// import LocaleReceiver from '../locale-provider/LocaleReceiver';
// import defaultLocale from '../locale/default';
// import warning from '../_util/warning';
// import type {
//   RcFile,
//   ShowUploadListInterface,
//   UploadChangeParam,
//   UploadFile,
//   UploadListType,
//   UploadLocale,
//   UploadType,
// } from './interface';
// import { UploadProps } from './interface';
// import UploadList from './UploadList';
// import { file2Obj, getFileItem, removeFileItem, updateFileList } from './utils';

// export const LIST_IGNORE = `__LIST_IGNORE_${Date.now()}__`;

// export { UploadProps };

// const InternalUpload: React.ForwardRefRenderFunction<unknown, UploadProps> = (props, ref) => {
//   const {
//     fileList,
//     defaultFileList,
//     onRemove,
//     showUploadList,
//     listType,
//     onPreview,
//     onDownload,
//     onChange,
//     onDrop,
//     previewFile,
//     disabled,
//     locale: propLocale,
//     iconRender,
//     isImageUrl,
//     progress,
//     prefixCls: customizePrefixCls,
//     className,
//     type,
//     children,
//     style,
//     itemRender,
//     maxCount,
//   } = props;

//   const [mergedFileList, setMergedFileList] = useMergedState(defaultFileList || [], {
//     value: fileList,
//     postState: list => list ?? [],
//   });

//   const [dragState, setDragState] = React.useState<string>('drop');

//   const upload = React.useRef<any>();

//   warning(
//     'fileList' in props || !('value' in props),
//     'Upload',
//     '`value` is not a valid prop, do you mean `fileList`?',
//   );

//   warning(
//     !('transformFile' in props),
//     'Upload',
//     '`transformFile` is deprecated. Please use `beforeUpload` directly.',
//   );

//   // Control mode will auto fill file uid if not provided
//   React.useMemo(() => {
//     const timestamp = Date.now();

//     (fileList || []).forEach((file, index) => {
//       if (!file.uid && !Object.isFrozen(file)) {
//         file.uid = `__AUTO__${timestamp}_${index}__`;
//       }
//     });
//   }, [fileList]);

//   const onInternalChange = (
//     file: UploadFile,
//     changedFileList: UploadFile[],
//     event?: { percent: number },
//   ) => {
//     let cloneList = [...changedFileList];

//     // Cut to match count
//     if (maxCount === 1) {
//       cloneList = cloneList.slice(-1);
//     } else if (maxCount) {
//       cloneList = cloneList.slice(0, maxCount);
//     }

//     setMergedFileList(cloneList);

//     const changeInfo: UploadChangeParam<UploadFile> = {
//       file: file as UploadFile,
//       fileList: cloneList,
//     };

//     if (event) {
//       changeInfo.event = event;
//     }

//     onChange?.(changeInfo);
//   };

//   const mergedBeforeUpload = async (file: RcFile, fileListArgs: RcFile[]) => {
//     const { beforeUpload, transformFile } = props;

//     let parsedFile: File | Blob | string = file;
//     if (beforeUpload) {
//       const result = await beforeUpload(file, fileListArgs);

//       if (result === false) {
//         return false;
//       }

//       // Hack for LIST_IGNORE, we add additional info to remove from the list
//       delete (file as any)[LIST_IGNORE];
//       if ((result as any) === LIST_IGNORE) {
//         Object.defineProperty(file, LIST_IGNORE, {
//           value: true,
//           configurable: true,
//         });
//         return false;
//       }

//       if (typeof result === 'object' && result) {
//         parsedFile = result as File;
//       }
//     }

//     if (transformFile) {
//       parsedFile = await transformFile(parsedFile as any);
//     }

//     return parsedFile as RcFile;
//   };

//   const onBatchStart: RcUploadProps['onBatchStart'] = batchFileInfoList => {
//     // Skip file which marked as `LIST_IGNORE`, these file will not add to file list
//     const filteredFileInfoList = batchFileInfoList.filter(info => !(info.file as any)[LIST_IGNORE]);

//     // Nothing to do since no file need upload
//     if (!filteredFileInfoList.length) {
//       return;
//     }

//     const objectFileList = filteredFileInfoList.map(info => file2Obj(info.file as RcFile));

//     // Concat new files with prev files
//     let newFileList = [...mergedFileList];

//     objectFileList.forEach(fileObj => {
//       // Replace file if exist
//       newFileList = updateFileList(fileObj, newFileList);
//     });

//     objectFileList.forEach((fileObj, index) => {
//       // Repeat trigger `onChange` event for compatible
//       let triggerFileObj: UploadFile = fileObj;

//       if (!filteredFileInfoList[index].parsedFile) {
//         // `beforeUpload` return false
//         const { originFileObj } = fileObj;
//         let clone;

//         try {
//           clone = new File([originFileObj], originFileObj.name, {
//             type: originFileObj.type,
//           }) as any as UploadFile;
//         } catch (e) {
//           clone = new Blob([originFileObj], {
//             type: originFileObj.type,
//           }) as any as UploadFile;
//           clone.name = originFileObj.name;
//           clone.lastModifiedDate = new Date();
//           clone.lastModified = new Date().getTime();
//         }

//         clone.uid = fileObj.uid;
//         triggerFileObj = clone;
//       } else {
//         // Inject `uploading` status
//         fileObj.status = 'uploading';
//       }

//       onInternalChange(triggerFileObj, newFileList);
//     });
//   };

//   const onSuccess = (response: any, file: RcFile, xhr: any) => {
//     try {
//       if (typeof response === 'string') {
//         response = JSON.parse(response);
//       }
//     } catch (e) {
//       /* do nothing */
//     }

//     // removed
//     if (!getFileItem(file, mergedFileList)) {
//       return;
//     }

//     const targetItem = file2Obj(file);
//     targetItem.status = 'done';
//     targetItem.percent = 100;
//     targetItem.response = response;
//     targetItem.xhr = xhr;

//     const nextFileList = updateFileList(targetItem, mergedFileList);

//     onInternalChange(targetItem, nextFileList);
//   };

//   const onProgress = (e: { percent: number }, file: RcFile) => {
//     // removed
//     if (!getFileItem(file, mergedFileList)) {
//       return;
//     }

//     const targetItem = file2Obj(file);
//     targetItem.status = 'uploading';
//     targetItem.percent = e.percent;

//     const nextFileList = updateFileList(targetItem, mergedFileList);

//     onInternalChange(targetItem, nextFileList, e);
//   };

//   const onError = (error: Error, response: any, file: RcFile) => {
//     // removed
//     if (!getFileItem(file, mergedFileList)) {
//       return;
//     }

//     const targetItem = file2Obj(file);
//     targetItem.error = error;
//     targetItem.response = response;
//     targetItem.status = 'error';

//     const nextFileList = updateFileList(targetItem, mergedFileList);

//     onInternalChange(targetItem, nextFileList);
//   };

//   const handleRemove = (file: UploadFile) => {
//     let currentFile: UploadFile;
//     Promise.resolve(typeof onRemove === 'function' ? onRemove(file) : onRemove).then(ret => {
//       // Prevent removing file
//       if (ret === false) {
//         return;
//       }

//       const removedFileList = removeFileItem(file, mergedFileList);

//       if (removedFileList) {
//         currentFile = { ...file, status: 'removed' };
//         mergedFileList?.forEach(item => {
//           const matchKey = currentFile.uid !== undefined ? 'uid' : 'name';
//           if (item[matchKey] === currentFile[matchKey] && !Object.isFrozen(item)) {
//             item.status = 'removed';
//           }
//         });
//         upload.current?.abort(currentFile);

//         onInternalChange(currentFile, removedFileList);
//       }
//     });
//   };

//   const onFileDrop = (e: React.DragEvent<HTMLDivElement>) => {
//     setDragState(e.type);

//     if (e.type === 'drop') {
//       onDrop?.(e);
//     }
//   };

//   // Test needs
//   React.useImperativeHandle(ref, () => ({
//     onBatchStart,
//     onSuccess,
//     onProgress,
//     onError,
//     fileList: mergedFileList,
//     upload: upload.current,
//   }));

//   const { getPrefixCls, direction } = React.useContext(ConfigContext);

//   const prefixCls = getPrefixCls('upload', customizePrefixCls);

//   const rcUploadProps = {
//     onBatchStart,
//     onError,
//     onProgress,
//     onSuccess,
//     ...(props as RcUploadProps),
//     prefixCls,
//     beforeUpload: mergedBeforeUpload,
//     onChange: undefined,
//   };

//   delete rcUploadProps.className;
//   delete rcUploadProps.style;

//   // Remove id to avoid open by label when trigger is hidden
//   // !children: https://github.com/ant-design/ant-design/issues/14298
//   // disabled: https://github.com/ant-design/ant-design/issues/16478
//   //           https://github.com/ant-design/ant-design/issues/24197
//   if (!children || disabled) {
//     delete rcUploadProps.id;
//   }

//   const renderUploadList = (button?: React.ReactNode, buttonVisible?: boolean) =>
//     showUploadList ? (
//       <LocaleReceiver componentName="Upload" defaultLocale={defaultLocale.Upload}>
//         {(locale: UploadLocale) => {
//           const {
//             showRemoveIcon,
//             showPreviewIcon,
//             showDownloadIcon,
//             removeIcon,
//             previewIcon,
//             downloadIcon,
//           } =
//             typeof showUploadList === 'boolean' ? ({} as ShowUploadListInterface) : showUploadList;
//           return (
//             <UploadList
//               prefixCls={prefixCls}
//               listType={listType}
//               items={mergedFileList}
//               previewFile={previewFile}
//               onPreview={onPreview}
//               onDownload={onDownload}
//               onRemove={handleRemove}
//               showRemoveIcon={!disabled && showRemoveIcon}
//               showPreviewIcon={showPreviewIcon}
//               showDownloadIcon={showDownloadIcon}
//               removeIcon={removeIcon}
//               previewIcon={previewIcon}
//               downloadIcon={downloadIcon}
//               iconRender={iconRender}
//               locale={{ ...locale, ...propLocale }}
//               isImageUrl={isImageUrl}
//               progress={progress}
//               appendAction={button}
//               appendActionVisible={buttonVisible}
//               itemRender={itemRender}
//             />
//           );
//         }}
//       </LocaleReceiver>
//     ) : (
//       button
//     );

//   if (type === 'drag') {
//     const dragCls = classNames(
//       prefixCls,
//       {
//         [`${prefixCls}-drag`]: true,
//         [`${prefixCls}-drag-uploading`]: mergedFileList.some(file => file.status === 'uploading'),
//         [`${prefixCls}-drag-hover`]: dragState === 'dragover',
//         [`${prefixCls}-disabled`]: disabled,
//         [`${prefixCls}-rtl`]: direction === 'rtl',
//       },
//       className,
//     );
//     return (
//       <span>
//         <div
//           className={dragCls}
//           onDrop={onFileDrop}
//           onDragOver={onFileDrop}
//           onDragLeave={onFileDrop}
//           style={style}
//         >
//           <RcUpload {...rcUploadProps} ref={upload} className={`${prefixCls}-btn`}>
//             <div className={`${prefixCls}-drag-container`}>{children}</div>
//           </RcUpload>
//         </div>
//         {renderUploadList()}
//       </span>
//     );
//   }

//   const uploadButtonCls = classNames(prefixCls, {
//     [`${prefixCls}-select`]: true,
//     [`${prefixCls}-select-${listType}`]: true,
//     [`${prefixCls}-disabled`]: disabled,
//     [`${prefixCls}-rtl`]: direction === 'rtl',
//   });

//   const renderUploadButton = (uploadButtonStyle?: React.CSSProperties) => (
//     <div className={uploadButtonCls} style={uploadButtonStyle}>
//       <RcUpload {...rcUploadProps} ref={upload} />
//     </div>
//   );

//   const uploadButton = renderUploadButton(children ? undefined : { display: 'none' });

//   if (listType === 'picture-card') {
//     return (
//       <span className={classNames(`${prefixCls}-picture-card-wrapper`, className)}>
//         {renderUploadList(uploadButton, !!children)}
//       </span>
//     );
//   }

//   return (
//     <span className={className}>
//       {uploadButton}
//       {renderUploadList()}
//     </span>
//   );
// };

// const Upload = React.forwardRef<unknown, UploadProps>(InternalUpload);
// if (process.env.NODE_ENV !== 'production') {
//   Upload.displayName = 'Upload';
// }

// Upload.defaultProps = {
//   type: 'select' as UploadType,
//   multiple: false,
//   action: '',
//   data: {},
//   accept: '',
//   showUploadList: true,
//   listType: 'text' as UploadListType, // or picture
//   className: '',
//   disabled: false,
//   supportServerRender: true,
// };

// export default Upload;

// import type {
//   RcFile as OriRcFile,
//   UploadProps as RcUploadProps,
//   UploadRequestOption as RcCustomRequestOptions,
// } from 'rc-upload/lib/interface';
// import type * as React from 'react';
// import type { ProgressProps } from '../progress';

// export interface RcFile extends OriRcFile {
//   readonly lastModifiedDate: Date;
// }

// export type UploadFileStatus = 'error' | 'success' | 'done' | 'uploading' | 'removed';

// export interface HttpRequestHeader {
//   [key: string]: string;
// }

// export interface UploadFile<T = any> {
//   uid: string;
//   size?: number;
//   name: string;
//   fileName?: string;
//   lastModified?: number;
//   lastModifiedDate?: Date;
//   url?: string;
//   status?: UploadFileStatus;
//   percent?: number;
//   thumbUrl?: string;
//   crossOrigin?: React.ImgHTMLAttributes<HTMLImageElement>['crossOrigin'];
//   originFileObj?: RcFile;
//   response?: T;
//   error?: any;
//   linkProps?: any;
//   type?: string;
//   xhr?: T;
//   preview?: string;
// }

// export interface InternalUploadFile<T = any> extends UploadFile<T> {
//   originFileObj: RcFile;
// }

// export interface UploadChangeParam<T = UploadFile> {
//   // https://github.com/ant-design/ant-design/issues/14420
//   file: T;
//   fileList: T[];
//   event?: { percent: number };
// }

// export interface ShowUploadListInterface {
//   showRemoveIcon?: boolean;
//   showPreviewIcon?: boolean;
//   showDownloadIcon?: boolean;
//   removeIcon?: React.ReactNode | ((file: UploadFile) => React.ReactNode);
//   downloadIcon?: React.ReactNode | ((file: UploadFile) => React.ReactNode);
//   previewIcon?: React.ReactNode | ((file: UploadFile) => React.ReactNode);
// }

// export interface UploadLocale {
//   uploading?: string;
//   removeFile?: string;
//   downloadFile?: string;
//   uploadError?: string;
//   previewFile?: string;
// }

// export type UploadType = 'drag' | 'select';
// export type UploadListType = 'text' | 'picture' | 'picture-card';
// export type UploadListProgressProps = Omit<ProgressProps, 'percent' | 'type'>;

// export type ItemRender<T = any> = (
//   originNode: React.ReactElement,
//   file: UploadFile,
//   fileList: Array<UploadFile<T>>,
//   actions: {
//     download: () => void;
//     preview: () => void;
//     remove: () => void;
//   },
// ) => React.ReactNode;

// type PreviewFileHandler = (file: File | Blob) => PromiseLike<string>;
// type TransformFileHandler = (
//   file: RcFile,
// ) => string | Blob | File | PromiseLike<string | Blob | File>;
// type BeforeUploadValueType = void | boolean | string | Blob | File;

// export interface UploadProps<T = any> extends Pick<RcUploadProps, 'capture'> {
//   type?: UploadType;
//   name?: string;
//   defaultFileList?: Array<UploadFile<T>>;
//   fileList?: Array<UploadFile<T>>;
//   action?: string | ((file: RcFile) => string) | ((file: RcFile) => PromiseLike<string>);
//   directory?: boolean;
//   data?:
//     | Record<string, unknown>
//     | ((file: UploadFile<T>) => Record<string, unknown> | Promise<Record<string, unknown>>);
//   method?: 'POST' | 'PUT' | 'PATCH' | 'post' | 'put' | 'patch';
//   headers?: HttpRequestHeader;
//   showUploadList?: boolean | ShowUploadListInterface;
//   multiple?: boolean;
//   accept?: string;
//   beforeUpload?: (
//     file: RcFile,
//     FileList: RcFile[],
//   ) => BeforeUploadValueType | Promise<BeforeUploadValueType>;
//   onChange?: (info: UploadChangeParam<UploadFile<T>>) => void;
//   onDrop?: (event: React.DragEvent<HTMLDivElement>) => void;
//   listType?: UploadListType;
//   className?: string;
//   onPreview?: (file: UploadFile<T>) => void;
//   onDownload?: (file: UploadFile<T>) => void;
//   onRemove?: (file: UploadFile<T>) => void | boolean | Promise<void | boolean>;
//   supportServerRender?: boolean;
//   style?: React.CSSProperties;
//   disabled?: boolean;
//   prefixCls?: string;
//   customRequest?: (options: RcCustomRequestOptions) => void;
//   withCredentials?: boolean;
//   openFileDialogOnClick?: boolean;
//   locale?: UploadLocale;
//   id?: string;
//   previewFile?: PreviewFileHandler;
//   /** @deprecated Please use `beforeUpload` directly */
//   transformFile?: TransformFileHandler;
//   iconRender?: (file: UploadFile<T>, listType?: UploadListType) => React.ReactNode;
//   isImageUrl?: (file: UploadFile) => boolean;
//   progress?: UploadListProgressProps;
//   itemRender?: ItemRender<T>;
//   /** Config max count of `fileList`. Will replace current one when `maxCount` is 1 */
//   maxCount?: number;
//   children?: React.ReactNode;
// }

// export interface UploadState<T = any> {
//   fileList: UploadFile<T>[];
//   dragState: string;
// }

// export interface UploadListProps<T = any> {
//   listType?: UploadListType;
//   onPreview?: (file: UploadFile<T>) => void;
//   onDownload?: (file: UploadFile<T>) => void;
//   onRemove?: (file: UploadFile<T>) => void | boolean;
//   items?: Array<UploadFile<T>>;
//   progress?: UploadListProgressProps;
//   prefixCls?: string;
//   showRemoveIcon?: boolean;
//   showDownloadIcon?: boolean;
//   showPreviewIcon?: boolean;
//   removeIcon?: React.ReactNode | ((file: UploadFile) => React.ReactNode);
//   downloadIcon?: React.ReactNode | ((file: UploadFile) => React.ReactNode);
//   previewIcon?: React.ReactNode | ((file: UploadFile) => React.ReactNode);
//   locale: UploadLocale;
//   previewFile?: PreviewFileHandler;
//   iconRender?: (file: UploadFile<T>, listType?: UploadListType) => React.ReactNode;
//   isImageUrl?: (file: UploadFile) => boolean;
//   appendAction?: React.ReactNode;
//   appendActionVisible?: boolean;
//   itemRender?: ItemRender<T>;
// }

// import type { InternalUploadFile, RcFile, UploadFile } from './interface';

// export function file2Obj(file: RcFile): InternalUploadFile {
//   return {
//     ...file,
//     lastModified: file.lastModified,
//     lastModifiedDate: file.lastModifiedDate,
//     name: file.name,
//     size: file.size,
//     type: file.type,
//     uid: file.uid,
//     percent: 0,
//     originFileObj: file,
//   };
// }

// /** Upload fileList. Replace file if exist or just push into it. */
// export function updateFileList(file: UploadFile<any>, fileList: UploadFile<any>[]) {
//   const nextFileList = [...fileList];
//   const fileIndex = nextFileList.findIndex(({ uid }: UploadFile) => uid === file.uid);
//   if (fileIndex === -1) {
//     nextFileList.push(file);
//   } else {
//     nextFileList[fileIndex] = file;
//   }
//   return nextFileList;
// }

// export function getFileItem(file: RcFile, fileList: UploadFile[]) {
//   const matchKey = file.uid !== undefined ? 'uid' : 'name';
//   return fileList.filter(item => item[matchKey] === file[matchKey])[0];
// }

// export function removeFileItem(file: UploadFile, fileList: UploadFile[]) {
//   const matchKey = file.uid !== undefined ? 'uid' : 'name';
//   const removed = fileList.filter(item => item[matchKey] !== file[matchKey]);
//   if (removed.length === fileList.length) {
//     return null;
//   }
//   return removed;
// }

// // ==================== Default Image Preview ====================
// const extname = (url: string = '') => {
//   const temp = url.split('/');
//   const filename = temp[temp.length - 1];
//   const filenameWithoutSuffix = filename.split(/#|\?/)[0];
//   return (/\.[^./\\]*$/.exec(filenameWithoutSuffix) || [''])[0];
// };

// const isImageFileType = (type: string): boolean => type.indexOf('image/') === 0;

// export const isImageUrl = (file: UploadFile): boolean => {
//   if (file.type && !file.thumbUrl) {
//     return isImageFileType(file.type);
//   }
//   const url: string = (file.thumbUrl || file.url || '') as string;
//   const extension = extname(url);
//   if (
//     /^data:image\//.test(url) ||
//     /(webp|svg|png|gif|jpg|jpeg|jfif|bmp|dpg|ico)$/i.test(extension)
//   ) {
//     return true;
//   }
//   if (/^data:/.test(url)) {
//     // other file types of base64
//     return false;
//   }
//   if (extension) {
//     // other file types which have extension
//     return false;
//   }
//   return true;
// };

// const MEASURE_SIZE = 200;
// export function previewImage(file: File | Blob): Promise<string> {
//   return new Promise(resolve => {
//     if (!file.type || !isImageFileType(file.type)) {
//       resolve('');
//       return;
//     }

//     const canvas = document.createElement('canvas');
//     canvas.width = MEASURE_SIZE;
//     canvas.height = MEASURE_SIZE;
//     canvas.style.cssText = `position: fixed; left: 0; top: 0; width: ${MEASURE_SIZE}px; height: ${MEASURE_SIZE}px; z-index: 9999; display: none;`;
//     document.body.appendChild(canvas);
//     const ctx = canvas.getContext('2d');
//     const img = new Image();
//     img.onload = () => {
//       const { width, height } = img;

//       let drawWidth = MEASURE_SIZE;
//       let drawHeight = MEASURE_SIZE;
//       let offsetX = 0;
//       let offsetY = 0;

//       if (width > height) {
//         drawHeight = height * (MEASURE_SIZE / width);
//         offsetY = -(drawHeight - drawWidth) / 2;
//       } else {
//         drawWidth = width * (MEASURE_SIZE / height);
//         offsetX = -(drawWidth - drawHeight) / 2;
//       }

//       ctx!.drawImage(img, offsetX, offsetY, drawWidth, drawHeight);
//       const dataURL = canvas.toDataURL();
//       document.body.removeChild(canvas);

//       resolve(dataURL);
//     };
//     img.src = window.URL.createObjectURL(file);
//   });
// }

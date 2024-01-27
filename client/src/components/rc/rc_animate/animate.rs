use web_sys::MouseEvent;
use yew::{function_component, html, use_state, Callback, Properties};

// /* eslint react/prop-types: 0 */
// import React from 'react';
// import unsafeLifecyclesPolyfill from 'rc-util/lib/unsafeLifecyclesPolyfill';
// import {
//   toArrayChildren,
//   mergeChildren,
//   findShownChildInChildrenByKey,
//   findChildInChildrenByKey,
//   isSameChildren,
// } from './ChildrenUtils';
// import AnimateChild from './AnimateChild';
// import animUtil from './util/animate';

pub const DEFAULT_KEY: &str = "rc_animate_87565785686786785685685685678657867"; //${Date.now()}

// const defaultKey = `rc_animate_${Date.now()}`;

// function getChildrenFromProps(props) {
//   const children = props.children;
//   if (React.isValidElement(children)) {
//     if (!children.key) {
//       return React.cloneElement(children, {
//         key: defaultKey,
//       });
//     }
//   }
//   return children;
// }

// function noop() {
// }

#[derive(Properties, PartialEq)]
pub struct AnimateProps {
    pub show_prop: bool,
}

#[derive(Properties, PartialEq)]
pub struct AnimateState {
    pub children: Vec<()>, //toArrayChildren(getChildrenFromProps(props)),
}

#[function_component(Animate)]
pub fn animate(props: &AnimateProps) -> Html {
    let _is_animate = true;
    let _animation = (); // {},
    let _component: &str = "span";
    let _component_props = (); // {},
    let _transition_enter = true;
    let _transition_leave = true;
    let _transition_appear = false;
    let _on_end: Callback<MouseEvent> = Callback::from(move |_| {}); //noop,
    let _on_enter: Callback<MouseEvent> = Callback::from(move |_| {}); //noop,
    let _on_leave: Callback<MouseEvent> = Callback::from(move |_| {}); //noop,
    let _on_appear: Callback<MouseEvent> = Callback::from(move |_| {}); //noop,
                                                                        //   componentDidMount() {
                                                                        // let _currently_animating_keys = (); //{};
    let _keys_to_enter: Vec<String> = vec![];
    let _keys_to_leave: Vec<String> = vec![];
    let state = use_state(|| AnimateState {
        children: Vec::new(),
    }); //children: toArrayChildren(getChildrenFromProps(props)),
    let _children_refs = {};
    let show_prop = props.show_prop;
    let state_cloned = state;
    let _children = state_cloned.children.clone();
    if show_prop {
        //   children = children.filter((child) => {
        //     return !!child.props[showProp];
        //   });
    }
    // children.for_each(|child| {
    //   if (child) {
    //     this.performAppear(child.key);
    //   }
    // });
    // }
    //   componentWillReceiveProps(nextProps) {
    let _next_props = props.clone();
    let _next_children = Vec::<()>::new(); //toArrayChildren(getChildrenFromProps(nextProps));
                                           //     const props = this.props;
                                           //     // exclusive needs immediate response
                                           //     if (props.exclusive) {
                                           //       Object.keys(this.currentlyAnimatingKeys).forEach((key) => {
                                           //         this.stop(key);
                                           //       });
                                           //     }
                                           //     const showProp = props.showProp;
                                           //     const currentlyAnimatingKeys = this.currentlyAnimatingKeys;
                                           //     // last props children if exclusive
                                           //     const currentChildren = props.exclusive ?
                                           //       toArrayChildren(getChildrenFromProps(props)) :
                                           //       this.state.children;
                                           //     // in case destroy in showProp mode
                                           //     let newChildren = [];
                                           // }
    html! {
           //     this.nextProps = nextProps;
    //     const nextChildren = toArrayChildren(getChildrenFromProps(nextProps));
    //     const props = this.props;
    //     // exclusive needs immediate response
    //     if (props.exclusive) {
    //       Object.keys(this.currentlyAnimatingKeys).forEach((key) => {
    //         this.stop(key);
    //       });
    //     }
    //     const showProp = props.showProp;
    //     const currentlyAnimatingKeys = this.currentlyAnimatingKeys;
    //     // last props children if exclusive
    //     const currentChildren = props.exclusive ?
    //       toArrayChildren(getChildrenFromProps(props)) :
    //       this.state.children;
    //     // in case destroy in showProp mode
    //     let newChildren = [];
        }
}

// class Animate extends React.Component {
//   static isAnimate = true; // eslint-disable-line

//   static defaultProps = {
//     animation: {},
//     component: 'span',
//     componentProps: {},
//     transitionEnter: true,
//     transitionLeave: true,
//     transitionAppear: false,
//     onEnd: noop,
//     onEnter: noop,
//     onLeave: noop,
//     onAppear: noop,
//   }

//   constructor(props) {
//     super(props);

//     this.currentlyAnimatingKeys = {};
//     this.keysToEnter = [];
//     this.keysToLeave = [];

//     this.state = {
//       children: toArrayChildren(getChildrenFromProps(props)),
//     };

//     this.childrenRefs = {};
//   }

//   componentDidMount() {
//     const showProp = this.props.showProp;
//     let children = this.state.children;
//     if (showProp) {
//       children = children.filter((child) => {
//         return !!child.props[showProp];
//       });
//     }
//     children.forEach((child) => {
//       if (child) {
//         this.performAppear(child.key);
//       }
//     });
//   }

//   componentWillReceiveProps(nextProps) {
//     this.nextProps = nextProps;
//     const nextChildren = toArrayChildren(getChildrenFromProps(nextProps));
//     const props = this.props;
//     // exclusive needs immediate response
//     if (props.exclusive) {
//       Object.keys(this.currentlyAnimatingKeys).forEach((key) => {
//         this.stop(key);
//       });
//     }
//     const showProp = props.showProp;
//     const currentlyAnimatingKeys = this.currentlyAnimatingKeys;
//     // last props children if exclusive
//     const currentChildren = props.exclusive ?
//       toArrayChildren(getChildrenFromProps(props)) :
//       this.state.children;
//     // in case destroy in showProp mode
//     let newChildren = [];
//     if (showProp) {
//       currentChildren.forEach((currentChild) => {
//         const nextChild = currentChild && findChildInChildrenByKey(nextChildren, currentChild.key);
//         let newChild;
//         if ((!nextChild || !nextChild.props[showProp]) && currentChild.props[showProp]) {
//           newChild = React.cloneElement(nextChild || currentChild, {
//             [showProp]: true,
//           });
//         } else {
//           newChild = nextChild;
//         }
//         if (newChild) {
//           newChildren.push(newChild);
//         }
//       });
//       nextChildren.forEach((nextChild) => {
//         if (!nextChild || !findChildInChildrenByKey(currentChildren, nextChild.key)) {
//           newChildren.push(nextChild);
//         }
//       });
//     } else {
//       newChildren = mergeChildren(
//         currentChildren,
//         nextChildren
//       );
//     }

//     // need render to avoid update
//     this.setState({
//       children: newChildren,
//     });

//     nextChildren.forEach((child) => {
//       const key = child && child.key;
//       if (child && currentlyAnimatingKeys[key]) {
//         return;
//       }
//       const hasPrev = child && findChildInChildrenByKey(currentChildren, key);
//       if (showProp) {
//         const showInNext = child.props[showProp];
//         if (hasPrev) {
//           const showInNow = findShownChildInChildrenByKey(currentChildren, key, showProp);
//           if (!showInNow && showInNext) {
//             this.keysToEnter.push(key);
//           }
//         } else if (showInNext) {
//           this.keysToEnter.push(key);
//         }
//       } else if (!hasPrev) {
//         this.keysToEnter.push(key);
//       }
//     });

//     currentChildren.forEach((child) => {
//       const key = child && child.key;
//       if (child && currentlyAnimatingKeys[key]) {
//         return;
//       }
//       const hasNext = child && findChildInChildrenByKey(nextChildren, key);
//       if (showProp) {
//         const showInNow = child.props[showProp];
//         if (hasNext) {
//           const showInNext = findShownChildInChildrenByKey(nextChildren, key, showProp);
//           if (!showInNext && showInNow) {
//             this.keysToLeave.push(key);
//           }
//         } else if (showInNow) {
//           this.keysToLeave.push(key);
//         }
//       } else if (!hasNext) {
//         this.keysToLeave.push(key);
//       }
//     });
//   }

//   componentDidUpdate() {
//     const keysToEnter = this.keysToEnter;
//     this.keysToEnter = [];
//     keysToEnter.forEach(this.performEnter);
//     const keysToLeave = this.keysToLeave;
//     this.keysToLeave = [];
//     keysToLeave.forEach(this.performLeave);
//   }

//   performEnter = (key) => {
//     // may already remove by exclusive
//     if (this.childrenRefs[key]) {
//       this.currentlyAnimatingKeys[key] = true;
//       this.childrenRefs[key].componentWillEnter(
//         this.handleDoneAdding.bind(this, key, 'enter')
//       );
//     }
//   }

//   performAppear = (key) => {
//     if (this.childrenRefs[key]) {
//       this.currentlyAnimatingKeys[key] = true;
//       this.childrenRefs[key].componentWillAppear(
//         this.handleDoneAdding.bind(this, key, 'appear')
//       );
//     }
//   }

//   handleDoneAdding = (key, type) => {
//     const props = this.props;
//     delete this.currentlyAnimatingKeys[key];
//     // if update on exclusive mode, skip check
//     if (props.exclusive && props !== this.nextProps) {
//       return;
//     }
//     const currentChildren = toArrayChildren(getChildrenFromProps(props));
//     if (!this.isValidChildByKey(currentChildren, key)) {
//       // exclusive will not need this
//       this.performLeave(key);
//     } else if (type === 'appear') {
//       if (animUtil.allowAppearCallback(props)) {
//         props.onAppear(key);
//         props.onEnd(key, true);
//       }
//     } else if (animUtil.allowEnterCallback(props)) {
//       props.onEnter(key);
//       props.onEnd(key, true);
//     }
//   }

//   performLeave = (key) => {
//     // may already remove by exclusive
//     if (this.childrenRefs[key]) {
//       this.currentlyAnimatingKeys[key] = true;
//       this.childrenRefs[key].componentWillLeave(this.handleDoneLeaving.bind(this, key));
//     }
//   }

//   handleDoneLeaving = (key) => {
//     const props = this.props;
//     delete this.currentlyAnimatingKeys[key];
//     // if update on exclusive mode, skip check
//     if (props.exclusive && props !== this.nextProps) {
//       return;
//     }
//     const currentChildren = toArrayChildren(getChildrenFromProps(props));
//     // in case state change is too fast
//     if (this.isValidChildByKey(currentChildren, key)) {
//       this.performEnter(key);
//     } else {
//       const end = () => {
//         if (animUtil.allowLeaveCallback(props)) {
//           props.onLeave(key);
//           props.onEnd(key, false);
//         }
//       };
//       if (!isSameChildren(this.state.children,
//         currentChildren, props.showProp)) {
//         this.setState({
//           children: currentChildren,
//         }, end);
//       } else {
//         end();
//       }
//     }
//   }

//   isValidChildByKey(currentChildren, key) {
//     const showProp = this.props.showProp;
//     if (showProp) {
//       return findShownChildInChildrenByKey(currentChildren, key, showProp);
//     }
//     return findChildInChildrenByKey(currentChildren, key);
//   }

//   stop(key) {
//     delete this.currentlyAnimatingKeys[key];
//     const component = this.childrenRefs[key];
//     if (component) {
//       component.stop();
//     }
//   }

//   render() {
//     const props = this.props;
//     this.nextProps = props;
//     const stateChildren = this.state.children;
//     let children = null;
//     if (stateChildren) {
//       children = stateChildren.map((child) => {
//         if (child === null || child === undefined) {
//           return child;
//         }
//         if (!child.key) {
//           throw new Error('must set key for <rc-animate> children');
//         }
//         return (
//           <AnimateChild
//             key={child.key}
//             ref={node => { this.childrenRefs[child.key] = node }}
//             animation={props.animation}
//             transitionName={props.transitionName}
//             transitionEnter={props.transitionEnter}
//             transitionAppear={props.transitionAppear}
//             transitionLeave={props.transitionLeave}
//           >
//             {child}
//           </AnimateChild>
//         );
//       });
//     }
//     const Component = props.component;
//     if (Component) {
//       let passedProps = props;
//       if (typeof Component === 'string') {
//         passedProps = {
//           className: props.className,
//           style: props.style,
//           ...props.componentProps,
//         };
//       }
//       return <Component {...passedProps}>{children}</Component>;
//     }
//     return children[0] || null;
//   }
// }

// export default unsafeLifecyclesPolyfill(Animate);

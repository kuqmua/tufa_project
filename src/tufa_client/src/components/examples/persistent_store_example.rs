// use crate::components::header::component::Header;
// use crate::routes::routes::Routes;
// use crate::routes::switch_routes::switch_routes;
// use crate::store::init;
// use crate::store::YewduxStore;
// use yew::prelude::*;
// use yew_router::prelude::*;
// use yewdux::prelude::*;
// use crate::components::drawer::component::Drawer;

// pub enum AppMessage {
//     ActionOne,
// }
// pub struct App {
//     pub dispatch: Dispatch<PersistentStore<YewduxStore>>,
// }

// impl Component for App {
//     type Message = AppMessage;
//     type Properties = DispatchProps<PersistentStore<YewduxStore>>;
//     fn create(_ctx: &Context<Self>) -> Self {
//         let dispatch = init();
//         Self { dispatch }
//     }
//     fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
//         true
//     }
//     fn view(&self, _ctx: &Context<Self>) -> Html {
//         html! {
//           <BrowserRouter>
//             <Header/>
//             // <Drawer/>
//             // <div style="width: 200px; height: 200px; display: flex; justify-content: center; align-items: center;">
//             <input type="checkbox" id="menu-opener" />
//           // </div>
//           <aside class="DrawerMenu" id="menu">
//               <nav class="Menu"></nav>
//               <label for="menu-opener" style="background-color: black; opacity: 0.1;"></label>
//           </aside>
//             <div
//               style="
//                 width: 100%;
//                 display: flex;
//                 justify-content: center;
//                 flex-direction: column;
//                 align-items: center;
//               "
//             >
//               <style>
//               // body {
//               //     background: linear-gradient(270deg, #4ad2af, #ff0000);
//               //     background-size: 400% 400%;

//               //     -webkit-animation: AnimationName 30s ease infinite;
//               //     -moz-animation: AnimationName 30s ease infinite;
//               //     animation: AnimationName 30s ease infinite;
//               // }

//               // @-webkit-keyframes AnimationName {
//               //     0%{background-position:53% 0%}
//               //     50%{background-position:48% 100%}
//               //     100%{background-position:53% 0%}
//               // }
//               // @-moz-keyframes AnimationName {
//               //     0%{background-position:53% 0%}
//               //     50%{background-position:48% 100%}
//               //     100%{background-position:53% 0%}
//               // }
//               // @keyframes AnimationName {
//               //     0%{background-position:53% 0%}
//               //     50%{background-position:48% 100%}
//               //     100%{background-position:53% 0%}
//               // }
//               // body {
//               //     background: linear-gradient(-45deg, #ee7752, #e73c7e, #23a6d5, #23d5ab);
//               //     background-size: 400% 400%;
//               //     animation: gradient 15s ease infinite;
//               //     height: 100vh;
//               // }

//               // @keyframes gradient {
//               //     0% {
//               //         background-position: 0% 50%;
//               //     }
//               //     50% {
//               //         background-position: 100% 50%;
//               //     }
//               //     100% {
//               //         background-position: 0% 50%;
//               //     }
//               // }
//               {"
//                 body {
//                   background-color: #16202A;
//                   margin: 0px;
//                   padding: 0px;
//                 }
//                 .DrawerMenu {
//                   position: fixed;
//                   z-index: 99;
//                   width: 100%;
//                   height: 100%;
//                   top: 0;
//                   bottom: 0;
//                   transform: translateX(-100%);
//                   transition: transform 0.5s cubic-bezier(0.4, 0.0, 0.2, 1);
//                   display: grid;
//                   grid-template-columns: 3fr 17fr;
//                 }
//                 #menu-opener:checked~.DrawerMenu {
//                  transform: none
//                 }
//                 .Menu {
//                   transition: all 500ms cubic-bezier(0.4, 0.0, 0.2, 1);
//                   background-color: rgb(21, 255, 0);
//                 }
//               "}
//               </style>
//               <Switch<Routes> render={Switch::render(switch_routes)} />
//             </div>
//           </BrowserRouter>
//         }
//     }
// }

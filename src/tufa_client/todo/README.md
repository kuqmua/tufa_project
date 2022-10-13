* remove contast yellow lines (,make them less then 1px and grey/white)
* make buttons less contrast
* make buttons filled with background color
* make stule struct with methods like in flutter
* make common padding/margin constants
* expander as wrapper component
* material design spiner
* proc macro to implement component around all the buttons inside </br>
https://fonts.google.com/icons?icon.set=Material+Icons </br>
like make http, save icons, open them, rewrite and make component around it inside namespace
* same drawer overlay closing function with expander component
* pack trunk + rust into dockerimage </br>
https://www.youtube.com/watch?v=uYhLWN86V48&t=142s </br>
* antd alert animation not done yet
* use web-sys = { version = "0.3.4", features = ["console"] } </br>
web_sys::console::log() </br>
let msg = format!("roots: {:?}\nroots colors: {:?}", roots_serialized, roots_colors_serialized); </br>
web_console::log_1(&msg.into()); </br>
instead of gloo console log </br>
* maybe should try https://github.com/material-components/material-components-web
* maybe write this first before ant design https://github.com/orgs/react-component/repositories?q=&type=public&language=typescript&sort=
use crate::components::app::App;
// use crate::components::test_drawer::TestDrawer;

pub fn entry() {
    // wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}

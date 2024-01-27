pub fn entry() {
    // wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<crate::components::app::App>(); //crate::components::test_drawer::TestDrawer
}

use gloo_console::log;
use yew::prelude::*;
use yew_element_ui::components::YELPagination;


#[function_component(App)]
fn app() -> Html {

    let cc = Callback::from(|v| {
        log!("v: ", v);
    });

    html! {
    <>
    <div>{"ddddddd"}</div>
    <YELPagination total={1000} on_current_change={cc}/>
    </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

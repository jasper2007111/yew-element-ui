use yew::prelude::*;
use yew_element_ui::components::YELButton;


#[function_component(App)]
fn app() -> Html {
    html! {
    <>
    <div>{"ddddddd"}</div>
    <YELButton>{"ceshi"}</YELButton>
    </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

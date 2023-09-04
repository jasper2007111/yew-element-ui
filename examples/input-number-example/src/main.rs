use gloo_console::log;
use yew::prelude::*;
use yew_element_ui::components::YELInputNumber;

#[function_component(App)]
fn app() -> Html {
    let value = use_state(|| 10.0);
    let value2 = use_state(|| 1.0);


    // let on_change = {
    //     let value_clone = value.clone();
    //     Callback::from(move |v| {
    //         value_clone.set(v);
    //         log!(format!("v: {}", v))
    //     })
    // };


    html! {
      <>
        <YELInputNumber value={*value} precision={Some(2)} step={0.1}></YELInputNumber>
        <YELInputNumber value={*value2} step_strictly={true}></YELInputNumber>
      </>
      }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

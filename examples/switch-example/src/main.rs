use gloo_console::log;
use yew::prelude::*;
use yew_element_ui::components::YELSwitch;

#[function_component(App)]
fn app() -> Html {
    let value = use_state(|| false);

    let on_change = {
        let value_clone: UseStateHandle<bool> = value.clone();
        Callback::from(move |v| {
            value_clone.set(v);
            log!(format!("v: {}", v))
        })
    };

    html! {
      <>
      <YELSwitch value={*value} on_change={on_change}  active_color="#13ce66" inactive_color="#ff4949" active_text="按月付费"
    inactive_text="按年付费"></YELSwitch>
      </>
      }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

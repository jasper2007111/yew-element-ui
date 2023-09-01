use gloo_console::log;
use yew::prelude::*;
use yew_element_ui::components::YELProgress;
use yew_element_ui::components::YELProgressType;



#[function_component(App)]
fn app() -> Html {
    let value = use_state(|| true);
    let arr = use_state(||vec!["选中且禁用".to_string(), "复选框 A".to_string()]);
    let arr_button = use_state(||vec![]);

    let on_change = {
        let value_clone: UseStateHandle<bool> = value.clone();
        Callback::from(move |v| {
            value_clone.set(v);
            log!(format!("v: {}", v))
        })
    };

    let on_group_change = {
        let arr_clone = arr.clone();
        Callback::from(move |v:Vec<String>| {
            log!("on_group_change");
            arr_clone.set(v.clone());
        })
    };

    let on_group_change_clone = {
        let arr_clone = arr_button.clone();
        Callback::from(move |v:Vec<String>| {
            log!("on_group_change_clone");
            arr_clone.set(v.clone());
        })
    };

    html! {
      <>
      <YELProgress percentage={50.0}></YELProgress>
      <YELProgress percentage={25.0} progress_type={Some(YELProgressType::Circle)}></YELProgress>
      </>
      }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

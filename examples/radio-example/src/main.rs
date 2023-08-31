use gloo_console::externs::log;
use yew::prelude::*;
use yew_element_ui::components::YELRadio;
use yew_element_ui::components::YELRadioGroup;
use yew_element_ui::components::YELRadioButton;
use yew_element_ui::common::YELSize;

use gloo_console::log;

#[function_component]
pub fn App() -> Html {
    let inner_value = use_state(|| String::default());

    let on_change = {
        let inner_value = inner_value.clone();
        Callback::from( move |v: String| {
            inner_value.set(v.clone());
            log!(v.clone());
        })
    };

    let value = &*inner_value;

    html! {
        <div>
        <div style="margin-bottom: 50px">
        <YELRadio value={value.clone()} label={"1"} on_change={on_change.clone()} border={true} size={Some(YELSize::Small)}>{"备选项1"}</YELRadio>
        <YELRadio value={value.clone()} label={"2"} on_change={on_change.clone()} border={true} size={Some(YELSize::Small)}>{"备选项2"}</YELRadio>
        </div>

        <div style="margin-bottom: 50px">
            <YELRadioGroup value={"1"} on_change={on_change.clone()} size={Some(YELSize::Small)}>
                <YELRadio label={"1"} border={true} >{"备选项1"}</YELRadio>
                <YELRadio label={"2"} border={true} >{"备选项2"}</YELRadio>
            </YELRadioGroup>
        </div>

        <div style="margin-bottom: 50px">
            <YELRadioGroup value={"广州"} on_change={on_change.clone()} >
                <YELRadioButton label={"上海"} ></YELRadioButton>
                <YELRadioButton label={"北京"} ></YELRadioButton>
                <YELRadioButton label={"广州"} ></YELRadioButton>
                <YELRadioButton label={"深圳"} ></YELRadioButton>
            </YELRadioGroup>
        </div>


        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

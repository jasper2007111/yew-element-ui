use gloo_console::log;
use yew::prelude::*;
use yew_element_ui::components::YELCheckbox;
use yew_element_ui::components::YELCheckboxGroup;
use yew_element_ui::components::YELCheckboxButton;

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
      <YELCheckbox value={*value} {on_change} disabled={true}>{"备选项"}</YELCheckbox>

      <div style="margin: 50px;"/>

      <YELCheckboxGroup value={arr.to_vec()} on_change={on_group_change}>
        <YELCheckbox label={"复选框 A"}></YELCheckbox>
        <YELCheckbox label={"复选框 B"}></YELCheckbox>
        <YELCheckbox label={"复选框 C"}></YELCheckbox>
        <YELCheckbox label={"禁用"} disabled={true}></YELCheckbox>
        <YELCheckbox label={"选中且禁用"} disabled={true}></YELCheckbox>
      </YELCheckboxGroup>

      <div style="margin: 50px;"/>

      <YELCheckboxGroup value={arr_button.to_vec()} on_change={on_group_change_clone}>
        <YELCheckboxButton label={"上海"}></YELCheckboxButton>
        <YELCheckboxButton label={"北京"}></YELCheckboxButton>
        <YELCheckboxButton label={"广州"}></YELCheckboxButton>
        <YELCheckboxButton label={"深圳"} disabled={true}></YELCheckboxButton>
      </YELCheckboxGroup>

      </>
      }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

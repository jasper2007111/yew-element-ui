use yew::prelude::*;
use super::msg_ctx::MessageContext;

#[derive(Clone, PartialEq, Properties)]
pub struct YELOptionProps {
    #[prop_or_default]
    pub label: String,

    #[prop_or_default]
    pub value: String,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub selected:bool
}

#[function_component]
pub fn YELOption(props: &YELOptionProps) -> Html {
    let msg_ctx = use_context::<MessageContext>().unwrap();

    let l = props.label.clone();
    let ll = props.label.clone();
    let onclick = Callback::from(move |_| {
        let id = js_sys::Math::random() * u32::MAX as f64;
        msg_ctx.dispatch((l.clone(), id as u32));
    });

    let mut classes = vec!["el-select-dropdown__item".to_string()];
    if props.selected {
        classes.push("selected".to_string());
    }
    html! {
        <li
            class={classes!(classes)}
            {onclick}
        >
            {ll.clone()}
        </li>
    }
}
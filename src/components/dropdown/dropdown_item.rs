use yew::prelude::*;
use super::super::select::msg_ctx::MessageContext;

#[derive(Clone, PartialEq, Properties)]
pub struct YELDropdownItemProps {
    #[prop_or_default]
    pub label: String,

    #[prop_or_default]
    pub value: String,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub divided:bool,

    #[prop_or_default]
    pub disabled:bool
}

#[function_component]
pub fn YELDropdownItem(props: &YELDropdownItemProps) -> Html {
    let msg_ctx = use_context::<MessageContext>().unwrap();

    let l = props.label.clone();
    let ll = props.label.clone();
    let onclick = Callback::from(move |_| {
        let id = js_sys::Math::random() * u32::MAX as f64;
        msg_ctx.dispatch((l.clone(), id as u32));
    });

    let mut classes = vec!["el-dropdown-menu__item".to_string()];
    if props.disabled {
        classes.push("is-disabled".to_string());
    }
    if props.divided {
        classes.push("el-dropdown-menu__item--divided".to_string());
    }
    html! {
        <li class={classes!(classes)} {onclick} >
            {ll.clone()}
        </li>
    }
}
use yew::prelude::*;
use web_sys::HtmlInputElement;

use super::message::MessageContext;
use crate::common::YELSize;

#[derive(PartialEq, Properties)]
pub struct YELRadioButtonProps {
    #[prop_or_default]
    pub label: String,

    #[prop_or_default]
    pub children: Children,  

    #[prop_or_default]
    pub size: Option<YELSize>
}

#[function_component]
pub fn YELRadioButton(props: &YELRadioButtonProps) -> Html {
    let radio_ref = use_node_ref();
    let msg_ctx = use_context::<MessageContext>();

    let value = if msg_ctx.is_some() {
        let message = msg_ctx.clone().unwrap().inner.to_owned();
        message.value.clone()
    } else {
        "".to_string()
    };
    let is_active = value == props.label;

    let mut label_classes = vec!["el-radio-button".to_string()];
    if is_active {
        label_classes.push("is-active".to_string());
    }

    let radio_ref_clone = radio_ref.clone();
    use_effect_with_deps(
        move |v| {
            let radio = radio_ref_clone.cast::<HtmlInputElement>().unwrap();
            radio.set_checked(*v);
        },
        is_active,
    );

    let onchange = {
        let label = props.label.clone();
        let radio_ref_clone = radio_ref.clone();
        let msg_ctx_clone = msg_ctx.clone();
        
        Callback::from( move |_| {
            let radio = radio_ref_clone.cast::<HtmlInputElement>().unwrap();
            radio.set_checked(true);
            
            if let Some(mc) = msg_ctx_clone.clone() {
                let mut msg = mc.inner.to_owned();
                msg.value = label.clone();
                mc.dispatch(msg);
            } 
        })
    };

    html! {
        <label
            class={classes!(label_classes)}
            role="radio" >
            <input
                ref={radio_ref}
                class="el-radio-button__orig-radio"
                value={props.label.clone()}
                type="radio"
                tabindex="-1"
                autocomplete="off"
                onchange={onchange} />
            <span
                class="el-radio-button__inner" >
                {props.label.clone()}
            </span>
        </label>
    }
}
use yew::prelude::*;
use web_sys::HtmlInputElement;

use crate::common::YELSize;
use super::message::MessageContext;

#[derive(PartialEq, Properties)]
pub struct YELRadioProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub value: String,

    #[prop_or_default]
    pub label: String,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub name: String,

    #[prop_or_default]
    pub border: bool,

    #[prop_or_default]
    pub size: Option<YELSize>,

    #[prop_or_default]
    pub on_change: Callback<String>
}

#[function_component]
pub fn YELRadio(props: &YELRadioProps) -> Html {
    let radio_ref = use_node_ref();

    let msg_ctx = use_context::<MessageContext>();
    let value = if msg_ctx.is_some() {
        let message = msg_ctx.clone().unwrap().inner.to_owned();
        message.value
    } else {
        props.value.clone()
    };

    let mut label_classes = vec!["el-radio".to_string()];
    if value == props.label {
        label_classes.push("is-checked".to_string());
    }

    if props.border {
        label_classes.push("is-bordered".to_string());
        if let Some(m) = msg_ctx.clone() {
            if let Some(s) =  m.inner.size.clone() {
                label_classes.push(format!("el-radio--{}", s));
            }
        } else if let Some(s) = props.size.clone() {
            label_classes.push(format!("el-radio--{}", s));
        }
    }


    let onchange = {
        let label = props.label.clone();
        let on_change = props.on_change.clone();
        let radio_ref_clone = radio_ref.clone();
        let msg_ctx_clone = msg_ctx.clone();
        
        Callback::from( move |_| {
            let radio = radio_ref_clone.cast::<HtmlInputElement>().unwrap();
            radio.set_checked(true);
            
            if let Some(mc) = msg_ctx_clone.clone() {
                let mut msg = mc.inner.to_owned();
                msg.value = label.clone();
                mc.dispatch(msg);
            } else {
                on_change.emit(label.clone());
            }
        })
    };

    let aria_checked = get_aria_checked(props.label==props.value);

    let mut span_classes = vec!["el-radio__input".to_string()];

    let is_checked = value == props.label;
    if is_checked {
        span_classes.push("is-checked".to_string());
    } 

    let radio_ref_clone = radio_ref.clone();
    use_effect_with_deps(
        move |v| {
            let radio = radio_ref_clone.cast::<HtmlInputElement>().unwrap();
            radio.set_checked(*v);
        },
        is_checked,
    );

    html! {
        <label class={classes!(label_classes)} role="radio" aria-checked={aria_checked}>
            <span class={classes!(span_classes)}>
                <span class="el-radio__inner"></span>
                <input
                    ref={radio_ref}
                    class="el-radio__original"
                    value = {props.label.clone()}
                    type="radio"
                    aria-hidden="true"
                    tabindex="-1"
                    autocomplete="off"
                    onchange={onchange}
                />
            </span>
            <span class="el-radio__label">
            {props.children.clone()}
            </span>
        </label>
    }
}

fn get_aria_checked(flag:bool) -> String {
    if flag {
        return "true".to_string();
    } else {
        return "false".to_string();
    }
}


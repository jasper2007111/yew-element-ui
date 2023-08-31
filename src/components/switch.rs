use yew::prelude::*;
use web_sys::{HtmlElement, HtmlInputElement};

use gloo_console::log;

#[derive(PartialEq, Properties)]
pub struct YELSwitchProps {
    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub value: bool,

    #[prop_or(40.0)]
    pub width: f64,

    #[prop_or_default]
    pub on_change: Callback<bool>,

    #[prop_or_default]
    pub active_text: String,

    #[prop_or_default]
    pub inactive_text: String,
        
    #[prop_or("#409EFF".to_string())]
    pub active_color: String,

    #[prop_or("#C0CCDA".to_string())]
    pub inactive_color: String
}

#[function_component]
pub fn YELSwitch(props: &YELSwitchProps) -> Html {
    let input_ref = use_node_ref();
    let core_ref = use_node_ref();

    let onchange = {
        let value = props.value.clone();
        let on_change = props.on_change.clone();
        let input_ref_clone = input_ref.clone();
        // let msg_ctx_clone = msg_ctx.clone();
        
        Callback::from( move |_| {
            log!("=========");
            let input = input_ref_clone.cast::<HtmlInputElement>().unwrap();
            input.set_checked(!value);
            
            // if let Some(mc) = msg_ctx_clone.clone() {
            //     let mut msg = mc.inner.to_owned();
            //     msg.value = label.clone();
            //     mc.dispatch(msg);
            // } else {
            on_change.emit(!value);
            // }
        })
    };

    let input_value = if props.value {
        "true"
    } else {
        "false"
    };

    let mut div_classes = vec!["el-switch".to_string()];
    if props.value  {
        div_classes.push("is-checked".to_string());
    }
    if props.disabled {
        div_classes.push("is-disabled".to_string());
    }

    let onclick = {
        let core_ref_clone = core_ref.clone();
        let active_color = props.active_color.clone();
        let inactive_color = props.inactive_color.clone();
        let on_change = props.on_change.clone();
        let value_clone = props.value.clone();
        let disabled = props.disabled;
        Callback::from( move |_| {
            if disabled {
                return;
            }
            let core = core_ref_clone.cast::<HtmlElement>().unwrap();
            let color = if value_clone {
                &active_color
            } else {
                &inactive_color
            };
            let _ = core.style().set_property("border-color", &color);
            let _ = core.style().set_property("background-color", &color);
            
            // // if let Some(mc) = msg_ctx_clone.clone() {
            // //     let mut msg = mc.inner.to_owned();
            // //     msg.value = label.clone();
            // //     mc.dispatch(msg);
            // // } else {
            on_change.emit(!value_clone);
            // // }
        })
    };

    let span_style = {
        let color = if props.value {
            &props.active_color
        } else {
            &props.inactive_color
        };

        format!("width: {}px; border-color: {}; background-color: {}", props.width, color, color)
    };

    let mut active_span_class = vec!["el-switch__label".to_string(), "el-switch__label--left".to_string()];
    if !props.value {
        active_span_class.push("is-active".to_string());
    }

    let mut inactive_span_class = vec!["el-switch__label".to_string(), "el-switch__label--right".to_string()];
    if props.value {
        inactive_span_class.push("is-active".to_string());
    }

    html! {
        <div class={classes!(div_classes)} role="switch" onclick={onclick}>
            <input ref={input_ref} class="el-switch__input" type="checkbox" onchange={onchange} value={input_value}/>
            if !props.active_text.is_empty() {
                <span class={classes!(active_span_class)}>
                    <span>{&props.active_text}</span>
                </span>
            }
            <span ref={core_ref} class="el-switch__core" style={span_style} />
            if !props.inactive_text.is_empty() {
                <span class={classes!(inactive_span_class)}>
                    <span>{&props.inactive_text}</span>
                </span>
            }
        </div>
    }
}

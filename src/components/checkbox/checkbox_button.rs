use yew::prelude::*;
use web_sys::HtmlInputElement;

use super::message::Message;
use super::message::MessageContext;

#[derive(PartialEq, Properties)]
pub struct YELCheckboxButtonProps {
    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub border: bool,

    #[prop_or_default]
    pub checked: bool,

    #[prop_or_default]
    pub id: AttrValue,

    #[prop_or_default]
    pub label: String,

    #[prop_or_default]
    pub indeterminate: bool,

    #[prop_or_default]
    pub true_label: String,

    #[prop_or_default]
    pub false_label: String,

    #[prop_or_default]
    pub value: bool,

    #[prop_or_default]
    pub on_change: Callback<bool>,

    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn YELCheckboxButton(props: &YELCheckboxButtonProps) -> Html {
    let input_ref = use_node_ref();
    let msg_ctx: Option<UseReducerHandle<Message>> = use_context::<MessageContext>();

    let handle_change: Callback<Event> = {
        let on_change = props.on_change.clone();
        let label = props.label.clone();
        let msg_ctx_clone = msg_ctx.clone();
        Callback::from(move |v: Event| {
            let target: HtmlInputElement = v.target_unchecked_into();
            // log!(target.checked());
            if let Some(m) = msg_ctx_clone.clone() {
                let mut ddd = m.inner.to_owned();
                if target.checked() {
                    ddd.value.push(label.clone());
                } else {
                    let index = ddd.value.iter().position(|x| *x == label.clone());
                    if let Some(i) = index {
                        ddd.value.remove(i);
                    }
                }

                m.dispatch(ddd)
            } else {
                if target.checked() {
                    on_change.emit(true);
                } else {
                    on_change.emit(false);
                }
            }
        })
    };

    html! {
        <label class={get_label_classes(props, &msg_ctx)} role="checkbox">
        {get_input(props, handle_change, &input_ref)}
        {get_slot_span(props)}
        </label>
    }
}
fn get_slot_span(props: &YELCheckboxButtonProps) -> Html {
    if !props.children.is_empty() || !props.label.is_empty() {
        return html!(
            <span class="el-checkbox-button__inner">
                {props.children.clone()}
                if props.children.is_empty() && !props.label.is_empty() {
                    {props.label.clone()}
                }
            </span>
        );
    }
    html!(
        <></>
    )
}

fn get_input(
    props: &YELCheckboxButtonProps,
    handle_change: Callback<Event>,
    input_ref: &NodeRef,
) -> Html {
    if !props.true_label.is_empty() || !props.false_label.is_empty() {
        html! {
            <input
                ref={input_ref}
                class="el-checkbox__original"
                type="checkbox"
                aria-hidden={if props.indeterminate {"true"} else {"false"}}
                true-value={props.true_label.clone()}
                false-value={props.false_label.clone()}
                disabled={props.disabled}
                onchange={handle_change}
                />
        }
    } else {
        html! {
            <input
                ref={input_ref}
                class="el-checkbox__original"
                type="checkbox"
                aria-hidden={if props.indeterminate {"true"} else {"false"}}
                disabled={props.disabled}
                value={if props.value {"true"} else {"false"}}
                onchange={handle_change}
                />
        }
    }
}

fn get_label_classes(
    props: &YELCheckboxButtonProps,
    msg_ctx: &Option<UseReducerHandle<Message>>,
) -> Vec<String> {
    let mut label_classes: Vec<String> = vec!["el-checkbox-button".to_string()];
    if props.disabled {
        label_classes.push("is-disabled".to_string());
    }

    if props.border {
        label_classes.push("is-border".to_string());
    }

    if is_checked(props, msg_ctx) {
        label_classes.push("is-checked".to_string());
    }
    label_classes
}

fn is_checked(props: &YELCheckboxButtonProps, msg_ctx: &Option<UseReducerHandle<Message>>) -> bool {
    if let Some(m) = msg_ctx {
        if m.inner.value.contains(&props.label) {
            return true;
        } else {
            return false;
        }
    } else {
        if props.checked {
            props.checked
        } else {
            props.value
        }
    }
}

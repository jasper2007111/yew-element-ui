use gloo_console::log;
use yew::prelude::*;

use crate::common::YELSize;

use super::message::{Message, MessageContext, MessageInner};

#[derive(PartialEq, Properties)]
pub struct YELCheckboxGroupProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub value: Vec<String>,

    #[prop_or_default]
    pub size: Option<YELSize>,

    #[prop_or_default]
    pub on_change: Callback<Vec<String>>
}

#[function_component]
pub fn YELCheckboxGroup(props: &YELCheckboxGroupProps) -> Html {
    let msg = use_reducer(|| Message {
        inner: MessageInner{
            value: props.value.clone(),
            size: props.size.clone()
        },
    });

    html! {
        <ContextProvider<MessageContext> context={msg}>
          <YELCheckboxGroupInner on_change={props.on_change.clone()} />
          <div class="el-checkbox-group" role="group" aria-label="checkbox-group">
            {props.children.clone()}
          </div>
        </ContextProvider<MessageContext>>
    }
}


#[function_component]
fn YELCheckboxGroupInner(props: &YELCheckboxGroupProps) -> Html {
    let msg_ctx = use_context::<MessageContext>();
    let arr = use_state(||vec![]);
    let is_frist = use_state(||false);

    if let Some(m) = msg_ctx {
        if *is_frist {
            if arr.len() != m.inner.value.len() {
                let change_arr = m.inner.value.to_vec();
                arr.set(change_arr.clone());
                props.on_change.emit(change_arr);
            }
        } else {
            is_frist.set(true);
            arr.set(m.inner.value.to_vec());
        }
    }

    html! {
        <>
        </>
    }
}

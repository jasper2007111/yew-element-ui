use yew::prelude::*;

use super::radio_group_inner::RadioGroupInner;
use super::message::{Message, MessageContext, MessageInner};
use crate::common::YELSize;

#[derive(PartialEq, Properties)]
pub struct YELRadioGroupProps {
    #[prop_or_default]
    pub value: String,

    #[prop_or_default]
    pub children: Children,  

    #[prop_or_default]
    pub on_change: Callback<String>,

    #[prop_or_default]
    pub size: Option<YELSize>
}

#[function_component]
pub fn YELRadioGroup(props: &YELRadioGroupProps) -> Html {
    let msg = use_reducer(|| Message {
        inner: MessageInner{
            value: props.value.clone(),
            size: props.size.clone()
        },
    });

    html! {
        <ContextProvider<MessageContext> context={msg}>
            <RadioGroupInner on_change={props.on_change.clone()}>
                {props.children.clone()}
            </RadioGroupInner>
            <div class="el-radio-group" role="radiogroup">
                {props.children.clone()}
            </div>
        </ContextProvider<MessageContext>>
    }
}
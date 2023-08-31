use yew::prelude::*;
use std::rc::Rc;

use crate::common::YELSize;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Message {
    // pub inner: (String, u32),
    pub inner: MessageInner
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MessageInner {
    pub value: String,
    pub size: Option<YELSize>
}

impl Reducible for Message {
    // type Action = (String, u32);
    type Action = MessageInner;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        Message { inner: action }.into()
    }
}

pub type MessageContext = UseReducerHandle<Message>;
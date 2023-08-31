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
    pub background_color: String,
    pub text_color: String,
    pub active_text_color: String,
    pub active_id: i32
}

impl Reducible for Message {
    // type Action = (String, u32);
    type Action = MessageInner;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        Message { inner: action }.into()
    }
}

pub type MessageContext = UseReducerHandle<Message>;
use yew::prelude::*;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Message {
    pub inner: (String, u32),
}

impl Reducible for Message {
    type Action = (String, u32);

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        Message { inner: action }.into()
    }
}

pub type MessageContext = UseReducerHandle<Message>;

#[derive(Properties, Debug, PartialEq)]
pub struct MessageProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn MessageProvider(props: &MessageProviderProps) -> Html {
    let msg = use_reducer(|| Message {
        inner: (String::default(), 0),
    });

    html! {
        <ContextProvider<MessageContext> context={msg}>
            {props.children.clone()}
        </ContextProvider<MessageContext>>
    }
}
use yew::prelude::*;

use super::message::MessageContext;

#[derive(PartialEq, Properties)]
pub struct RadioGroupInnerProps {
    #[prop_or_default]
    pub value: String,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub on_change: Callback<String>
}

// #[function_component]
// pub fn RadioGroupInner(props: &RadioGroupInnerProps) -> Html {
//     let msg_ctx = use_context::<MessageContext>().unwrap();
    
//     let message = msg_ctx.inner.to_owned();
//     props.on_change.emit(message.0.clone());

//     html! {

//     }
// }

pub enum Msg {
    MessageContextUpdated(MessageContext),
}

pub struct RadioGroupInner {
    message: MessageContext,
    _context_listener: ContextHandle<MessageContext>,
}

impl Component for RadioGroupInner {
    type Message = Msg;
    type Properties = RadioGroupInnerProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (message, context_listener) = ctx
            .link()
            .context(ctx.link().callback(Msg::MessageContextUpdated))
            .expect("No Message Context Provided");

        Self {
            message,
            _context_listener: context_listener,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MessageContextUpdated(message) => {
                // self.message = message.clo;
                let msg = message.inner.to_owned();
                _ctx.props().on_change.emit(msg.value.clone());
                false
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            
        }
    }
}
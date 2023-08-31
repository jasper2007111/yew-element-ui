use super::msg_ctx::MessageProvider;
use super::select_inner::YELSelectInner;

use yew::prelude::*;

pub struct YELSelect {

}

pub enum YELSelectMsg {
}


#[derive(Clone, PartialEq, Properties)]
pub struct YELSelectProps {
    #[prop_or_default]
    pub children: Children,

    pub data:Vec<String>,

    #[prop_or_default]
    pub on_change: Callback<String>
}
impl Component for YELSelect {
    type Message = YELSelectMsg;
    type Properties = YELSelectProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <MessageProvider>
                <YELSelectInner data={ctx.props().data.clone()}>
                    // {ctx.props().children.clone()}
                </YELSelectInner >
            </MessageProvider>
        }
    }
}

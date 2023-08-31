use yew::prelude::*;

use super::dropdown_inner::YELDropdownInner;
use super::super::select::msg_ctx::MessageProvider;

pub struct YELDropdown {
    props: YELDropdownProps
}


#[derive(Clone, PartialEq, Properties)]
pub struct YELDropdownProps {
    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub button_type: String,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub command: Callback<String>
}
pub enum YELDropdownMsg {
}

impl Component for YELDropdown {
    type Message = YELDropdownMsg;
    type Properties = YELDropdownProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone()
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <MessageProvider>
                <YELDropdownInner command={ctx.props().command.clone()}>
                    {ctx.props().children.clone()}
                </YELDropdownInner >
            </MessageProvider>
        }
    }
}
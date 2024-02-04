use yew::prelude::*;

pub struct YELTooltip {
    pub props: YELTooltipProps
}

#[derive(Clone, PartialEq, Properties)]
pub struct YELTooltipProps {
    #[prop_or_default]
    pub children: Children
}

pub enum YELTooltipMsg {
}

impl Component for YELTooltip {
    type Message = YELTooltipMsg;
    type Properties = YELTooltipProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            {self.props.children.clone()}
            </>
        }
    }
}
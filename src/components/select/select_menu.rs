use yew::prelude::*;

pub struct YELSelectMenu {
    props: YELSelectMenuProps
}

pub enum YELSelectMenuMsg {
}

#[derive(Clone, PartialEq, Properties)]
pub struct YELSelectMenuProps {
    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub size: String,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub min_width:Option<f64>
}

impl Component for YELSelectMenu {
    type Message = YELSelectMenuMsg;
    type Properties = YELSelectMenuProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone()
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            if let Some(width) = ctx.props().min_width {
                <div class="el-select-dropdown el-popper" style={format!("min-width: {}px", width)}>
                    {self.props.children.clone()}
                </div>
            } else {
                <div class="el-select-dropdown el-popper">
                    {self.props.children.clone()}
                </div>
            }
        }
    }
}
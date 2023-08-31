use yew::prelude::*;

pub struct YELScrollbar;

pub enum YELScrollbarMsg {
}

#[derive(Clone, PartialEq, Properties)]
pub struct YELScrollbarProps {
    #[prop_or_default]
    pub children: Children
}

impl Component for YELScrollbar {
    type Message = YELScrollbarMsg;
    type Properties = YELScrollbarProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="el-scrollbar">
                <div class="el-select-dropdown__wrap el-scrollbar__wrap el-scrollbar__wrap--hidden-default">
                    <ul class="el-scrollbar__view el-select-dropdown__list">
                        {ctx.props().children.clone()}
                    </ul>
                </div>
            </div>
        }
    }
}
use yew::prelude::*;
pub struct YELDropdownMenu;

pub enum YELDropdownMenuMsg {
}

#[derive(Clone, PartialEq, Properties)]
pub struct YELDropdownMenuProps {
    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub children: Children
}

impl Component for YELDropdownMenu {
    type Message = YELDropdownMenuMsg;
    type Properties = YELDropdownMenuProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <ul class="el-dropdown-menu el-popper">
                {
                    ctx.props().children.clone()
                }
            </ul>
        }
    }
}
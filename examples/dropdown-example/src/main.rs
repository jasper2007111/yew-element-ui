use yew::prelude::*;
use yew_element_ui::components::YELDropdown;
use yew_element_ui::components::YELDropdownMenu;
use yew_element_ui::components::YELDropdownItem;

use gloo_console::log;

pub struct App;

pub enum AppMsg {
    OnCommand(String)
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::OnCommand(v) => {
                log!(v);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <YELDropdown command={ctx.link().callback(|v|{
                    AppMsg::OnCommand(v)
                })}>
                    <span class="el-dropdown-link">
                    {"下拉菜单"}<i class="el-icon-arrow-down el-icon--right"></i>
                    </span>
                    <div slot="dropdown">
                        <YELDropdownMenu>
                            <YELDropdownItem label="菜单1"/>
                            <YELDropdownItem label="菜单2"/>
                            <YELDropdownItem label="菜单3"/>
                        </YELDropdownMenu>
                    </div>
                </YELDropdown>
            </>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

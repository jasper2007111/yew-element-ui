use yew::prelude::*;
use yew_element_ui::components::YELLink;
use yew_element_ui::components::YELLinkType;

use gloo_console::log;

pub struct App;

pub enum AppMsg {
    OnCommand(String),
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::OnCommand(v) => {
                log!(v);
                false
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
                  <>
                      <YELLink >{"标签一"}</YELLink>
                      <YELLink href="https://element.eleme.io" >{"默认链接"}</YELLink>
                      <YELLink link_type={YELLinkType::Primary}>{"主要链接"}</YELLink>
                      <YELLink link_type={YELLinkType::Success}>{"成功链接"}</YELLink>
                      <YELLink link_type={YELLinkType::Warning}>{"警告链接"}</YELLink>
                      <YELLink link_type={YELLinkType::Danger}>{"危险链接"}</YELLink>
                      <YELLink link_type={YELLinkType::Info}>{"信息链接"}</YELLink>
                  </>
              }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

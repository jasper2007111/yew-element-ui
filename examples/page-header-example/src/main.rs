use yew::prelude::*;
use yew_element_ui::components::YELPageHeader;

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
                      <YELPageHeader title="首页" content="内容"></YELPageHeader>
                  </>
              }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

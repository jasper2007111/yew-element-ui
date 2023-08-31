use yew::prelude::*;
use yew_element_ui::components::YELTag;
use yew_element_ui::components::YELTagType;

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
                <YELTag closable={true}>{"标签一"}</YELTag>
                <YELTag tag_type={Some(YELTagType::Success)}>{"标签二"}</YELTag>
                <YELTag tag_type={Some(YELTagType::Info)}>{"标签三"}</YELTag>
                <YELTag tag_type={Some(YELTagType::Warning)}>{"标签四"}</YELTag>
                <YELTag tag_type={Some(YELTagType::Danger)}>{"标签五"}</YELTag>
            </>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

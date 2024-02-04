use yew::prelude::*;
use yew_element_ui::components::YELTag;
use yew_element_ui::components::YELSlider;

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
                <YELSlider></YELSlider>
            </>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

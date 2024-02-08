use yew::prelude::*;
use yew_element_ui::components::YELTag;
use yew_element_ui::components::YELSlider;

use gloo_console::log;

pub struct App {
    pub value:f64
}

pub enum AppMsg {
    OnCommand(String),
    OnChange(f64)
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            value: 50.0
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::OnCommand(v) => {
                log!(v);
                false
            },
            AppMsg::OnChange(v) => {
                self.value = v;
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <YELSlider value={self.value} on_change={ctx.link().callback(|v| {
                    AppMsg::OnChange(v)
                })}></YELSlider>
            </>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

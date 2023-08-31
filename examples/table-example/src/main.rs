use yew::prelude::*;
use yew_element_ui::components::YELTable;
use yew_element_ui::components::YELTableColumnProps;
use yew_element_ui::components::YELTableRow;

use bevy_reflect::Reflect;

use gloo_console::log;

#[derive(Reflect, Clone)]
struct Data {
    name: String,
    date: String,
    address: String,
}

pub struct App {
    data: Vec<Data>,
}

pub enum AppMsg {
    OnCommand(String),
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            data: vec![
                Data {
                    name: "Jasper".to_string(),
                    date: "2023-08-16".to_string(),
                    address: "地址".to_string(),
                },
                Data {
                    name: "Tom".to_string(),
                    date: "2023-08-16".to_string(),
                    address: "地址".to_string(),
                },
            ],
        }
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
        let columns = vec![
            YELTableColumnProps {
                label: "名字".to_string(),
                width: 180.0,
            },
            YELTableColumnProps {
                label: "日期".to_string(),
                width: 300.0,
            },
            YELTableColumnProps {
                label: "地址".to_string(),
                width: 180.0,
            },
        ];

        html! {
            <>
                <YELTable columns={columns}>
                    {
                        self.data.clone().into_iter().map(|d|{
                            let v = vec!["11".to_string(), "22".to_string(), "33".to_string()];
                            html!{
                                <YELTableRow data={v}/>
                            }
                        }).collect::<Html>()
                    }
                </YELTable>
            </>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

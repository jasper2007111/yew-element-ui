use yew::prelude::*;

use super::menu_type::YELMenuMode;
use super::message::{Message, MessageContext, MessageInner};


#[derive(PartialEq, Properties)]
pub struct YELMenuProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or("#ffffff".to_string())]
    pub background_color: String,

    #[prop_or("#303133".to_string())]
    pub text_color: String,

    #[prop_or("#409EFF".to_string())]
    pub active_text_color: String,

    #[prop_or_default]
    pub mode: YELMenuMode
}

#[function_component]
pub fn YELMenu(props: &YELMenuProps) -> Html {
    let msg = use_reducer(|| Message {
        inner: MessageInner{
            background_color: props.background_color.clone(),
            text_color: props.text_color.clone(),
            active_id: i32::default(),
            active_text_color: props.active_text_color.clone()
        },
    });

    let component = html!(
        <ul role="menubar" class={classes!(get_ul_classes(props.mode.clone()))} style={format!("background-color: {}", props.background_color.clone())}>
            {props.children.clone()}
        </ul>
    );

    html! {
        <ContextProvider<MessageContext> context={msg}>
            {component}
        </ContextProvider<MessageContext>>
    }
}

fn get_ul_classes(mode: YELMenuMode) -> Vec<String> {
    let mut vec = vec!["el-menu".to_string()];
    if mode == YELMenuMode::Horizontal {
        vec.push("el-menu--horizontal".to_string());
    }
    vec
}
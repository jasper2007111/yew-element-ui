use gloo_console::log;
use yew::prelude::*;

use super::message::MessageContext;

#[derive(PartialEq, Properties)]
pub struct YELMenuItemProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub index: String,

    #[prop_or(1)]
    pub level: u8,

    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
}

#[function_component]
pub fn YELMenuItem(props: &YELMenuItemProps) -> Html {
    let msg_ctx = use_context::<MessageContext>();
    let item_id = use_state(|| {
        let id = js_sys::Math::random() * u32::MAX as f64;
        id as i32
    });

    let mut padding = 0;
    for _ in 0..props.level {
        padding += 20;
    }

    let style = {
        let mut s = format!("padding-left: {padding}px;");
        let active = *item_id == msg_ctx.clone().unwrap().inner.active_id;
        if let Some(m) = msg_ctx.clone() {
            s.push_str(&format!("background-color: {};", m.inner.background_color));

            if active {
                s.push_str(&format!("color: {};", m.inner.active_text_color));
            } else {
                s.push_str(&format!("color: {};", m.inner.text_color));
            }
        }
        s
    };

    let onclick = {
        let on_click = props.on_click.clone();
        let msg_ctx_clone = msg_ctx.clone();
        let item_id_clone = *item_id.clone();
        Callback::from(move|e| {
            if let Some(mc) = msg_ctx_clone.clone() {
                let mut msg = mc.inner.to_owned();
                msg.active_id = item_id_clone;
                mc.dispatch(msg);
            } 
            on_click.emit(e);
        })
    };

    html! {
        <li onclick={onclick} class="el-menu-item" style={style}>
            {props.children.clone()}
        </li>
    }
}

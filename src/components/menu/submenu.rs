use gloo_console::log;
use yew::prelude::*;
use web_sys::HtmlElement;

use crate::utils::slot_util;

#[derive(PartialEq, Properties)]
pub struct YELSubmenuProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub level: u8
}

#[function_component]
pub fn YELSubmenu(props: &YELSubmenuProps) -> Html {
    let title = slot_util::get_comp_list_slot_by_name(props.children.clone(), "title".to_string());
    let content = slot_util::get_comp_list_slot_by_name(props.children.clone(), "content".to_string());

    let opened = use_state(||false);

    let root_li_classes = vec!["el-submenu".to_string()];

    let root_ref = use_node_ref();
    let root_ref_clone = root_ref.clone();
    use_effect(move || {
        // Make a call to DOM API after component is rendered
        // if let Some(n) = root_ref_clone.cast::<HtmlElement>() {
        //     log!(format!("root_ref: {:?}", n.class_name()));
        //     let mut padding = 20;
        //     let mut parent = n.parent_element();

        //     if let Some(p) = parent {
        //         let mut pp = p;
        //         log!("pp.class_name(): ", pp.class_name().clone());
        //         while pp.class_name().contains("el-submenu") {
        //             padding += 20;

        //             parent = pp.parent_element();
        //             if let Some(p) = parent {
        //                 pp = p;
        //             } else {
        //                 break;
        //             }
        //         }
        //     }

        //     log!("padding: ", padding);
        //     // if n.class_name().contains("el-submenu") {

        //     // }
        // }
    });

    let mut padding = 0;
    for _ in 0..props.level {
        padding += 20;
    }

    let onclick = {
        let opened_clone = opened.clone();
        Callback::from(move |_e|{
            opened_clone.set(!*opened_clone);
        })
    };

    html! {
        <li ref={&root_ref} class={classes!(root_li_classes)} role="menuitem">
            <div class="el-submenu__title" style={format!("padding-left: {padding}px;")} {onclick}>
                if let Some(t) = title {
                    {t}
                }
                <i class={"el-submenu__icon-arrow el-icon-arrow-down"}></i>
            </div>
            if *opened {
                <ul
                role="menu"
                class={"el-menu el-menu--inline"}>
                if let Some(c) = content {
                {c}
                }
                </ul>
            }
        </li>
    }
}
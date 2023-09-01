use yew::prelude::*;

use crate::utils::slot_util;

#[derive(PartialEq, Properties)]
pub struct YELPageHeaderProps {
    #[prop_or_default]
    pub title: String,

    #[prop_or_default]
    pub content: String,

    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,

    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn YELPageHeader(props: &YELPageHeaderProps) -> Html {
    let title_slot = slot_util::get_slot_by_name(props.children.clone(), "title".to_string());
    let content_slot = slot_util::get_slot_by_name(props.children.clone(), "content".to_string());

    html! {
        <div class="el-page-header">
            <div class="el-page-header__left" onclick={props.on_click.clone()}>
            <i class="el-icon-back"></i>
            <div class="el-page-header__title">
                if let Some(t) = title_slot {
                    {t}
                } else {
                    { props.title.clone() }
                }
            </div>
            </div>
            <div class="el-page-header__content">
                if let Some(c) = content_slot {
                    {c}
                } else {
                    { props.content.clone() }
                }
            </div>
        </div>
    }
}

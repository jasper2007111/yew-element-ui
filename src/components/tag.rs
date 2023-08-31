use yew::prelude::*;

use crate::common::YELSize;

#[derive(Clone, PartialEq, Debug, Eq)]
pub enum YELTagType {
    Success,
    Warning,
    Danger,
    Info,
}

impl std::fmt::Display for YELTagType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let result = match *self {
            YELTagType::Success => "success",
            YELTagType::Warning => "warning",
            YELTagType::Danger => "danger",
            YELTagType::Info => "info",
        };
        write!(f, "{}", result)
    }
}

#[derive(Clone, PartialEq, Debug, Eq)]
pub enum YELTagEffect {
    Dark,
    Light,
    Plain,
}

impl std::fmt::Display for YELTagEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let result = match *self {
            YELTagEffect::Dark => "dark",
            YELTagEffect::Light => "light",
            YELTagEffect::Plain => "plain",
        };
        write!(f, "{}", result)
    }
}

#[derive(PartialEq, Properties)]
pub struct YELTagProps {
    #[prop_or_default]
    pub tag_type: Option<YELTagType>,

    #[prop_or_default]
    pub effect: Option<YELTagEffect>,

    #[prop_or_default]
    pub size: Option<YELSize>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub hit: bool,

    #[prop_or_default]
    pub color: String,

    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,

    #[prop_or_default]
    pub closable: bool,
}

#[function_component]
pub fn YELTag(props: &YELTagProps) -> Html {
    let on_click = {
        let on_click = props.on_click.clone();
        Callback::from(move |e| {
            on_click.emit(e);
        })
    };

    let on_click_clone = on_click.clone();

    html! {
        <span
            class={get_span_classes(props)}
            style={if props.color.is_empty() {"".to_string()} else {format!("background-color: {}", props.color)}}
            onclick={on_click}
            >
                {props.children.clone()}
                if props.closable {
                    <i class="el-tag__close el-icon-close" onclick={ on_click_clone }></i>
                }
            </span>
    }
}

fn get_span_classes(props: &YELTagProps) -> Vec<String> {
    let mut span_classes = vec!["el-tag".to_string()];
    if let Some(t) = props.tag_type.clone() {
        span_classes.push(format!("el-tag--{}", t));
    }

    if let Some(e) = props.effect.clone() {
        span_classes.push(format!("el-tag--{}", e));
    }

    if let Some(s) = props.size.clone() {
        span_classes.push(format!("el-tag--{}", s));
    }

    if props.hit {
        span_classes.push("is-hit".to_string());
    }

    span_classes
}

use yew::prelude::*;

#[derive(Clone, PartialEq, Debug, Eq)]
pub enum YELLinkType {
    Primary,
    Success,
    Warning,
    Danger,
    Info,
}

impl std::fmt::Display for YELLinkType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let result = match *self {
            YELLinkType::Primary => "primary",
            YELLinkType::Success => "success",
            YELLinkType::Warning => "warning",
            YELLinkType::Danger => "danger",
            YELLinkType::Info => "info",
        };
        write!(f, "{}", result)
    }
}

#[derive(PartialEq, Properties)]
pub struct YELLinkProps {
    #[prop_or_default]
    pub link_type: Option<YELLinkType>,

    #[prop_or_default]
    pub underline: bool,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub href: String,

    #[prop_or_default]
    pub icon: String,

    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,

    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn YELLink(props: &YELLinkProps) -> Html {
    let mut classes = vec!["el-link".to_string()];

    if let Some(t) = &props.link_type {
        classes.push(format!("el-link--{}", t));
    }

    if props.disabled {
        classes.push("is-disabled".to_string());
        if props.underline {
            classes.push("is-underline".to_string());
        }
    }

    let handle_click = {
        let disabled = props.disabled;
        let href = props.href.clone();
        let on_click = props.on_click.clone();
        Callback::from(move|e|{
            if !disabled {
                if !href.is_empty() {
                    on_click.emit(e);
                }
            }
        })
    };

    html! {
        <a
            class={classes}
            href={
                if props.disabled {
                    "".to_string()
                } else {
                    props.href.clone()
                }}
            onclick={handle_click} >
            if !props.icon.is_empty() {
                <i class={props.icon.clone()} />
            }
            if !props.children.is_empty() {
                <span class="el-link--inner">
                    {props.children.clone()}
                </span>
            }
            </a>
    }
}

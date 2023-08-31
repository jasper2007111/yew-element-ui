use yew::prelude::*;
use crate::common::YELSize;

#[derive(Clone, PartialEq, Debug, Eq)]
pub enum YELButtonType {
    Primary,
    Success,
    Warning,
    Danger,
    Info,
    Text
}

impl std::fmt::Display for YELButtonType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let result = match *self {
            YELButtonType::Primary => "primary",
            YELButtonType::Success => "success",
            YELButtonType::Warning => "warning",
            YELButtonType::Danger => "danger",
            YELButtonType::Info => "info",
            YELButtonType::Text => "text",
        };
        write!(f, "{}", result)
    }
}

pub enum YELButtonMsg {}
pub struct YELButton {
    props: YELButtonProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct YELButtonProps {
    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub button_type: Option<YELButtonType>,

    #[prop_or_default]
    pub on_clicked: Callback<MouseEvent>,

    #[prop_or_default]
    pub loading: bool,

    #[prop_or_default]
    pub plain: bool,

    #[prop_or_default]
    pub size: Option<YELSize>,

    #[prop_or_default]
    pub width: String,

    #[prop_or_default]
    pub height: String,

    #[prop_or_default]
    pub children: Children
}

impl Component for YELButton {
    type Message = YELButtonMsg;
    type Properties = YELButtonProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {}
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let disabled = ctx.props().disabled.clone();
        let button_type = ctx.props().button_type.clone();
        let loading = ctx.props().loading.clone();

        let onclick = ctx.props().on_clicked.reform(move |event: MouseEvent| {
            event.stop_propagation();
            event.clone()
        });

        let mut classes = Vec::new();
        classes.push(String::from("el-button"));

        if let Some(t) = button_type {
            classes.push(format!("el-button--{}", t));
        }

        if disabled {
            classes.push(String::from("is-disabled"));
        }

        if self.props.plain {
            classes.push(String::from("is-plain"));
        }

        if let Some(s) = self.props.size.clone() {
            classes.push(format!("el-button--{}", s));
        }

        let mut style = String::default();
        if !self.props.width.is_empty() {
            style.push_str(&format!("width: {}", self.props.width));
        }

        if !self.props.height.is_empty() {
            style.push_str(&format!("height: {}", self.props.height));
        }

        html! {
            <button class={classes!(classes.clone())} {onclick} disabled={disabled.clone()}  style={style}>
            {self.props.children.clone()}
            if loading {
                <i class="el-icon-loading"></i>
            }
            </button>
        }
    }
}

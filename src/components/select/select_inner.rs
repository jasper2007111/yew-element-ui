use yew::prelude::*;
use web_sys::HtmlElement;

use super::msg_ctx::MessageContext;
use super::select_menu::YELSelectMenu;
use super::option::YELOption;
use crate::components::input::YELInput;
use crate::components::scrollbar::scrollbar::YELScrollbar;

pub struct YELSelectInner {
    message: MessageContext,
    _context_listener: ContextHandle<MessageContext>,
    input_root_ref: NodeRef,
    visible: bool,
    props: YELSelectInnerProps,
    input_width: Option<f64>
}

pub enum YELSelectInnerMsg {
    MessageContextUpdated(MessageContext),
    OnFocus,
}

#[derive(Clone, PartialEq, Properties)]
pub struct YELSelectInnerProps {
    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub size: String,

    #[prop_or_default]
    pub children: Children,

    pub data:Vec<String>,

    #[prop_or_default]
    pub on_change: Callback<String>
}

impl Component for YELSelectInner {
    type Message = YELSelectInnerMsg;
    type Properties = YELSelectInnerProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (message, context_listener) = ctx
            .link()
            .context(
                ctx.link()
                    .callback(YELSelectInnerMsg::MessageContextUpdated),
            )
            .expect("No Message Context Provided");

        Self {
            input_width: None,
            visible: false,
            input_root_ref: NodeRef::default(),
            props: ctx.props().clone(),
            message,
            _context_listener: context_listener,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            YELSelectInnerMsg::MessageContextUpdated(message) => {
                self.message = message;
                self.visible = false;
                self.props.on_change.emit(self.message.inner.0.clone());
                true
            }
            YELSelectInnerMsg::OnFocus => {
                self.visible = true;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut menus = vec![];
        for value in ctx.props().data.clone().into_iter() {
            let selected = if self.message.inner.0 == value {
                true
            } else {
                false
            };
            
            menus.push(html!(<YELOption label={value.clone()} value = {value.clone()} {selected}/>));
        }

        let mut suffix_classes = vec!["el-select__caret".to_string(), "el-input__icon".to_string(), "el-icon-arrow-up".to_string()];
        if self.visible {
            suffix_classes.push("is-reverse".to_string());
        } 

        html! {
            <div
                class={classes!(self.get_root_classes())}
            >
            <YELInput root_ref={&self.input_root_ref} value={String::from(self.message.inner.0.clone())} on_focus={ctx.link().callback(|_|{
                YELSelectInnerMsg::OnFocus
            })}>
              <div slot="suffix">
              <i class={classes!(suffix_classes)}></i>
              </div>
            </YELInput>
            if self.visible {
                <YELSelectMenu min_width={self.input_width}>
                    <YELScrollbar>
                        {menus}
                    </YELScrollbar>
                </YELSelectMenu>
            }
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(div) = self.input_root_ref.cast::<HtmlElement>() {
                let rect = div.get_bounding_client_rect();
                self.input_width = Some(rect.width());
            }
        }
    }
}

impl YELSelectInner {
    fn get_root_classes(&self) -> Vec<String> {
        let mut vec = vec!["el-select".to_string()];
        if let Some(c) = self.get_select_size() {
            vec.push(c);
        }
        vec
    }
    fn get_select_size(&self) -> Option<String> {
        if !self.props.size.is_empty() {
            return Some(self.props.size.clone());
        }
        return None;
    }
}

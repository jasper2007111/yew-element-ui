use gloo::utils::document;
use web_sys::{Element, HtmlElement, Node};
use yew::prelude::*;

use super::super::select::msg_ctx::MessageContext;
use crate::utils::slot_util;

use popper_rs::modifier::{Modifier, Offset};
use popper_rs::prelude::{Options, Placement};
use popper_rs::state::ApplyAttributes;
use popper_rs::yew::use_popper;
use yew::platform::spawn_local;

use yew::virtual_dom::{VNode, VPortal};

#[derive(PartialEq, Properties)]
pub struct TooltipProperties {
    pub id: AttrValue,
    pub reference_node: NodeRef,

    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub placement: Placement,
}

#[function_component(Tooltip)]
pub fn example(props: &TooltipProperties) -> Html {
    let tooltip_ref = use_node_ref();
    let reference_ref = props.reference_node.clone();

    let options = use_memo(
        |placement| Options {
            placement: *placement,
            modifiers: vec![Modifier::Offset(Offset {
                skidding: 0,
                distance: 8,
            })],
            ..Default::default()
        },
        props.placement,
    );

    let popper = use_popper(reference_ref.clone(), tooltip_ref.clone(), options).unwrap();

    {
        let popper = popper.instance.clone();
        use_effect(|| {
            spawn_local(async move {
                popper.update().await;
            });
        });
    }

    use_effect_with_deps(
        |(tooltip_ref, attributes)| {
            tooltip_ref.apply_attributes(attributes);
        },
        (tooltip_ref.clone(), popper.state.attributes.popper.clone()),
    );

    html!(
        <div
            data-show="true"
            ref={tooltip_ref}
            id={props.id.clone()}
            role="tooltip"
            class="tooltip"
            style={&popper.state.styles.popper}
        >
            { for props.children.iter() }
            <div class="arrow" data-popper-arrow="true" style={&popper.state.styles.arrow}></div>
        </div>
    )
}

pub struct YELDropdownInner {
    props: YELDropdownInnerProps,
    message: MessageContext,
    _context_listener: ContextHandle<MessageContext>,
    visible: bool,
    trigger_ref: NodeRef,

    trigger_origin: (f64, f64),
    trigger_size: (f64, f64),

    poper_id: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct YELDropdownInnerProps {
    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub button_type: String,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub command: Callback<String>,
}
pub enum YELDropdownInnerMsg {
    MessageContextUpdated(MessageContext),
    OnClick,
}

impl Component for YELDropdownInner {
    type Message = YELDropdownInnerMsg;
    type Properties = YELDropdownInnerProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (message, context_listener) = ctx
            .link()
            .context(
                ctx.link()
                    .callback(YELDropdownInnerMsg::MessageContextUpdated),
            )
            .expect("No Message Context Provided");

        let id = js_sys::Math::random() * u32::MAX as f64;

        Self {
            props: ctx.props().clone(),
            message,
            _context_listener: context_listener,
            visible: false,
            trigger_ref: NodeRef::default(),
            trigger_origin: (0.0, 0.0),
            trigger_size: (0.0, 0.0),
            poper_id: format!("dropdown-menu-{}", id as u32),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            YELDropdownInnerMsg::MessageContextUpdated(message) => {
                self.message = message;
                self.visible = false;
                self.props.command.emit(self.message.inner.0.clone());
                true
            }
            YELDropdownInnerMsg::OnClick => {
                self.visible = !self.visible;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let trigger_elm = slot_util::get_slots_default(ctx.props().children.clone());
        let menu_elm =
            slot_util::get_comp_slot_by_name(ctx.props().children.clone(), "dropdown".to_string());

        let style_width = self.trigger_size.0+40.0;
        let style_top = self.trigger_origin.1 + self.trigger_size.1;
        let style_left = self.trigger_origin.0;
        let style = format!("position: absolute; z-index: 2045; top: {}px; left: {}px; width: {}px;", style_top, style_left, style_width);
        html! {
            <div class="el-dropdown">
            <div ref={&self.trigger_ref} style="margin-bottom: 15px;" onclick={ctx.link().callback(|_| {
                YELDropdownInnerMsg::OnClick
            })}>
                if let Some(t) = trigger_elm {{t}}
            </div>
            if let Some(m) = menu_elm {
                if self.visible {
                    <MyComponent child_node={m} div_id={self.poper_id.clone()} style={style}/>
                }
            }
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(div) = self.trigger_ref.cast::<HtmlElement>() {
                let rect = div.get_bounding_client_rect();
                self.trigger_size = (rect.width(), rect.height());
                self.trigger_origin = (rect.x(), rect.y());
            }
        }
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        let dd = document().get_element_by_id(&self.poper_id);
        if let Some(d) = dd {
            let parent = d.parent_element();
            if let Some(p) = parent {
                let n: Node = d.into();
                let _ = p.remove_child(&n);
            }
            // d.parent_element().unwrap().remove_child(&d);
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct MyComponentProps {
    #[prop_or_default]
    pub child_node: VNode,

    pub div_id: String,

    pub style: String
}

#[function_component]
fn MyComponent(props: &MyComponentProps) -> Html {
    let node = use_memo(
        |_| {
            let dd = document().get_element_by_id(&props.div_id);
            if let Some(d) = dd {
                let p = VPortal::new(props.child_node.clone(), d);
                return Html::VPortal(p);
            }
            let div: Element = document().create_element("div").unwrap();
            div.set_id(&props.div_id);
            let _ = div.set_attribute("style", &props.style);
            let body = gloo::utils::document().body().unwrap();
            let _ = body.append_child(&div);

            let p = VPortal::new(props.child_node.clone(), div);
            Html::VPortal(p)
        },
        (),
    );
    (*node).clone()
}

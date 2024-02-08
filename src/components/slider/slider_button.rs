use js_sys::Math::log;
use wasm_bindgen::convert::OptionIntoWasmAbi;
use web_sys::Element;
use yew::prelude::*;

use crate::components::tooltip::tooltip::YELTooltip;
use crate::components::YELButton;
use std::cell::RefCell;

use gloo_console::log;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};

pub struct YELSliderButton {
    pub on_dragging: RefCell<Option<&'static JsValue>>,
    pub on_drag_end: Option<Closure<dyn FnMut(MouseEvent)>>,
    pub start_y: f64,
    pub start_x: f64,
    pub current_x: f64,
    pub current_y: f64,
    pub start_position: f64,
    pub props: YELSliderButtonProps,
    pub dragging: bool,
    pub div_ref: NodeRef
}

#[derive(PartialEq, Properties, Clone)]
pub struct YELSliderButtonProps {
    #[prop_or_default]
    pub vertical: bool,

    #[prop_or_default]
    pub value: f64,

    #[prop_or_default]
    pub on_change: Callback<f64>,
}

pub enum YELSliderButtonMsg {
    OnMouseEnter(MouseEvent),
    OnMouseLeave(MouseEvent),
    OnMouseDown(MouseEvent),
    OnDragging(MouseEvent),
    OnDragEnd(MouseEvent),
}

impl Component for YELSliderButton {
    type Message = YELSliderButtonMsg;
    type Properties = YELSliderButtonProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            start_y: 0.0,
            start_x: 0.0,
            on_dragging: RefCell::new(None),
            on_drag_end: None,
            current_x: 0.0,
            current_y: 0.0,
            start_position: 0.0,
            dragging: false,
            props: ctx.props().clone(),
            div_ref: NodeRef::default()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            YELSliderButtonMsg::OnMouseEnter(e) => {
                log!("YELSliderButtonMsg::OnMouseEnter");
                false
            }
            YELSliderButtonMsg::OnMouseLeave(e) => {
                log!("YELSliderButtonMsg::OnMouseLeave");
                false
            }
            YELSliderButtonMsg::OnMouseDown(e) => {
                log!("YELSliderButtonMsg::OnMouseDown");
                e.prevent_default();
                self.dragging = true;
                let window = web_sys::window().unwrap();

                let parent_link = ctx.link().get_parent().expect("No Parent found").clone();
                let op = parent_link.try_downcast::<super::slider::YELSlider>().expect("msg");
                // if let Some(s) = op {
                //     log!(s);
                // }
                // log!();

                if self.props.vertical {
                    self.start_y = e.client_y() as f64;
                } else {
                    self.start_x = e.client_x() as f64;
                }

                self.start_position = self.current_position();

                let ctx_cp = ctx.link().clone();
                let on_dragging: Closure<dyn FnMut(MouseEvent)> =
                    Closure::wrap(Box::new(move |e: MouseEvent| {
                        // log!("mousemove");
                        ctx_cp.send_message(YELSliderButtonMsg::OnDragging(e));
                    }) as Box<dyn FnMut(_)>);

                let ctx_cp_1 = ctx.link().clone();
                let on_drag_end: Closure<dyn FnMut(MouseEvent)> =
                    Closure::wrap(Box::new(move |e: MouseEvent| {
                        log!("mouseup");
                        ctx_cp_1.send_message(YELSliderButtonMsg::OnDragEnd(e));
                    }) as Box<dyn FnMut(_)>);

                let mut cb_ref: &'static JsValue;
                unsafe {
                    cb_ref =
                        std::mem::transmute::<&JsValue, &'static JsValue>(on_dragging.as_ref());
                }

                window
                    .add_event_listener_with_callback(
                        "mouseup",
                        &on_drag_end.as_ref().unchecked_ref(),
                    )
                    .expect("msg");
                on_drag_end.forget();

                window
                    .add_event_listener_with_callback("mousemove", cb_ref.unchecked_ref())
                    .expect("msg");

                self.on_dragging.replace(Some(cb_ref));
                on_dragging.forget();
                false
            }
            YELSliderButtonMsg::OnDragging(e) => {
                if !self.dragging {
                    return false;
                }
                // log!("YELSliderButtonMsg::OnDragging");
                let mut diff = 0.0;

                if self.props.vertical {
                    self.current_y = e.client_y() as f64;
                    diff = (self.start_y - self.current_y) / 100.0;
                } else {
                    self.current_x = e.client_x() as f64;
                    let div = self.div_ref.cast::<web_sys::Element>().unwrap();
                    let parent_div = div.parent_element();
                    let mut parent_size = 0.0;
                    if let Some(p) = parent_div  {
                        // log!("parent: ", p.get_bounding_client_rect());
                        parent_size = p.get_bounding_client_rect().width();
                    }
                    diff = (self.current_x - self.start_x) / parent_size*100.0;
                }

                let mut new_position = self.start_position + diff;
                // log!("new_position: ", new_position);
                if new_position<0.0 {
                    new_position = 0.0;
                } else if new_position>100.0 {
                    new_position = 100.0;
                }
                let max = 100;
                let min = 0;
                let step = 1;
                let length_per_step = 100 / (max-min) / step;

                let steps = js_sys::Math::round(new_position/length_per_step as f64);
                let value = steps*(length_per_step as f64) * ((max - min) as f64)*0.01 + min as f64;
                // log!("value: ", value);
                self.props.value = value;
                self.props.on_change.emit(value);
                true
            }
            YELSliderButtonMsg::OnDragEnd(e) => {
                log!("YELSliderButtonMsg::OnDragEnd");
                self.dragging = false;
                let window = web_sys::window().unwrap();
                let dd = self.on_dragging.take();
                if let Some(on_dragging) = dd {
                    log!("YELSliderButtonMsg::OnDragEnd========");
                    window
                        .remove_event_listener_with_callback(
                            "mousemove",
                            on_dragging.unchecked_ref(),
                        )
                        .expect("msg");
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div ref={&self.div_ref} class="el-slider__button-wrapper" style={self.wrapper_style()} onmouseenter={ctx.link().callback(|e|{
                YELSliderButtonMsg::OnMouseEnter(e)
            })} onmouseleave={ctx.link().callback(|e|{
                YELSliderButtonMsg::OnMouseLeave(e)
            })} onmousedown={ctx.link().callback(|e|{
                YELSliderButtonMsg::OnMouseDown(e)
            })}>
                <YELTooltip>
                    <div class="el-slider__button" ></div>
                </YELTooltip>
            </div>
        }
    }
}

impl YELSliderButton {
    fn wrapper_style(&self) -> String {
        let current_position = format!("{}%", self.current_position());
        if self.props.vertical {
            return format!("bottom: {}", current_position);
        }
        format!("left: {}", current_position)
    }

    fn current_position(&self) -> f64 {
        let per = self.props.value/100.0*100.0;
        per
    }
}

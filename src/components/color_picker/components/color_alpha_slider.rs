use web_sys::HtmlElement;
use yew::prelude::*;

pub enum Msg {
    OnFristRended,
    OnClicked(MouseEvent),
}

pub struct YELColorAlphaSlider {
    thumb_left: f64,
    thumb_top: f64,
    root_ref: NodeRef,
    bar_ref: NodeRef,
    thumb_ref: NodeRef,
    props: YELColorAlphaSliderProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct YELColorAlphaSliderProps {
    #[prop_or_default]
    pub vertical: bool,

    #[prop_or_default]
    pub value: (u8, u8, u8, u8),

    #[prop_or_default]
    pub on_change: Callback<f64>,
}

impl Component for YELColorAlphaSlider {
    type Message = Msg;
    type Properties = YELColorAlphaSliderProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            thumb_left: 0.0,
            thumb_top: 0.0,
            root_ref: NodeRef::default(),
            bar_ref: NodeRef::default(),
            thumb_ref: NodeRef::default(),
            props: ctx.props().clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnFristRended => {
                let root = self.root_ref.cast::<HtmlElement>().unwrap();
                let thumb = self.thumb_ref.cast::<HtmlElement>().unwrap();
                let alpha = self.props.value.3 as f64 / 255.0 * 100.0;
                if self.props.vertical {
                    self.thumb_left = 0.0;
                    self.thumb_top = js_sys::Math::round(
                        alpha * (root.offset_height() as f64 - thumb.offset_height() as f64 / 2.0),
                    ) / 100.0;
                } else {
                    self.thumb_top = 0.0;
                    self.thumb_left = js_sys::Math::round(
                        alpha * (root.offset_width() as f64 - thumb.offset_width() as f64 / 2.0),
                    ) / 100.0;
                }
                true
            }
            Msg::OnClicked(e) => {
                let root = self.root_ref.cast::<HtmlElement>().unwrap();
                let rect = root.get_bounding_client_rect();
                let thumb = self.thumb_ref.cast::<HtmlElement>().unwrap();
                if self.props.vertical {
                    let mut top = e.client_y() as f64 - rect.top() as f64;
                    top = js_sys::Math::max(thumb.offset_height() as f64 / 2.0, top);
                    top = js_sys::Math::min(
                        top,
                        rect.height() as f64 - thumb.offset_height() as f64 / 2.0,
                    );
                    self.thumb_top = top;
                    self.thumb_left = 0.0;
                    self.props.on_change.emit(js_sys::Math::round(
                        (top - thumb.offset_height() as f64 / 2.0)
                            / (rect.height() as f64 - thumb.offset_height() as f64)
                            * 100.0,
                    ));
                } else {
                    let mut left = e.client_x() as f64 - rect.left() as f64;
                    left = js_sys::Math::max(thumb.offset_width() as f64 / 2.0, left);
                    left = js_sys::Math::min(
                        left,
                        rect.width() as f64 - thumb.offset_width() as f64 / 2.0,
                    );
                    self.thumb_left = left;
                    self.thumb_top = 0.0;
                    self.props.on_change.emit(js_sys::Math::round(
                        (left - thumb.offset_width() as f64 / 2.0)
                            / (rect.width() as f64 - thumb.offset_width() as f64)
                            * 100.0,
                    ));
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = vec!["el-color-alpha-slider".to_string()];
        if self.props.vertical {
            classes.push("is-vertical".to_string());
        }
        let background = Self::get_background(
            ctx.props().value.0,
            ctx.props().value.1,
            ctx.props().value.2,
        );
        html! {
            <div ref={&self.root_ref} class={classes}>
                <div ref={&self.bar_ref} onclick={ctx.link().callback(|e|{
                    Msg::OnClicked(e)
                })} class="el-color-alpha-slider__bar" style={format!("background: {}", background)}></div>
                <div ref={&self.thumb_ref} class="el-color-alpha-slider__thumb" style={format!("left: {}px; top: {}px", self.thumb_left, self.thumb_top)}></div>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_message(Msg::OnFristRended);
        }
    }
}

impl YELColorAlphaSlider {
    pub fn get_background(r: u8, g: u8, b: u8) -> String {
        format!(
            "linear-gradient(to right, rgba({}, {}, {}, 0) 0%, rgba({}, {}, {}, 1) 100%)",
            r, g, b, r, g, b
        )
    }
}

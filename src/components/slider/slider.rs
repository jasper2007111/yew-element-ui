use yew::prelude::*;

use super::slider_button::YELSliderButton;
use crate::components::input_number::YELInputNumber;

#[derive(PartialEq, Properties, Clone)]
pub struct YELSliderProps {
    #[prop_or_default]
    pub min: i32,

    #[prop_or(100)]
    pub max: i32,

    #[prop_or(0.0)]
    pub value: f64,

    #[prop_or_default]
    pub vertical: bool,

    #[prop_or_default]
    pub show_input: bool,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub range: bool,
}
pub struct YELSlider {
    props: YELSliderProps,
}

pub enum YELSliderMsg {}

impl Component for YELSlider {
    type Message = YELSliderMsg;
    type Properties = YELSliderProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div
                class={self.get_div_classes()}
                role="slider"
                aria-valuemin={self.props.min.to_string()}
                aria-valuemax={self.props.max.to_string()}
                aria-orientation={if self.props.vertical{"vertical"} else {"horizontal"}}
                aria-disabled={self.slider_disabled().to_string()}
                >
                if self.props.show_input && !self.props.range {
                    <YELInputNumber
                        class={AttrValue::from("el-slider__input")}
                    />
                }
                <div
                    class={self.get_runway_class()}
                    >
                    <YELSliderButton/>
                </div>
            </div>
        }
    }

    
}

impl YELSlider {
    fn get_runway_style(&self) -> String {
        "".to_string()
    }
    fn get_runway_class(&self) -> Vec<String> {
        let mut classes = vec!["el-slider__runway".to_string()];
        if self.props.show_input {
            classes.push("show-input".to_string());
        }

        if self.slider_disabled() {
            classes.push("disabled".to_string());
        }

        classes
    }

    pub fn get_size() -> i32 {
        0
    }

    fn get_div_classes(&self) -> Vec<String> {
        let mut classes = vec!["el-slider".to_string()];
        if self.props.vertical {
            classes.push("is-vertical".to_string());
        }

        if self.props.show_input {
            classes.push("el-slider--with-input".to_string());
        }

        classes
    }

    fn slider_disabled(&self) -> bool {
        if self.props.disabled {
            return true;
        }
        false
    }
}

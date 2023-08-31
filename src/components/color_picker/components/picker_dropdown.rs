use yew::prelude::*;

use crate::components::button::YELButton;
use crate::utils::ColorUtil;

use super::color_hue_slider::YELColorHueSlider;
use super::sv_panel::YELSvPanel;
use super::color_alpha_slider::YELColorAlphaSlider;
use crate::components::YELButtonType;
use crate::common::YELSize;

pub enum Msg {
    None,
    OnHandleTrigger(MouseEvent),
    OnHueChanged(f64),
    OnSvChanded((f64, f64)),
    OnAlphaChanged(f64),
    OnConfirmValue,
    OnClearValue,
}
pub struct YELPickerDropdown {
    hue: f64,
    saturation: f64,
    value: f64,
    alpha: f64,
    show_panel_color: bool,
    props: YELPickerDropdownProps,
    color_hex: String,
    rgba:(u8, u8, u8, u8)
}

#[derive(Clone, PartialEq, Properties)]
pub struct YELPickerDropdownProps {
    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub value: String,

    #[prop_or_default]
    pub show_alpha: bool,

    #[prop_or_default]
    pub on_change: Callback<String>,

    #[prop_or_default]
    pub on_confirm_value: Callback<String>,

    #[prop_or_default]
    pub on_clear_value: Callback<String>,

    #[prop_or_default]
    pub custom_input: String
}

impl Component for YELPickerDropdown {
    type Message = Msg;
    type Properties = YELPickerDropdownProps;

    fn create(ctx: &Context<Self>) -> Self {
        if !ctx.props().value.is_empty() {
            let rgb_result = csscolorparser::parse(ctx.props().value.as_str());
            if rgb_result.is_ok() {
                let color = rgb_result.unwrap();
                let hsv = color.to_hsva();
                let h = hsv.0;
                let s: f64 = hsv.1;
                let v = hsv.2;

                let temp_saturation = js_sys::Math::floor(s * 100.0);
                let temp_value = js_sys::Math::floor(v * 100.0);

                let rgba = color.to_rgba8();
                return Self {
                    alpha: rgba[3] as f64 /255.0*100.0,
                    rgba: (rgba[0], rgba[1], rgba[2], rgba[3]),
                    color_hex: color.to_hex_string(),
                    hue: h,
                    saturation: temp_saturation,
                    value: temp_value,
                    show_panel_color: false,
                    props: ctx.props().clone(),
                };
            }
        }
        Self {
            alpha: 100.0,
            rgba:(0,0,0,255),
            color_hex: String::default(),
            hue: 0.0,
            saturation: 0.0,
            value: 0.0,
            show_panel_color: false,
            props: ctx.props().clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::None => false,
            Msg::OnHandleTrigger(_e) => false,
            Msg::OnHueChanged(v) => {
                self.hue = v;
                self.update_color();
                true
            }
            Msg::OnSvChanded((s, v)) => {
                self.saturation = s;
                self.value = v;
                self.update_color();
                true
            }
            Msg::OnClearValue => {
                self.props.on_clear_value.emit("clear".to_string());
                false
            }
            Msg::OnConfirmValue => {
                self.props.on_confirm_value.emit("confirm".to_string());
                false
            },
            Msg::OnAlphaChanged(v) =>{
                self.alpha = v;
                self.update_color();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let custom_input;
        if !self.color_hex.is_empty() {
            let rgb = self.rgba;
            if self.props.show_alpha {
                custom_input = format!("rgba({}, {}, {}, {:.2})", rgb.0, rgb.1, rgb.2, rgb.3 as f64/255.0);
            } else {
                custom_input = self.color_hex.clone();
            }
        } else {
            custom_input = self.props.custom_input.clone();
        }
        let mut classes = vec!["el-color-picker".to_string()];
        if self.get_color_disabled() {
            classes.push("is-disabled".to_string());
        }
        html! {
            <div class="el-color-dropdown el-color-picker__panel">
                <div class="el-color-dropdown__main-wrapper">
                    <div style="float: right;"><YELColorHueSlider hue={self.hue} on_change={ctx.link().callback(|v|{
                        Msg::OnHueChanged(v)
                    })}/></div>
                    <YELSvPanel hue={self.hue} saturation={self.saturation} value={self.value} on_change={ctx.link().callback(|e|{
                        Msg::OnSvChanded(e)
                    })}/>
                </div>
                if self.props.show_alpha {
                    <YELColorAlphaSlider value={self.rgba} on_change={ctx.link().callback(|v|{
                        Msg::OnAlphaChanged(v)
                    })}/>
                }
                // <predefine v-if="predefine" :color="color" :colors="predefine"></predefine>
                <div class="el-color-dropdown__btns">
                    <span class="el-color-dropdown__value">
                        // TODO 暂未实现input组件，使用文本代替
                        {custom_input}
                    </span>
                    <YELButton button_type={Some(YELButtonType::Primary)} size={Some(YELSize::Mini)} on_clicked={ctx.link().callback(|_|{
                        Msg::OnClearValue
                    })}>{"清空"}</YELButton>
                    <YELButton size={Some(YELSize::Mini)} plain={true} on_clicked={ctx.link().callback(|_|{
                        Msg::OnConfirmValue
                    })}>{"确定"}</YELButton>
                </div>
            </div>
        }
    }
}

impl YELPickerDropdown {
    pub fn get_color_disabled(&self) -> bool {
        self.props.disabled
    }

    // pub fn get_show_alpha_class(&self) -> String {
    //     if self.props.show_alpha {
    //         return "is-alpha".to_string();
    //     }
    //     "".to_string()
    // }

    // pub fn get_displayed_color(&self) -> String {
    //     "#fff".to_string()
    // }

    pub fn update_color(&mut self) {
        let rgb = ColorUtil::hsv2rgb(self.hue, self.saturation, self.value);
        // log!(format!("r: {}, g: {}, b:{}", rgb.0, rgb.1, rgb.2));

        if self.props.show_alpha {
            self.rgba = (rgb.0, rgb.1, rgb.2, 255);
            let a = self.alpha/100.0;
            self.props.custom_input = format!("rgba({}, {}, {}, {:.2})", rgb.0, rgb.1, rgb.2, a);
        } else {
            self.rgba = (rgb.0, rgb.1, rgb.2, 255);
            self.props.custom_input = ColorUtil::rgb2hex(rgb.0, rgb.1, rgb.2);
        }

        self.props.on_change.emit(self.props.custom_input.clone());
        self.color_hex = String::default();
        // log!(self.props.custom_input.clone());
    }
}

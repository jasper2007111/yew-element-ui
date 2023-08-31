
use yew::prelude::*;

use super::components::picker_dropdown::YELPickerDropdown;

pub enum Msg {
    OnHandleTrigger(MouseEvent),
    OnPickerDropdownValueChanged(String),
    OnPickerDropdownConfirmValue,
    OnPickerDropdownClearValue
}
pub struct YELColorPicker {
    show_picker:bool,
    show_panel_color: bool,
    props:YELColorPickerProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct YELColorPickerProps {
    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub value: String,

    #[prop_or_default]
    pub size: String,

    #[prop_or_default]
    pub show_alpha: bool,

    #[prop_or_default]
    pub on_change: Callback<String>
}

impl Component for YELColorPicker {
    type Message = Msg;
    type Properties = YELColorPickerProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            show_picker:false,
            show_panel_color: false,
            props: ctx.props().clone()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnHandleTrigger(_e)=>{
                if self.get_color_disabled() {
                    return false;
                }
                self.show_picker = !self.show_picker;
                true
            },
            Msg::OnPickerDropdownValueChanged(v)=>{
                self.props.value = v.clone();
                self.show_panel_color = true;
                self.props.on_change.emit(v.clone());
                true
            }
            Msg::OnPickerDropdownConfirmValue=>{
                self.show_picker =false;
                true
            },
            Msg::OnPickerDropdownClearValue=>{
                self.show_picker =false;
                self.props.value = String::default();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let mut classes = vec!["el-color-picker".to_string()];
        if self.get_color_disabled() {
            classes.push("is-disabled".to_string());
        }
        if !self.props.size.is_empty() {
            classes.push(format!("el-color-picker--{}", self.props.size));
        }
        html! {
            <div class={classes!(classes)}>
                if self.get_color_disabled() {
                    <div class="el-color-picker__mask"></div>
                }
                <div class="el-color-picker__trigger" onclick={ctx.link().callback(move |e: MouseEvent| { 
                    Msg::OnHandleTrigger(e)
                })}>
                    <span class={format!("el-color-picker__color {}", self.get_show_alpha_class())}>
                        <span class="el-color-picker__color-inner" style={format!("background-color: {};", self.get_displayed_color())}>
                        </span>
                        if self.props.value.is_empty() || !self.show_panel_color {
                            <span class="el-color-picker__empty el-icon-close"></span>
                        }
                    </span>
                    if !self.props.value.is_empty() || self.show_panel_color {
                        <span class="el-color-picker__icon el-icon-arrow-down"></span>
                    }
                </div>
                if self.show_picker {
                    <YELPickerDropdown value={self.props.value.clone()} on_change={ctx.link().callback(|v|{
                        Msg::OnPickerDropdownValueChanged(v)
                    })} on_confirm_value={ctx.link().callback(|_|{
                        Msg::OnPickerDropdownConfirmValue
                    })} on_clear_value={ctx.link().callback(|_|{
                        Msg::OnPickerDropdownClearValue
                    })} show_alpha={self.props.show_alpha}/>
                }
            </div>
        }
    }
}

impl YELColorPicker {
    pub fn get_color_disabled(&self)->bool {
        self.props.disabled
    }

    pub fn get_show_alpha_class(&self)->String {
        if self.props.show_alpha {
            return "is-alpha".to_string();
        }
        "".to_string()
    }

    pub fn get_displayed_color(&self) ->String {
        if self.props.value.is_empty()  {
            return "#fff".to_string()
        }
        self.props.value.clone()
    }
}


use std::collections::HashMap;

use web_sys::Element;
use yew::prelude::*;

pub enum YELRateMsg {
    OnMouseMove((i32, MouseEvent)),
    OnSelectValue(i32),
    OnMouseLeave,
    OnKeydown(KeyboardEvent)
}
pub struct YELRate {
    pointer_at_left_half:bool,
    current_value:f64,
    hover_index:i32,
    props:YELRateProps
}

struct ClassMapVaule  {
    pub value:String,
    pub excluded: bool
}

#[derive(Clone, PartialEq, Properties)]
pub struct YELRateProps {
    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub on_change: Callback<f64>,

    #[prop_or(0.0)]
    pub value:f64,

    #[prop_or(5)]
    pub max:i32,

    #[prop_or(vec!["el-icon-star-on".to_string(); 3])]
    pub icon_classes:Vec<String>,

    #[prop_or(vec!["#F7BA2A".to_string(), "#F7BA2A".to_string(), "#F7BA2A".to_string()])]
    pub colors:Vec<String>,

    #[prop_or("#C6D1DE".to_string())]
    pub void_color:String,

    #[prop_or("#EFF2F7".to_string())]
    pub disabled_void_color:String,

    #[prop_or("el-icon-star-on".to_string())]
    pub disabled_void_icon_class:String,

    #[prop_or("el-icon-star-off".to_string())]
    pub void_icon_class:String,

    #[prop_or("#1f2d3d".to_string())]
    pub text_color:String,

    #[prop_or(2)]
    pub low_threshold:i32,

    #[prop_or(4)]
    pub high_threshold:i32,

    #[prop_or(false)]
    pub allow_half:bool,

    #[prop_or(false)]
    pub show_score:bool,

    #[prop_or(false)]
    pub show_text:bool,

    #[prop_or(vec!["极差".to_string(), "失望".to_string(), "一般".to_string(), "满意".to_string(), "惊喜".to_string()])]
    pub texts:Vec<String>
}

impl Component for YELRate {
    type Message = YELRateMsg;
    type Properties = YELRateProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            pointer_at_left_half:true,
            current_value: ctx.props().value,
            hover_index:-1,
            props: ctx.props().clone()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            YELRateMsg::OnMouseMove((index, e))=> {
                if self.is_rate_disabled() {
                    return false;
                }
                if self.props.allow_half {
                    let element: Element = e.target_unchecked_into();
                    let mut target = None;
                    // 这段代码原本是Element UI的实现，但是Yew的鼠标移动事件并不能穿透，所以这段代码弃用
                    // if element.class_list().contains("el-rate__item") {
                    //     target = element.query_selector(".el-rate__icon").unwrap();
                    // }
                    // if target.is_some()&& target.clone().unwrap().class_list().contains("el-rate__decimal") {
                    //     target = target.clone().unwrap().parent_element();
                    // } else if element.class_list().contains("el-rate__decimal") {
                    //     target = element.parent_element();
                    // }
                    if target.is_none() {
                        target = Some(element);
                    }
                    if target.is_some() {
                        let offset_x = e.offset_x()*2;
                        let client_width = target.clone().unwrap().client_width();
                        self.pointer_at_left_half = offset_x <= client_width;
                        self.current_value = if self.pointer_at_left_half {
                            (index+1) as f64 - 0.5
                        } else {
                            (index+1) as f64
                        };
                    } 
                } else {

                    
                    self.current_value = (index+1) as f64;
                }
                self.hover_index = index+1;
                true
            },
            YELRateMsg::OnMouseLeave=>{
                if self.is_rate_disabled() {
                    return false;
                }
                if self.props.allow_half {
                    self.pointer_at_left_half = self.props.value != js_sys::Math::floor(self.props.value);
                }
                self.current_value = self.props.value;
                true
            },
            YELRateMsg::OnSelectValue(i)=>{
                if self.is_rate_disabled() {
                    return false;
                }
                if self.props.allow_half && self.pointer_at_left_half {
                    self.props.value = self.current_value;
                } else {
                    self.current_value = (i+1) as f64;
                    self.props.value = self.current_value;
                }
                self.props.on_change.emit(self.props.value);
                self.hover_index = -1;
                true
            },
            YELRateMsg::OnKeydown(event) => {
                if self.is_rate_disabled() {
                    return false;
                }
                let mut current_value = self.current_value;
                let keycode = event.key_code();
                if keycode == 38 ||keycode == 39 {
                    if self.props.allow_half {
                        current_value += 0.5;
                    } else {
                        current_value += 1.0;
                    }
                    event.stop_propagation();
                    event.prevent_default();
                } else if keycode == 37 || keycode == 40 {
                    if self.props.allow_half {
                        current_value -= 0.5;
                    } else {
                        current_value -= 1.0;
                    }
                    event.stop_propagation();
                    event.prevent_default();
                }
                current_value = if current_value < 0.0 {
                    0.0
                } else if current_value > self.props.max as f64{
                    self.props.max as f64
                } else {
                    current_value
                };

                if current_value != self.current_value {
                    self.props.value = current_value;
                    self.current_value = self.props.value;
                    self.props.on_change.emit(self.props.value);
                    true
                } else {
                    false
                }
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let show_text = ctx.props().show_text.clone();
        let max = ctx.props().max.clone();
        let show_score = ctx.props().show_score.clone();
        let text_color = ctx.props().text_color.clone();

        let span_style = if self.is_rate_disabled() {
            "cursor: auto"
        } else {
            "cursor: pointer"
        };

        let mut span_vec = vec![];
        for i in 0..max  {
            span_vec.push(html!{
                <span class={"el-rate__item"} style={span_style}>
                <i class={self.get_classes(i)} style={self.get_icon_style(i)} onmousemove={ctx.link().callback(move |e: MouseEvent| { 
                    YELRateMsg::OnMouseMove((i, e))
                })} onmouseleave={ctx.link().callback(move |_e: MouseEvent| { 
                    YELRateMsg::OnMouseLeave
                })} onclick={ctx.link().callback(move |_e: MouseEvent| { 
                    YELRateMsg::OnSelectValue(i)
                })}>
                if self.is_show_decimal_icon(i+1) {
                    <i class={self.get_decimal_icon_class()} style={self.get_decimal_style()}/>
                }
                </i>
                </span>
            });
        }
        html! {
            <div onkeydown={ctx.link().callback(move|e:KeyboardEvent|{
                YELRateMsg::OnKeydown(e)
            })} class="el-rate" role="slider" tabindex="0" aria-valuemin="0" aria-valuemax={max.to_string()} aria-valuenow={self.current_value.to_string()} >
            {span_vec}
            if show_score || show_text {
                <span class="el-rate__text" style={format!("color: {}", text_color)}>{self.get_text()}</span>
            }
            </div>
        }
    }
}

impl YELRate {
    pub fn get_active_color(&self)->String {
        let s = self.get_value_from_map(js_sys::Math::ceil(self.current_value) as i32, self.get_color_map());
        s
    }
    fn get_color_map(&self)->HashMap<i32, ClassMapVaule> {
        let mut map = HashMap::new();

        map.insert(self.props.low_threshold, ClassMapVaule{
            value: self.props.colors[0].clone(),
            excluded: false
        });

        map.insert(self.props.high_threshold, ClassMapVaule{
            value: self.props.colors[1].clone(),
            excluded: true
        });

        map.insert(self.props.max, ClassMapVaule{
            value: self.props.colors[2].clone(),
            excluded: false
        });

        map
    }

    fn get_class_map(&self)->HashMap<i32, ClassMapVaule> {
        let mut map = HashMap::new();

        map.insert(self.props.low_threshold, ClassMapVaule{
            value: self.props.icon_classes[0].clone(),
            excluded: false
        });

        map.insert(self.props.high_threshold, ClassMapVaule{
            value: self.props.icon_classes[1].clone(),
            excluded: true
        });

        map.insert(self.props.max, ClassMapVaule{
            value: self.props.icon_classes[2].clone(),
            excluded: false
        });

        map
    }
    fn get_value_from_map(&self, value: i32, map:HashMap<i32, ClassMapVaule>) -> String{
        for (key, item) in &map {
            if item.excluded {
                if value>*key {
                    return item.value.clone();
                } 
            } else {
                if value<=*key {
                    return item.value.clone();
                }
            }
        }

        "".to_string()
    }
    pub fn get_icon_style(&self, item:i32) -> String {
        let void_color = if self.is_rate_disabled() {
            self.props.disabled_void_color.clone()
        } else {
            self.props.void_color.clone()
        };
        if item<self.current_value as i32 {
            format!("color: {}; hover: {}", self.get_active_color(), self.hover_index==item)
        } else {
            format!("color: {}; hover: {}", void_color, self.hover_index==item)
        }
    }
    pub fn get_active_class(&self) -> String  {
        let s = self.get_value_from_map(js_sys::Math::ceil(self.current_value) as i32, self.get_class_map());
        s
    }
    pub fn get_void_class(&self) -> String {
        if self.is_rate_disabled() {
            self.props.disabled_void_icon_class.clone()
        } else {
            self.props.void_icon_class.clone()
        }
    }
    pub fn get_classes(&self, item:i32)->Vec<String> {
        let mut result = vec![];
        let mut threshold = js_sys::Math::ceil(self.current_value) as i32;

        if self.props.allow_half && self.current_value !=js_sys::Math::floor(self.current_value) {
            threshold -=1;
        }
        for _ in 0..threshold {
            let get_active_class =  self.get_active_class();
            result.push(get_active_class);
        }

        for _ in threshold..self.props.max {
            result.push(self.get_void_class());
        }

        vec!["el-rate__icon".to_string(), result[item as usize].clone()]
    }

    pub fn get_text(&self) ->String {
        if self.props.show_score {
            if self.is_rate_disabled() {
                return format!("{}", self.props.value);
            }
            return format!("{}", self.current_value);
        } else if self.props.show_text && self.current_value as i32 >0 {
            let s = self.props.texts[(js_sys::Math::ceil(self.current_value) as usize)-1].clone();
            return s;
        }
        "".to_string()
    }

    pub fn get_decimal_style(&self) -> String {
        let width;
        if self.is_rate_disabled() {
            width = format!("{}%", self.get_value_decimal());
        } else if self.props.allow_half {
            width = "50%".to_string();
        } else {
            width = "".to_string();
        }
        format!("color: {}; width: {}", self.get_active_color(), width)
    }

    pub fn get_value_decimal(&self) -> f64 {
        return self.props.value * 100.0 - js_sys::Math::floor(self.props.value) * 100.0;
    }

    pub fn is_show_decimal_icon(&self, item:i32) -> bool {
        let show_when_disabled = self.is_rate_disabled() && self.get_value_decimal()>0.0 && (item as f64)-1.0<self.props.value && (item as f64) >self.props.value;
        let show_when_allow_half = self.props.allow_half && self.pointer_at_left_half && (item as f64)-0.5<=self.current_value && (item as f64)>self.current_value;
        return  show_when_disabled || show_when_allow_half;
    }

    pub fn get_decimal_icon_class(&self) ->String {
        let s = self.get_value_from_map(self.props.value as i32, self.get_class_map());
        format!("el-rate__decimal {}", s)
    }

    pub fn is_rate_disabled(&self) -> bool {
        // TODO 原始实现会有elForm的判断，目前没有实现
        // return this.disabled || (this.elForm || {}).disabled;
        return self.props.disabled;
    }
}

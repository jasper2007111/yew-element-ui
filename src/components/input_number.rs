use yew::prelude::*;
use gloo_console::warn;

use crate::common::YELSize;
use crate::components::YELInput;

#[derive(PartialEq, Properties)]
pub struct YELInputNumberProps {
    #[prop_or_default]
    pub size: Option<YELSize>,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or(true)]
    pub controls: bool,

    #[prop_or_default]
    pub controls_position: AttrValue,

    #[prop_or(0.0)]
    pub value: f64,

    #[prop_or(f64::MAX)]
    pub max: f64,

    #[prop_or(f64::MIN)]
    pub min: f64,

    #[prop_or(1.0)]
    pub step: f64,

    #[prop_or_default]
    pub step_strictly: bool,

    #[prop_or_default]
    pub precision: Option<i32>,

    #[prop_or_default]
    pub label: AttrValue,

    #[prop_or_default]
    pub placeholder: AttrValue,
    
    #[prop_or_default]
    pub on_change: Callback<(f64, f64)>,

    #[prop_or_default]
    pub on_blur: Callback<Event>,

    #[prop_or_default]
    pub on_focus: Callback<FocusEvent>,
}

#[function_component]
pub fn YELInputNumber(props: &YELInputNumberProps) -> Html {
    let current_value = {
        let value = props.value;
        use_state(|| value)
    };

    let set_crrent_value = {
        let current_value_clone = current_value.clone();
        let max = props.max;
        let min = props.min;
        let precision = props.precision;
        let step = props.step;
        let on_change = props.on_change.clone();
        move |new_value: f64| {
            let old_value = *current_value_clone;
            let mut value = new_value.clone();
            if let Some(p) = precision {
                value = to_precision(value, step, value, precision);
            }
            if new_value >= max {
                value = max;
            }
            if new_value <= min {
                value = min;
            }
            if old_value == value {
                return;
            }
            on_change.emit((value, old_value));
            current_value_clone.set(value);
        }
    };

    let on_decrease = {
        let value = *current_value;
        let step = props.step.clone();
        let precision = props.precision.clone();
        let set_crrent_value_clone = set_crrent_value.clone();
        Callback::from(move |e| {
            let v = decrease(value, step, precision);
            set_crrent_value_clone(v);
        })
    };
    let on_increase = {
        let value = *current_value;
        let step = props.step.clone();
        let precision = props.precision.clone();
        let set_crrent_value_clone = set_crrent_value.clone();
        Callback::from(move |e| {
            let v = increase(value, step, precision);
            set_crrent_value_clone(v);
        })
    };

    let handle_input_change = {
        let set_crrent_value_clone = set_crrent_value.clone();
        Callback::from(move |v:String| {
            let vv = v.parse::<f64>().unwrap();
            set_crrent_value_clone(vv);
        })
    };

    html! {
        <div class={get_div_classes(props)}>
            if props.controls {
                <span
                    class="el-input-number__decrease"
                    role="button"
                    onclick={on_decrease}
                >
                <i class={format!("el-icon-{}", {
                    if get_controls_at_right(props) {
                        "arrow-down"
                    } else {
                        "minus"
                    }
                })}></i>
                </span>
                <span
                    class="el-input-number__increase"
                    role="button"
                    onclick={on_increase}
                >
                <i class={format!("el-icon-{}", {
                    if get_controls_at_right(props) {
                        "arrow-down"
                    } else {
                        "plus"
                    }
                })}></i>
                </span>
            }
            <YELInput 
                value={get_display_value(*current_value, props.step_strictly, props.step)} 
                on_change={handle_input_change}
                on_blur={props.on_blur.clone()}
                on_focus={props.on_focus.clone()}
                placeholder={&props.placeholder}
            />
        </div>
    }
}

fn decrease(val: f64, step: f64, precision: Option<i32>) -> f64 {
    let precision_factor = js_sys::Math::pow(10.0, get_num_precision(val, step, precision));
    let num = (precision_factor * val - precision_factor * step as f64)/precision_factor;
    return to_precision(val, step, num, precision);
}

fn increase(val: f64, step: f64, precision: Option<i32>) -> f64 {
    let precision_factor = js_sys::Math::pow(10.0, get_num_precision(val, step, precision));
    let num = (precision_factor * val + precision_factor * step as f64)/precision_factor;
    return to_precision(val, step, num, precision);
}

fn to_precision(
    val: f64,
    step: f64,
    num: f64,
    precision_option: Option<i32>,
) -> f64 {
    let precision = if let Some(p) = precision_option {
        p
    } else {
        get_num_precision(val, step, precision_option) as i32
    };
    let a = js_sys::Math::round(num * js_sys::Math::pow(10.0, precision as f64));
    let b = js_sys::Math::pow(10.0, precision as f64);
    return a / b;
}

fn get_precision(val: f64) -> i32 {
    let value_string = val.to_string();
    let dot_position = value_string.find('.');
    if let Some(p) = dot_position {
        return (value_string.len() as i32) - (p as i32) - 1;
    }
    0
}

fn get_num_precision(value: f64, step: f64, precision: Option<i32>) -> f64 {
    let step_precision = get_precision(step);
    if let Some(precision) = precision {
        if step_precision > precision {
            // TODO
            warn!("[InputNumber] precision should not be less than the decimal places of step");
        }
        return precision.into();
    }
    return js_sys::Math::max(get_precision(value).into(), step_precision.into());
}

fn get_display_value(current_value: f64, step_strictly: bool, step: f64) -> String {
    let mut current_value_mut = current_value;

    if step_strictly {
        let step_precision = get_precision(step);
        let precision_factor = js_sys::Math::pow(10.0, step_precision as f64);
        current_value_mut = js_sys::Math::round(current_value_mut / step) * precision_factor * step / precision_factor;
    }

    // if (this.precision !== undefined) {
    // currentValue = currentValue.toFixed(this.precision);
    // }
    
    format!("{}", current_value_mut)
}
fn get_div_classes(props: &YELInputNumberProps) -> Vec<String> {
    let mut classes = vec!["el-input-number".to_string()];

    let size = get_input_number_size(props);
    if !size.is_empty() {
        classes.push(format!("el-input-number--{}", size));
    }

    let disabled = get_input_number_disabled(props);
    if disabled {
        classes.push("is-disabled".to_string());
    }

    if !props.controls {
        classes.push("is-without-controls".to_string());
    }

    if get_controls_at_right(props) {
        classes.push("is-controls-right".to_string());
    }

    classes
}

fn get_input_number_size(props: &YELInputNumberProps) -> String {
    if let Some(s) = &props.size {
        return s.to_string();
    }
    String::default()
}

fn get_input_number_disabled(props: &YELInputNumberProps) -> bool {
    return props.disabled;
}

fn get_controls_at_right(props: &YELInputNumberProps) -> bool {
    props.controls && props.controls_position == "right"
}

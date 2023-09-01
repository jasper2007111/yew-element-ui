use yew::prelude::*;

#[derive(Clone, PartialEq, Debug, Eq)]
pub enum YELProgressType {
    Line,
    Circle,
    Dashboard,
}

impl std::fmt::Display for YELProgressType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let result = match *self {
            YELProgressType::Line => "line",
            YELProgressType::Circle => "circle",
            YELProgressType::Dashboard => "dashboard",
        };
        write!(f, "{}", result)
    }
}

#[derive(Clone, PartialEq, Debug, Eq)]
pub enum YELProgressStatus {
    Success,
    Exception,
    Warning,
}

impl std::fmt::Display for YELProgressStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let result = match *self {
            YELProgressStatus::Success => "success",
            YELProgressStatus::Exception => "exception",
            YELProgressStatus::Warning => "warning",
        };
        write!(f, "{}", result)
    }
}

#[derive(Clone, PartialEq, Debug, Eq)]
pub enum YELProgressStrokeLinecap {
    Butt,
    Round,
    Square,
}

impl std::fmt::Display for YELProgressStrokeLinecap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let result = match *self {
            YELProgressStrokeLinecap::Butt => "butt",
            YELProgressStrokeLinecap::Round => "round",
            YELProgressStrokeLinecap::Square => "square",
        };
        write!(f, "{}", result)
    }
}

#[derive(PartialEq, Properties)]
pub struct YELProgressProps {
    #[prop_or(Some(YELProgressType::Line))]
    pub progress_type: Option<YELProgressType>,

    #[prop_or_default]
    pub status: Option<YELProgressStatus>,

    #[prop_or_default]
    pub percentage: f64,

    #[prop_or(6.0)]
    pub stroke_width: f64,

    #[prop_or("#ebeef5".to_string())]
    pub define_back_color: String,

    #[prop_or_default]
    pub color: String,

    #[prop_or(true)]
    pub show_text: bool,

    #[prop_or_default]
    pub text_inside: bool,

    #[prop_or("#606266".to_string())]
    pub text_color: String,

    #[prop_or(126.0)]
    pub width: f64,

    #[prop_or(Some(YELProgressStrokeLinecap::Round))]
    pub stroke_linecap: Option<YELProgressStrokeLinecap>,
    
}

#[function_component]
pub fn YELProgress(props: &YELProgressProps) -> Html {
    html! {
        <div
            class={get_div_classes(props)}
            role="progressbar"
            aria-valuenow={format!("{}", props.percentage.clone())}
            aria-valuemin="0"
            aria-valuemax="100">
            if let Some(t) = props.progress_type.clone()  {
                if t == YELProgressType::Line {
                    <div class="el-progress-bar">
                        <div class="el-progress-bar__outer" 
                             style={format!("height: {}px; background-color:{}", props.stroke_width, props.define_back_color)}>
                            <div class="el-progress-bar__inner" style={get_bar_style(props)}>
                                if props.show_text && props.text_inside {
                                    <div
                                        class="el-progress-bar__innerText"
                                        style={format!("color: {};", props.text_color)}>{get_content(props)}</div>
                                }
                            </div>
                        </div>
                    </div>
                } else {
                    <div class="el-progress-circle" style={format!("height: {}px; width: {}px;", props.width, props.width)}>
                        <svg viewBox="0 0 100 100">
                            <path
                              class="el-progress-circle__track"
                              d={get_track_path(props)}
                              stroke={props.define_back_color.clone()}
                              stroke-width={format!("{}", get_relative_stroke_width(props))}
                              fill="none"
                              style={get_trail_path_style(props)}></path>
                            <path
                              class="el-progress-circle__path"
                              d={get_track_path(props)}
                              stroke={get_stroke(props)}
                              fill="none"
                              stroke-linecap={format!("{}", props.stroke_linecap.clone().unwrap())}
                              stroke-width={
                                format!("{}", get_relative_stroke_width(props))
                              }
                              style={get_circle_path_style(props)}></path>
                        </svg>
                    </div>
                }
                if props.show_text && !props.text_inside {
                    <div 
                        class="el-progress__text"
                        style={format!("font-size: {}px; color: {}", get_progress_text_size(props), props.text_color)}>
                        if props.status.is_some() {
                            <i class={get_icon_class(props)}></i>
                        } else {
                            { get_content(props) }
                        }
                    </div>
                }
             }
        </div>
    }
}

fn get_icon_class(props: &YELProgressProps) -> String {
    if let Some(s) = &props.status  {
        if *s == YELProgressStatus::Warning {
            return "el-icon-warning".to_string();
        }

        let progress_type = props.progress_type.clone().unwrap();
        if progress_type == YELProgressType::Line {
            if *s == YELProgressStatus::Success {
                return "el-icon-circle-check".to_string();
            } else {
                return "el-icon-circle-close".to_string();
            }
        } else {
            if *s == YELProgressStatus::Success {
                return "el-icon-check".to_string();
            } else {
                return "el-icon-close".to_string();
            }
        }
    }

    "".to_string()
}

fn get_progress_text_size(props: &YELProgressProps) -> f64 {
    let progress_type = props.progress_type.clone().unwrap();
    if progress_type == YELProgressType::Line {
        12.0 + props.stroke_width * 0.4
    } else {
        props.width * 0.111111 + 2.0
    }
}

fn get_circle_path_style(props: &YELProgressProps) -> String {
    let perimeter = get_perimeter(props);
    let f = perimeter * get_rate(props) * (props.percentage / 100.0);
    let mut str = format!("stroke-dasharray: {}px {}px;", f, perimeter);

    str.push_str(&format!("stroke-dashoffset: {};", get_stroke_dashoffset(props)));
    str.push_str("transition: stroke-dasharray 0.6s ease 0s, stroke 0.6s ease");
    str
}

fn get_stroke(props: &YELProgressProps) -> String {
    let ret = if !props.color.is_empty() {
        get_current_color(props)
    } else {
        if let Some(s) = props.status.clone() {
            let result = match s {
                YELProgressStatus::Success => "#13ce66".to_string(),
                YELProgressStatus::Exception => "#ff4949".to_string(),
                YELProgressStatus::Warning => "#e6a23c".to_string(),
            };
            result
        } else {
            "#20a0ff".to_string()
        }
    };

    return ret;
}

fn get_rate(props: &YELProgressProps) -> f64 {
    let progress_type = props.progress_type.clone().unwrap();
    if progress_type == YELProgressType::Dashboard {
        return 0.75;
    }
    1.0
}

fn get_perimeter(props: &YELProgressProps) -> f64 {
    return 2.0 * std::f64::consts::PI * get_radius(props) as f64;
}

fn get_stroke_dashoffset(props: &YELProgressProps) -> String {
    let perimeter = get_perimeter(props);
    let offset = -1.0 * perimeter * (1.0 - get_rate(props)) / 2.0;
    format!("{}px", offset)
}

fn get_trail_path_style(props: &YELProgressProps) -> String {
    let perimeter = get_perimeter(props);
    format!(
        "stroke-dasharray: {} {}; stroke-dashoffset: {};",
        perimeter * get_rate(props),
        perimeter,
        get_stroke_dashoffset(props)
    )
}

fn get_track_path(props: &YELProgressProps) -> String {
    let radius = get_radius(props);
    let progress_type = props.progress_type.clone().unwrap();
    let is_dashboard = if progress_type == YELProgressType::Dashboard {
        true
    } else {
        false
    };

    let mut str = "M 50 50".to_string();
    if is_dashboard {
        str.push_str(&format!("\nm 0 {}", radius));
        str.push_str(&format!(
            "\na {} {} 0 1 1 0 {}",
            radius,
            radius,
            radius * 2
        ));
        str.push_str(&format!(
            "\na {} {} 0 1 1 0 -{}",
            radius,
            radius,
            radius * 2
        ));
    } else {
        str.push_str(&format!("\nm 0 -{}", radius));
        str.push_str(&format!(
            "\na {} {} 0 1 1 0 {}",
            radius,
            radius,
            radius * 2
        ));
        str.push_str(&format!(
            "\na {} {} 0 1 1 0 -{}",
            radius,
            radius,
            radius * 2
        ));
    }

    str
}

fn get_radius(props: &YELProgressProps) -> i32 {
    if let Some(t) = props.progress_type.clone() {
        if t == YELProgressType::Circle || t == YELProgressType::Dashboard {
            let d = 50.0 - get_relative_stroke_width(props) / 2.0;
            return d as i32;
        }
    }
    0
}

fn get_relative_stroke_width(props: &YELProgressProps) -> f64 {
    props.stroke_width / props.width * 100.0
}

fn get_content(props: &YELProgressProps) -> String {
    // if (typeof this.format === 'function') {
    //     return this.format(this.percentage) || '';
    // } else {
    //     return `${this.percentage}%`;
    // }

    format!("{}%", props.percentage)
}

fn get_bar_style(props: &YELProgressProps) -> String {
    format!(
        "width: {}%; background-color: {}",
        props.percentage,
        get_current_color(props)
    )
}

fn get_current_color(props: &YELProgressProps) -> String {
    props.color.clone()
}

fn get_div_classes(props: &YELProgressProps) -> Vec<String> {
    let mut classes = vec!["el-progress".to_string()];

    if let Some(t) = props.progress_type.clone() {
        classes.push(format!("el-progress--{}", t));
    }

    if let Some(s) = props.status.clone() {
        classes.push(format!("is-{}", s));
    }

    classes
}

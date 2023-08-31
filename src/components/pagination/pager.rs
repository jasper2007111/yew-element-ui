use web_sys::HtmlElement;
use yew::prelude::*;
use gloo_console::log;

#[derive(PartialEq, Properties)]
pub struct YELPagerProps {
    #[prop_or_default]
    pub current_page: i32,

    #[prop_or_default]
    pub page_count: i32,

    #[prop_or_default]
    pub pager_count: i32,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub on_change: Callback<i32>
}

#[function_component]
pub fn YELPager(props: &YELPagerProps) -> Html {
    // let show_prev_more = use_state(|| false);
    // let show_next_more = use_state(|| false);
    let mut show_prev_more = false;
    let mut show_next_more = false;

    let quickprev_icon_class = use_state(||"el-icon-more".to_string());
    let quicknext_icon_class = use_state(||"el-icon-more".to_string());

    let pagers = {
        let pager_count = props.pager_count.clone();
        let half_pager_count = (pager_count - 1) / 2;

        let current_page = props.current_page.clone();
        let page_count = props.page_count.clone();

        let mut show_prev_more_tmp = false;
        let mut show_next_more_tmp = false;

        if page_count > pager_count {
            if current_page > pager_count - half_pager_count {
                show_prev_more_tmp = true;
            }

            if current_page < page_count - half_pager_count {
                show_next_more_tmp = true;
            }
        }

        let mut array = vec![];

        if show_prev_more_tmp && !show_next_more_tmp {
            let start_page = page_count - (pager_count - 2);
            for i in start_page..page_count {
                array.push(i);
            }
          } else if !show_prev_more_tmp && show_next_more_tmp {
            for i in 2..pager_count {
                array.push(i);
            }
          } else if show_prev_more_tmp && show_next_more_tmp {
            let offset = (js_sys::Math::floor(pager_count as f64 / 2.0) - 1.0) as i32;
            for i in (current_page - offset)..=(current_page + offset) {
                array.push(i);
            }
          } else {
            for i in 2..page_count {
                array.push(i);
            }
          }
  
          show_prev_more = show_prev_more_tmp;
          show_next_more = show_next_more_tmp;
  
          array
    };

    let next_btn_callback = {
        let page_count = props.page_count.clone();
        let pager_count = props.pager_count.clone();
        let current_page = props.current_page.clone();
        let on_change = props.on_change.clone();
        Callback::from( move |e: MouseEvent| {
            if let Some(h) = e.target_dyn_into::<HtmlElement>() {
                if h.tag_name() == "LI" {
                    let mut new_page = 0;

                    let pager_count_offset = pager_count - 2;
                    if h.class_name().contains("more") {
                        if h.class_name().contains("quickprev") {
                            new_page = current_page - pager_count_offset;
                        } else if h.class_name().contains("quicknext") {
                            new_page = current_page + pager_count_offset;
                        }
                    } else {
                        if let Some(s) = h.text_content() {
                            if let Ok(n) = s.parse::<i32>() {
                                new_page = n;
                            }
                        }
                    }

                    if new_page<1 {
                        new_page = 1;
                    }

                    if new_page>page_count {
                        new_page = page_count;
                    }

                    if new_page != current_page {
                        on_change.emit(new_page);
                    }
                }
            }
        })
    };

    let on_mouseleave_callback = {
        let quicknext_icon_class_clone = quicknext_icon_class.clone();
        Callback::from( move |_|{
            quicknext_icon_class_clone.clone().set("el-icon-more".to_string());
        })
    };

    let on_mouseenter_callback = {
        let quicknext_icon_class_clone = quicknext_icon_class.clone();
        Callback::from( move |d|{
            if d=="right" {
                quicknext_icon_class_clone.clone().set("el-icon-d-arrow-right".to_string());
            } else {
                quicknext_icon_class_clone.clone().set("el-icon-d-arrow-left".to_string());
            }
        })
    };

    let on_mouseenter_callback_clone = on_mouseenter_callback.clone();

    html! {
        <ul class="el-pager" onclick={next_btn_callback}>
            if props.page_count > 0{
                <li class={classes!(get_li_class_1st(props.current_page==1, props.disabled))}>{"1"}</li>
            }
            if show_prev_more {
                <li 
                    class={classes!(get_more_classes(true, quickprev_icon_class.to_string(), props.disabled.clone()))}
                    onmouseenter={Callback::from( move |_|{
                        on_mouseenter_callback_clone.emit("left");
                    })} 
                    onmouseleave={on_mouseleave_callback.clone()} >

                </li>
            }
            {
                pagers.into_iter().map(|m| {
                    let classes = get_li_class_1st(props.current_page.clone()==m, props.disabled.clone());
                    html!(
                        <li class={classes!(classes)}>{m}</li>
                    )
                }).collect::<Html>()
            }
            if show_next_more {
                <li 
                    class={classes!(get_more_classes(false, quicknext_icon_class.to_string(), props.disabled.clone()))}
                    onmouseenter={Callback::from( move |_|{
                        on_mouseenter_callback.emit("right");
                    })} 
                    onmouseleave={on_mouseleave_callback.clone()} >
                </li>
            }
            if props.page_count > 1 {
                <li class={classes!(get_li_class_last(props.current_page==props.page_count, props.disabled))}>{props.page_count.clone()}</li>
            }
        </ul>
    }
}

fn get_li_class_1st(active: bool, disabled: bool) -> Vec<String> {
    let mut vec = vec!["number".to_string()];
    if active {
        vec.push("active".to_string());
    }
    if disabled {
        vec.push("disabled".to_string());
    }
    vec
}

fn get_more_classes(is_prev: bool, icon_class: String, disabled: bool) -> Vec<String> {
    let mut vec = vec!["el-icon".to_string()];
    vec.push("more".to_string());

    if is_prev {
        vec.push("btn-quickprev".to_string());
    } else {
        vec.push("btn-quicknext".to_string());
    }

    vec.push(icon_class);
    if disabled {
        vec.push("disabled".to_string());
    }

    vec
}

fn get_li_class_last(active: bool, disabled: bool) -> Vec<String> {
    let mut vec = vec!["number".to_string()];
    if active {
        vec.push("active".to_string());
    }
    if disabled {
        vec.push("disabled".to_string());
    }
    vec
}

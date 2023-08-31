use yew::prelude::*;
use gloo_console::log;

use super::pager::YELPager;

#[derive(PartialEq, Properties)]
pub struct YELPaginationProps {
    #[prop_or_default]
    pub total: i32,

    #[prop_or(7)]
    pub pager_count: i32,

    #[prop_or(10)]
    pub page_size: i32,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    on_prev_click: Callback<i32>,

    #[prop_or_default]
    on_next_click: Callback<i32>,

    #[prop_or_default]
    pub on_current_change: Callback<i32>
}

#[function_component]
pub fn YELPagination(props: &YELPaginationProps) -> Html {
    let internal_current_page = use_state(|| 1);
    let internal_page_size = use_state(|| 0);

    let internal_page_count = {
        js_sys::Math::max(1.0, js_sys::Math::ceil(props.total as f64 / props.page_size as f64)) as i32
        // if (typeof this.total === 'number') {
        //     return Math.max(1, Math.ceil(this.total / this.internalPageSize));
        //   } else if (typeof this.pageCount === 'number') {
        //     return Math.max(1, this.pageCount);
        //   }
        //   return null;
    };

    let prev_btn_callback = {
        let internal_current_page_clone = internal_current_page.clone();
        let disabled = props.disabled;
        let on_current_change_clone = props.on_current_change.clone();
        Callback::from( move |_e| {
            if !disabled {
                let mut new_val = *internal_current_page_clone.clone() - 1;
                if new_val<1 {
                    new_val = 1;
                }
                internal_current_page_clone.set(new_val);
                on_current_change_clone.emit(new_val);
            }
        })
    };

    let next_btn_callback = {
        let disabled = props.disabled;
        let internal_current_page_clone = internal_current_page.clone();
        let on_current_change_clone = props.on_current_change.clone();
        Callback::from( move |_e| {
            if !disabled {
                let mut new_val = *internal_current_page_clone + 1;
                if new_val > internal_page_count {
                    new_val = *internal_current_page_clone;
                }
                internal_current_page_clone.set(new_val);
                on_current_change_clone.emit(new_val);
            }
        })
    };

    let pager_callback = {
        let internal_current_page_clone = internal_current_page.clone();
        let on_current_change_clone = props.on_current_change.clone();
        Callback::from( move |v| {
            internal_current_page_clone.set(v);
            on_current_change_clone.emit(v);
        })
    };

    let prev = html!(
        <button
        type="button"
        class="btn-prev"
        onclick={prev_btn_callback}
        disabled={ *internal_current_page.clone() <= 1 }>
        <i class="el-icon el-icon-arrow-left"></i>
        </button>
    );

    let next = html!(
        <button
        type="button"
        class="btn-next"
        onclick={next_btn_callback.clone()}
        disabled={ *internal_current_page >= internal_page_count }>
        <i class="el-icon el-icon-arrow-right"></i>
        </button>
    );

    html! {
        <div class="el-pagination">
            {prev}
            <YELPager
                current_page={ *internal_current_page.clone() }
                page_count={ internal_page_count}
                pager_count={ props.pager_count.clone() }
                on_change={ pager_callback } />
            {next}
        </div>
    }
}
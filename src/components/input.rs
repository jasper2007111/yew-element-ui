use std::collections::HashMap;
use web_sys::{HtmlElement, HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use yew::virtual_dom::VNode;

use std::cell::RefCell;

pub enum YELInputMsg {
    None,
    OnInput(InputEvent),
    OnFocus(FocusEvent),
    OnBlur(Event),
    OnChange(Event),
    OnClear(),
    OnMouseEnter(),
    OnMouseLeave(),
    OnHandlePasswordVisible(),
}

pub struct YELInput {
    textarea_ref: NodeRef,
    input_ref: NodeRef,
    password_visible: bool,
    hovering: bool,
    focused: bool,
    need_focus: bool,
    // 缓存查找的
    slot_map: RefCell<HashMap<String, bool>>,
    props: YELInputProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct YELInputProps {
    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub value: String,

    #[prop_or("text".to_string())]
    pub input_type: String,

    #[prop_or_default]
    pub size: String,

    #[prop_or_default]
    pub tabindex: String,

    #[prop_or_default]
    pub readonly: bool,

    #[prop_or_default]
    pub show_password: bool,

    #[prop_or_default]
    pub show_word_limit: bool,

    // 前缀
    #[prop_or_default]
    pub prefix_icon: String,

    // 后缀
    #[prop_or_default]
    pub suffix_icon: String,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub placeholder: String,

    #[prop_or_default]
    pub on_input: Callback<String>,

    #[prop_or_default]
    pub on_focus: Callback<FocusEvent>,

    #[prop_or_default]
    pub on_blur: Callback<Event>,

    #[prop_or_default]
    pub on_change: Callback<String>,

    // 输入框关联的label文字
    #[prop_or_default]
    pub label: String,

    #[prop_or("off".to_string())]
    pub autocomplete: String,

    #[prop_or(None)]
    pub max_length: Option<i32>,

    #[prop_or(None)]
    pub min_length: Option<i32>,

    #[prop_or_default]
    pub clearable: bool,

    #[prop_or(None)]
    pub rows: Option<u32>,

    #[prop_or_default]
    pub root_ref: NodeRef,
}

impl Component for YELInput {
    type Message = YELInputMsg;
    type Properties = YELInputProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            need_focus: false,
            hovering: false,
            slot_map: RefCell::new(HashMap::default()),
            textarea_ref: NodeRef::default(),
            input_ref: NodeRef::default(),
            password_visible: false,
            focused: false,
            props: ctx.props().clone(),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.props = ctx.props().clone();
        return true;
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            YELInputMsg::None => false,
            YELInputMsg::OnInput(e) => {
                let target: HtmlInputElement = e.target_unchecked_into();
                self.props.value = target.value().clone();
                self.props.on_input.emit(target.value().clone());
                true
            }
            YELInputMsg::OnFocus(e) => {
                self.focused = true;
                self.props.on_focus.emit(e);
                false
            }
            YELInputMsg::OnBlur(e) => {
                self.focused = false;
                self.props.on_blur.emit(e);
                // TODO 原版的这个没用实现
                // if (this.validateEvent) {
                //     this.dispatch('ElFormItem', 'el.form.blur', [this.value]);
                // }
                false
            }
            YELInputMsg::OnChange(e) => {
                let target: HtmlInputElement = e.target_unchecked_into();
                self.props.value = target.value().clone();
                self.props.on_change.emit(target.value().clone());
                true
            }
            YELInputMsg::OnClear() => {
                self.props.on_input.emit("".to_string());
                self.props.on_change.emit("".to_string());
                self.props.value = "".to_string();

                if self.props.input_type == "textarea" {
                    let textarea = self.textarea_ref.cast::<HtmlTextAreaElement>().unwrap();
                    textarea.set_value("");
                } else if self.props.input_type == "text" {
                    let input = self.input_ref.cast::<HtmlInputElement>().unwrap();
                    input.set_value("");
                }
                false
            }
            YELInputMsg::OnMouseEnter() => {
                // log!("OnMouseEnter");
                self.hovering = true;
                false
            }
            YELInputMsg::OnMouseLeave() => {
                // log!("OnMouseLeave");
                self.hovering = false;
                false
            }
            YELInputMsg::OnHandlePasswordVisible() => {
                self.password_visible = !self.password_visible;
                // 此处标记渲染结束后执行
                self.need_focus = true;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let classes = self.get_root_div_classes();
        html! {
            <div
                ref= {&self.props.root_ref}
                class={classes!(classes.clone())}
                onmouseenter={ctx.link().callback(|_|{
                    YELInputMsg::OnMouseEnter()
                })}
                onmouseleave={ctx.link().callback(|_|{
                    YELInputMsg::OnMouseLeave()
                })}
            >
                if self.props.input_type == "textarea" {
                    <textarea
                        tabindex = {self.props.tabindex.clone()}
                        value = {self.props.value.clone()}
                        class = "el-textarea__inner"
                        oninput = {ctx.link().callback(|e| {
                            YELInputMsg::OnInput(e)
                        })}
                        ref = {&self.textarea_ref}
                        disabled = {self.is_input_disabled()}
                        readonly = {self.props.readonly}
                        autocomplete = {self.props.autocomplete.clone()}
                        oninput = {ctx.link().callback(|e|{
                            YELInputMsg::OnInput(e)
                        })}
                        onfocus = {ctx.link().callback(|e|{
                            YELInputMsg::OnFocus(e)
                        })}
                        onblur = {ctx.link().callback(|e|{
                            YELInputMsg::OnFocus(e)
                        })}
                        onchange = {ctx.link().callback(|e|{
                            YELInputMsg::OnChange(e)
                        })}
                        placeholder = {self.props.placeholder.clone()}
                        aria-label = {self.props.label.clone()}
                    >
                    </textarea>
                    if self.is_word_limit_visible() {
                        <span class="el-input__count">{format!("{}/{}", self.get_text_length(), self.props.max_length.unwrap())}</span>
                    }
                } else {
                    {self.get_input_node(ctx)}
                }
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if self.props.input_type == "textarea" {
                let textarea = self.textarea_ref.cast::<HtmlTextAreaElement>().unwrap();
                if let Some(maxlength) = self.props.max_length {
                    textarea.set_max_length(maxlength);
                }

                if let Some(minlenght) = self.props.min_length {
                    textarea.set_min_length(minlenght);
                }

                if !self.props.value.is_empty() {
                    textarea.set_value(&self.props.value);
                }

                if let Some(value) = self.props.rows {
                    textarea.set_rows(value);
                }
            } else if self.props.input_type == "text" {
                let input = self.input_ref.cast::<HtmlInputElement>().unwrap();
                if let Some(maxlength) = self.props.max_length {
                    input.set_max_length(maxlength);
                }

                if let Some(minlenght) = self.props.min_length {
                    input.set_min_length(minlenght);
                }

                if !self.props.value.is_empty() {
                    input.set_value(&self.props.value);
                }
            }
        }
        if self.need_focus {
            self.need_focus = false;
            if self.props.input_type == "textarea" {
                let textarea = self.textarea_ref.cast::<HtmlTextAreaElement>().unwrap();
                let _ = textarea.focus();
            } else if self.props.input_type == "text" {
                let input = self.input_ref.cast::<HtmlInputElement>().unwrap();
                let _ = input.focus();
            }
        }
    }
}

impl YELInput {
    pub fn show_clear(&self) -> bool {
        // TODO 未实现全
        // return this.clearable &&
        //   !this.inputDisabled &&
        //   !this.readonly &&
        //   this.nativeInputValue &&
        //   (this.focused || this.hovering);
        let flag = self.props.clearable
            && !self.is_input_disabled()
            && !self.props.readonly
            && (self.hovering || self.focused);
        return flag;
    }
    pub fn get_text_length(&self) -> usize {
        let len = self.props.value.len();
        return len;
    }
    pub fn get_root_div_classes(&self) -> Vec<String> {
        let mut classes: Vec<_> = vec![];
        if self.props.input_type == "textarea" {
            classes.push("el-textarea".to_string());
        } else {
            classes.push("el-input".to_string());
        }

        if !self.props.size.is_empty() {
            classes.push(format!("el-input--{}", self.props.size));
        }
        if self.is_input_disabled() {
            classes.push("is-disabled".to_string());
        }
        if self.is_input_exceed() {
            classes.push("is-exceed".to_string());
        }

        let has_prepend = self.has_slot("prepend".to_string());
        let has_append = self.has_slot("append".to_string());
        if has_prepend || has_append {
            classes.push("el-input-group".to_string());
            if has_append {
                classes.push("el-input-group--append".to_string());
            }
            if has_prepend {
                classes.push("el-input-group--prepend".to_string());
            }
        }

        let has_prefix = self.has_slot("prefix".to_string());
        if has_prefix || !self.props.prefix_icon.is_empty() {
            classes.push("el-input--prefix".to_string());
        }

        let has_suffix = self.has_slot("suffix".to_string());
        if has_suffix
            || !self.props.suffix_icon.is_empty()
            || self.props.clearable
            || self.props.show_password
        {
            classes.push("el-input--suffix".to_string());
        }

        classes
    }

    pub fn is_input_disabled(&self) -> bool {
        return self.props.disabled;
    }

    pub fn get_upper_limit(&self) -> i32 {
        if let Some(len) = self.props.max_length {
            return len;
        }
        return 0;
    }

    pub fn is_input_exceed(&self) -> bool {
        return self.is_word_limit_visible()
            && (self.get_text_length() as i32 > self.get_upper_limit());
    }

    pub fn get_suffix_visible(&self) -> bool {
        // (this.validateState && this.needStatusIcon);

        let has_suffix = self.has_slot("suffix".to_string());
        if has_suffix
            || !self.props.suffix_icon.is_empty()
            || self.show_clear()
            || self.props.show_password
            || self.is_word_limit_visible()
        {
            return true;
        }

        return false;
    }

    pub fn show_pwd_visible(&self) -> bool {
        // return this.showPassword &&
        // !this.inputDisabled &&
        // !this.readonly &&
        // (!!this.nativeInputValue || this.focused);

        return self.props.show_password
            && !self.is_input_disabled()
            && !self.props.readonly
            && self.focused;
    }

    pub fn is_word_limit_visible(&self) -> bool {
        let flag = self.props.show_word_limit
            && self.props.max_length.is_some()
            && (self.props.input_type == "text" || self.props.input_type == "textarea")
            && !self.is_input_disabled()
            && !self.props.readonly
            && !self.props.show_password;
        flag
    }

    pub fn get_slot(&self, slot_name: String) -> Option<VNode> {
        for i in self.props.children.clone().into_iter() {
            match i {
                VNode::VTag(ref vtag) => {
                    match vtag.attributes {
                        yew::virtual_dom::Attributes::Static(vev) => {
                            for g in vev {
                                if g.0 == "slot" {
                                    // log!(format!("{:?}", g.1));
                                    if g.1 == slot_name {
                                        return Some(i);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        None
    }

    pub fn has_slot(&self, name: String) -> bool {
        // TODO 此处原本打算缓存一下，但发现这个最终会在view的方法里调用，而那个是不可修改的self。
        // 这个也让我知道了，Rust的一些使用上的问题，安全的代价可能比想象的高。
        // 不过此处问题已经通过RefCell的方式解决
        let clone_name = name.clone();
        let mut ref_cell = self.slot_map.borrow_mut();
        let c = ref_cell.get(&clone_name.clone());
        if c.is_some() {
            return *c.unwrap();
        }
        match self.get_slot(name) {
            Some(_) => {
                ref_cell.insert(clone_name.clone(), true);
                return true;
            }
            None => {
                ref_cell.insert(clone_name.clone(), false);
                return false;
            }
        }
    }

    pub fn get_input_node(&self, ctx: &Context<YELInput>) -> VNode {
        let input_type;
        if self.props.show_password {
            if self.password_visible {
                input_type = "text".to_string()
            } else {
                input_type = "password".to_string()
            }
        } else {
            input_type = self.props.input_type.clone()
        };
        let has_prepend = self.has_slot("prepend".to_string());
        let has_append = self.has_slot("append".to_string());
        let has_prefix = self.has_slot("prefix".to_string());
        let has_suffix = self.has_slot("suffix".to_string());
        return html!(
            <>
                // 前置元素
                if has_prepend {
                    <div class="el-input-group__prepend">
                        {self.get_slot("prepend".to_string()).unwrap().clone()}
                    </div>
                }
                <input
                    tabindex={self.props.tabindex.clone()}
                    disabled={self.is_input_disabled()}
                    value = {self.props.value.clone()}
                    type = {input_type}
                    class = "el-input__inner"
                    readonly = {self.props.readonly}
                    ref = {&self.input_ref}
                    autocomplete = {self.props.autocomplete.clone()}
                    oninput = {ctx.link().callback(|e|{
                        YELInputMsg::OnInput(e)
                    })}
                    onfocus = {ctx.link().callback(|e|{
                        YELInputMsg::OnFocus(e)
                    })}
                    onblur = {ctx.link().callback(|e|{
                        YELInputMsg::OnFocus(e)
                    })}
                    onchange = {ctx.link().callback(|e|{
                        YELInputMsg::OnChange(e)
                    })}
                    placeholder = {self.props.placeholder.clone()}
                    aria-label = {self.props.label.clone()}
                />
                // 前置内容
                if has_prefix || !self.props.prefix_icon.is_empty() {
                    <span class="el-input__prefix">
                        if has_prefix {
                            {self.get_slot("prefix".to_string()).unwrap().clone()}
                        }
                        if !self.props.prefix_icon.is_empty() {
                            <i class={format!("el-input__icon {}",self.props.prefix_icon.clone())}></i>
                        }
                    </span>
                }

                // 后置内容
                if self.get_suffix_visible() {
                    <span class="el-input__suffix">
                        <span class="el-input__suffix-inner">
                            if self.show_clear() || self.show_pwd_visible() || self.is_word_limit_visible() {
                                if self.show_clear() {
                                    <i
                                        class="el-input__icon el-icon-circle-close el-input__clear"
                                        onclick={ctx.link().callback(|_|{
                                            YELInputMsg::OnClear()
                                        })}
                                        onmousedown={ctx.link().callback(|e: MouseEvent| {
                                            e.stop_propagation();
                                            YELInputMsg::None
                                        })}
                                    />
                                }
                                if self.show_pwd_visible() {
                                    <i
                                        class="el-input__icon el-icon-view el-input__clear"
                                        onclick={ctx.link().callback(|_|{
                                            YELInputMsg::OnHandlePasswordVisible()
                                        })}
                                    />
                                }
                                if self.is_word_limit_visible() {
                                    <span class="el-input__count">
                                        <span class="el-input__count-inner">
                                            {format!("{}/{}", self.get_text_length(), self.props.max_length.unwrap())}
                                        </span>
                                    </span>
                                }
                            } else {
                                if has_suffix {
                                    {self.get_slot("suffix".to_string()).unwrap().clone()}
                                }
                                if !self.props.suffix_icon.is_empty() {
                                    <i class={format!("el-input__icon {}", self.props.suffix_icon)}/>
                                }
                            }
                        </span>
                    </span>
                }
                if has_append {
                    <div class="el-input-group__append">
                        {self.get_slot("append".to_string()).unwrap().clone()}
                    </div>
                }
            </>
        );
    }

    pub fn calc_textarea_height(
        &self,
        target_element: HtmlElement,
        min_rows: Option<i32>,
        max_rows: Option<i32>,
    ) -> Option<(String, String)> {
        let hidden_textarea_result = gloo::utils::document().create_element("textarea");
        if hidden_textarea_result.is_ok() {
            let (context_style, padding_size, border_size, box_sizing) =
                self.calculate_node_styling(target_element);

            let hidden_textarea = hidden_textarea_result.unwrap();
            let body = gloo::utils::document().body().unwrap();
            let _ = body.append_child(&hidden_textarea);

            let hidden_style = "
            height:0 !important;
            visibility:hidden !important;
            overflow:hidden !important;
            position:absolute !important;
            z-index:-1000 !important;
            top:0 !important;
            right:0 !important
            "
            .to_string();
            let _ = hidden_textarea.set_attribute("style", &format!("{};{}", context_style, hidden_style));

            // hidden_textarea.set_node_value(&target_element.node_value().unwrap());

            let mut height = hidden_textarea.scroll_height() as f64;
            if box_sizing == "border-box" {
                height += border_size;
            } else if box_sizing == "content-box" {
                height -= padding_size;
            }

            hidden_textarea.set_node_value(Some(""));
            let single_row_height = hidden_textarea.scroll_height() as f64 - padding_size;

            let mut result = ("".to_string(), "".to_string());

            if min_rows.is_some() {
                let mut min_height = single_row_height * min_rows.unwrap() as f64;
                if box_sizing == "border-box" {
                    min_height += padding_size + border_size;
                }
                height = js_sys::Math::max(min_height, height);
                result.0 = format!("{}px", min_height);
            }

            if max_rows.is_some() {
                let mut max_height = single_row_height * max_rows.unwrap() as f64;
                if box_sizing == "border-box" {
                    max_height += padding_size + border_size;
                }
                height = js_sys::Math::min(max_height, height);
            }

            result.1 = format!("{}px", height);

            if hidden_textarea.parent_node().is_some() {
                let parent_node = hidden_textarea.parent_node().unwrap();
                let _ = parent_node.remove_child(&hidden_textarea);
            }
            return Some(result);
        }
        return None;
    }

    fn calculate_node_styling(&self, target_element: HtmlElement) -> (String, f64, f64, String) {
        let mut context_style = String::default();
        let mut box_sizing = String::default();
        let mut padding_size = 0.0;
        let mut border_size = 0.0;

        let window = gloo::utils::window();
        let style_result = window.get_computed_style(&target_element);
        if style_result.is_ok() {
            let style = style_result.unwrap().expect("style is null");
            let box_sizing_result = style.get_property_value("box-sizing");
            if box_sizing_result.is_ok() {
                box_sizing = box_sizing_result.unwrap();
            }

            let padding_bottom_result = style.get_property_value("padding-bottom");
            if padding_bottom_result.is_ok() {
                padding_size += js_sys::Number::parse_float(&padding_bottom_result.unwrap());
            }
            let padding_top_result = style.get_property_value("padding-top");
            if padding_top_result.is_ok() {
                padding_size += js_sys::Number::parse_float(&padding_top_result.unwrap());
            }

            let border_bottom_width_result = style.get_property_value("border-bottom-width");
            if border_bottom_width_result.is_ok() {
                border_size += js_sys::Number::parse_float(&border_bottom_width_result.unwrap());
            }

            let border_top_width_result = style.get_property_value("border-top-width");
            if border_top_width_result.is_ok() {
                border_size += js_sys::Number::parse_float(&border_top_width_result.unwrap());
            }

            let hidden_style = vec![
                "letter-spacing",
                "line-height",
                "padding-top",
                "padding-bottom",
                "font-family",
                "font-weight",
                "font-size",
                "text-rendering",
                "text-transform",
                "width",
                "text-indent",
                "padding-left",
                "padding-right",
                "border-width",
                "box-sizing",
            ];
            let mut cc = vec![];
            for name in hidden_style.into_iter() {
                let value_result = style.get_property_value(name);
                if value_result.is_ok() {
                    cc.push(format!(
                        "{}: {}",
                        name,
                        js_sys::Number::parse_float(&value_result.unwrap())
                    ));
                } else {
                    cc.push(format!("{}: 0", name))
                }
            }
            context_style = cc.join(";");
        }
        (context_style, padding_size, border_size, box_sizing)
    }
}

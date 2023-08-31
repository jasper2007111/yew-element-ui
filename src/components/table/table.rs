use yew::prelude::*;

use super::table_header::YELTableHeader;
use super::table_body::YELTableBody;
use super::table_column::YELTableColumnProps;

use web_sys::HtmlElement;

pub struct YELTable {
    props: YELTableProps,
    hidden_columns_ref: NodeRef,
    header_wrapper_ref: NodeRef,
    table_header_ref: NodeRef,
    body_wrapper_ref: NodeRef,
    table_root_ref: NodeRef,
    root_size: (f64, f64)
}

#[derive(Clone, PartialEq, Properties)]
pub struct YELTableProps {
    #[prop_or(true)]
    pub show_header: bool,
    
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub columns: Vec<YELTableColumnProps>
}

pub enum YELTableMsg {
    OnReRender
}

impl Component for YELTable {
    type Message = YELTableMsg;
    type Properties = YELTableProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
            hidden_columns_ref: NodeRef::default(),
            header_wrapper_ref: NodeRef::default(),
            table_header_ref: NodeRef::default(),
            body_wrapper_ref: NodeRef::default(),
            table_root_ref: NodeRef::default(),
            root_size: (0.0, 0.0)
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            YELTableMsg::OnReRender => {
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div ref={&self.table_root_ref} class="el-table el-table--fit el-table--enable-row-hover el-table--enable-row-transition" style="width: 100%;">
                <div class="hidden-columns"><div></div> <div></div> <div></div></div>
                <YELTableHeader size={self.root_size} columns={ctx.props().columns.clone()}/>
                <YELTableBody size={self.root_size} columns={ctx.props().columns.clone()} >
                {ctx.props().children.clone()}
                </YELTableBody>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(div) = self.table_root_ref.cast::<HtmlElement>() {
                let rect = div.get_bounding_client_rect();
                self.root_size = (rect.width(), rect.height());
                ctx.link().send_message(YELTableMsg::OnReRender);
            }
        }
    }
}
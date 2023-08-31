use yew::prelude::*;
pub struct YELTableColumn;

pub enum YELTableColumnMsg {

}

#[derive(Clone, PartialEq, Properties)]
pub struct YELTableColumnProps {
    pub label: String,

    #[prop_or(180.0)]
    pub width: f64
}

// impl YELTableColumnProps {
//     pub fn create_by_lable(lable: String ) -> Self {
//        Self { lable, width: 180.0 }
//     }

//     pub fn new(lable: String, width: f64)->Self{
//         Self { lable, width }
//     }
// }

impl Component for YELTableColumn {
    type Message = YELTableColumnMsg;
    type Properties = YELTableColumnProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <th colspan="1" rowspan="1" class="el-table_1_column_1 is-leaf el-table__cell">
            <div class="cell">{ctx.props().label.clone()}</div>
            </th>
        }
    }
}
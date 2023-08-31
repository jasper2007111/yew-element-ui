use yew::prelude::*;

use crate::utils::slot_util;
use super::table_slot::TableSlotType;

pub struct YELTableRow;

pub enum YELTableRowMsg {

}

#[derive(Clone, PartialEq, Properties)]
pub struct YELTableRowProps {
    pub data: Vec<String>,

    #[prop_or_default]
    pub children: Children
}

impl Component for YELTableRow {
    type Message = YELTableRowMsg;
    type Properties = YELTableRowProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut i = 1;

        let op = slot_util::get_comp_list_slot_by_name(ctx.props().children.clone(), format!("{:?}", TableSlotType::Operate));
        
        html! {
            <tr class="el-table__row">
            {
                ctx.props().data.clone().into_iter().map(|v|{
                    let b = i;
                    i = i + 1;
                    html!(
                        <td rowspan="1" colspan="1" class={format!("el-table_1_column_{} el-table__cell", b)}>
                            if v=="Operate" {
                                if let Some(o) = op.clone() {
                                    <div class="cell">{o}</div>
                                } else {
                                    <div class="cell"></div>
                                }
                            } else {
                                <div class="cell">{v}</div>
                            }
                        </td>)}).collect::<Html>()
            }
            </tr>
        }
    }
}
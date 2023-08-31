use yew::prelude::*;

use super::table_column::YELTableColumnProps;

pub struct YELTableHeader;

pub enum YELTableHeaderMsg {
}

#[derive(Clone, PartialEq, Properties)]
pub struct YELTableHeaderProps {
    #[prop_or_default]
    pub columns: Vec<YELTableColumnProps>,

    #[prop_or_default]
    pub size: (f64, f64)
}


impl Component for YELTableHeader {
    type Message = YELTableHeaderMsg;
    type Properties = YELTableHeaderProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut i = 1;

        let parent_width = ctx.props().size.0;
        let last_width = if parent_width == 0.0 {
            180.0
        } else {
            let mut dd = 0.0;
            for d in ctx.props().columns.clone().into_iter() {
                dd = dd + d.width;
            }

            if let Some(w) = ctx.props().columns.clone().last() {
                dd = dd - w.width;
            }

           parent_width-dd
        };

        let mut h = 0;
        let mut width_vec = vec![];
        if ctx.props().columns.len()>0 {
            if ctx.props().columns.len()>1 {
                for i in 0..ctx.props().columns.len()-1 {
                    width_vec.push(ctx.props().columns[i].width);
                }
                width_vec.push(last_width);
            } else {
                width_vec.push(parent_width);
            }
        } 

        html! {
            <div class="el-table__header-wrapper">
            <table cellspacing="0" cellpadding="0" border="0" class="el-table__header" style="width: 1200px;">
                <colgroup>
                    {ctx.props().columns.clone().into_iter().map(|_|{
                        let g = h;
                        h = h + 1;
                        html!(<col name="el-table_1_column_1" width={format!("{}", width_vec[g])}/>)
                    }).collect::<Html>()}
                </colgroup>
                <thead>
                    <tr>
                        {ctx.props().columns.clone().into_iter().map(|v|{
                            let b = i;
                            i = i + 1;
                            let label = v.label.clone();
                            html!(<th colspan="1" rowspan="1" class={format!("el-table_1_column_{} is-leaf el-table__cell", b)}>
            <div class="cell">{label}</div>
            </th>)}).collect::<Html>()}
                    </tr>
                </thead>
           </table>
           </div>
        }
    }
}
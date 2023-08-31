use yew::prelude::*;
use yew_element_ui::components::YELMenu;
use yew_element_ui::components::YELMenuItem;
use yew_element_ui::components::YELSubmenu;

#[function_component(App)]
fn app() -> Html {
    html! {
    <>
    <div>{"ddddddd"}</div>
    <YELMenu>
        <YELSubmenu level={1}>
                <div slot="title">
                    <i class="el-icon-location"></i>
                    <span>{"导航一"}</span>
                </div>
                // <el-menu-item-group>
                //     <template slot="title">分组一</template>
                //     <el-menu-item index="1-1">选项1</el-menu-item>
                //     <el-menu-item index="1-2">选项2</el-menu-item>
                // </el-menu-item-group>
                // <el-menu-item-group title="分组2">
                //     <el-menu-item index="1-3">选项3</el-menu-item>
                // </el-menu-item-group>
                <div slot="content">
                    <YELSubmenu level={2}>
                        <div slot="title">
                            <span>{"选项4"}</span>
                        </div>
                        <div slot="content">
                            <YELMenuItem level={3}>{"选项1"}</YELMenuItem>
                        </div>
                    </YELSubmenu>
                </div>
        </YELSubmenu>
        <YELMenuItem>
            <i class="el-icon-menu"></i>
            <span slot="title">{"导航二"}</span>
        </YELMenuItem>
        <YELMenuItem>
            <i class="el-icon-document"></i>
            <span slot="title">{"导航三"}</span>
        </YELMenuItem>
        <YELMenuItem>
            <i class="el-icon-setting"></i>
            <span slot="title">{"导航四"}</span>
        </YELMenuItem>
    </YELMenu>
    </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

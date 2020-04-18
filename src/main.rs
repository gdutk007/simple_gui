//use std::collections::VecDeque;

use orbtk::*;
//use orbtk::theme::DEFAULT_THEME_CSS;

//use calc;

widget!(MainView {
    background: Brush,
    count: u32,
    text: String16
});

impl Template for MainView{
    fn template(self,id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView!")
            .background("#ffffff")
            .count(0)
            .child(
                TextBlock::create()
                .width(20.0)
                .height(50.0)
                .text(id)
                .vertical_alignment("center")
                .build(ctx),
            )
            // .child(
            //     Container::create()
            //         .background("#ffffff")
            //         .border_brush("#ccc000")
            //         .border_width((100.0,20.0))
            //         .child(
            //             Container::create()
            //                 .vertical_alignment("center")
            //                 .border_width((10.0,20.0))
            //                 .background("#400000")
            //                 .border_brush("#ccc000")
            //                 .padding(50.0)
            //                 .child(
            //                     TextBlock::create()
            //                     .width(20.0)
            //                     .height(50.0)
            //                     .text(id)
            //                     .vertical_alignment("center")
            //                     .build(ctx),
            //                 )
            //                 .build(ctx),
            //         )
            //         // .child(
            //         //     Container::create()
            //         //         .vertical_alignment("end")
            //         //         .border_width((10.0,20.0))
            //         //         .background("#400FFF")
            //         //         .border_brush("#ccc000")
            //         //         .padding(50.0)
            //         //         .child(
            //         //             TextBox::create()
            //         //             .width(250.0)
            //         //             .height(50.0)
            //         //             .text("")
            //         //             .vertical_alignment("start")
            //         //             .build(ctx),
            //         //         )
            //         //         .build(ctx),
            //         // )
            //         .build(ctx),
        
            // )
    }
}

fn main(){
    Application::new()
    .window(|ctx| {
        Window::create()
            .title("OrbTk - minimal example")
            .position((100.0, 100.0))
            .size(420.0, 730.0)
            .child(MainView::create().text("This is some text").build(ctx))
            .build(ctx)
    })
    .run();
}




// impl Template for MainView {
//     fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
//         self.name("MainView")
          
//             .text("")
//             .child(
//                 Grid::create()
//                     .rows(Rows::create().row(72.0).row("*").build())
//                     .child(
//                         Container::create()
//                             .padding(8.0)
//                             .class("header")
//                             .attach(Grid::row(0))
//                             .child(
//                                 Grid::create()
//                                     .child(
//                                         ScrollViewer::create()
//                                             .scroll_viewer_mode(("custom", "disabled"))
//                                             .child(
//                                                 TextBlock::create()
//                                                     .width(0.0)
//                                                     .height(14.0)
//                                                     .text("")
//                                                     .id("input")
//                                                     .vertical_alignment("start")
//                                                     .build(ctx),
//                                             )
//                                             .build(ctx),
//                                     )
//                                     .child(
//                                         TextBlock::create()
//                                         .element("text-block")
//                                             .text(id)
//                                             .vertical_alignment("end")
//                                             .horizontal_alignment("end")
//                                             .build(ctx),
//                                     )
//                                     .build(ctx),
//                             )
//                             .build(ctx),
//                     )
//                     .child(
//                         Container::create()
//                             .class("content")
//                             .padding(8.0)
//                             .attach(Grid::row(1))
//                             .child(Grid::create())
//                             .build(ctx),
//                     )
//                     .build(ctx),
//             )
//     }
// }

//! This example uses SDL3 to render a triangle and then draw widgets over it using wtk
use sdl3::render::{FPoint, Vertex, VertexIndices};
use wtk::prelude::*;
use wtk::draw::DrawContextInternal;
use sdl3::pixels::FColor;
use sdl3;
use wtk::widgets::ChildWidget;

fn main() {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut canvas = video_subsystem.window("embedded wtk demo", 800, 600)
        .build()
        .unwrap()
        .into_canvas();

    let widgets: Vec<ChildWidget> = vec![
        Button::new("foo", |b| { b.set_text("clicked"); }).shared() as SharedWidget,
        Button::new("bar", |b| { b.set_text("clicked"); }).shared() as SharedWidget,
    ].into_iter().map(ChildWidget::new).collect();
    

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut draw = true;
    let mut wtk_origin = Point::zero();
    'main: loop {
        if draw {
            draw = false;
            canvas.set_draw_color(sdl3::pixels::Color::RGB(255, 255, 255));
            canvas.render_geometry(&[
                Vertex {
                    position: FPoint::new(400.0, 150.0),
                    color: FColor::RED,
                    tex_coord: FPoint::new(0.0, 0.0),
                },
                Vertex {
                    position: FPoint::new(200.0, 450.0),
                    color: FColor::BLUE,
                    tex_coord: FPoint::new(0.0, 0.0),
                },
                Vertex {
                    position: FPoint::new(600.0, 450.0),
                    color: FColor::GREEN,
                    tex_coord: FPoint::new(0.0, 0.0),
                },
            ], None, VertexIndices::Sequential).unwrap();
            
            // here we draw WTK widgets
            let mut ctx = wtk::draw::DrawContext::new(Point::zero());
            ctx.draw_widgets(Orientation::Vertical, 0, None, &widgets);
            let size = ctx.bounds().size();
            wtk_origin = Point::new(400 - size.width / 2, 300 - size.height / 2);
            ctx.set_zero_point(wtk_origin);
            ctx.run_backend(&mut canvas);
            canvas.present();
        }
        for e in event_pump.poll_iter().map(|e| Into::<Event>::into(e)) {
            match e {
                Event::Quit => break 'main,
                _ => {}
            }
            for child in &widgets {
                let abs_bounds = wtk_origin + child.bounds.get();
                draw |= child.widget.borrow_mut().process_event(&e, abs_bounds);
            }
        }
    }
}

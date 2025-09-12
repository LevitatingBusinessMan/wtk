extern crate wtk;
use wtk::prelude::*;

struct MousePosWidget(Point);
impl Widget for MousePosWidget {
    fn draw(&self, ctx: &mut DrawContext) {
        ctx.draw_text(format!("mouse x: {} y: {}", self.0.x, self.0.y), Point::zero());
    }
    fn process_event(&mut self, event: &Event) -> bool {
        if let Event::MouseMove(pos) = event {
            self.0 = *pos;
            true
        } else {
            false
        }
    }
}

fn main() {
    let mut app = App::<SDLBackend>::new("custom widget");
    let label = Label::new("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.");
    app.add_widget(label.shared());
    app.add_widget(MousePosWidget(Point::zero()).shared());
    app.run();
}

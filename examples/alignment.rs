extern crate wtk;
use wtk::prelude::*;

fn main() {
    let mut app = App::<SDLBackend>::new("WTK alignment example");

    let label = || Label::new("label").shared();
    let button = || Button::new("clickme", |b| { b.set_text("clicked!"); }).shared();

    let mut left = WBox::vertical();
    let mut right = WBox::vertical();
    
    let horizontal_test_widget = || WBox::horizontal()
        .border(true)
        .padding(5)
        .with( vec![
            WBox::vertical().with(vec![
                label(),
                button(),
            ]).shared(),
            button(),
        ]);
    
    let vertical_test_widget = || WBox::vertical()
        .border(true)
        .padding(5)
        .with( vec![
            WBox::horizontal().with(vec![
                label(),
                button(),
            ]).align(Alignment::Center).shared(),
            button(),
        ]);

    left.add_widget(Label::new("horizontal start").shared());
    left.add_widget(horizontal_test_widget().align(Alignment::Start).shared());

    left.add_widget(Label::new("horizontal center").shared());
    left.add_widget(horizontal_test_widget().align(Alignment::Center).shared());

    left.add_widget(Label::new("horizontal end").shared());
    left.add_widget(horizontal_test_widget().align(Alignment::End).shared());

    right.add_widget(Label::new("vertical start").shared());
    right.add_widget(vertical_test_widget().align(Alignment::Start).shared());

    right.add_widget(Label::new("vertical center").shared());
    right.add_widget(vertical_test_widget().align(Alignment::Center).shared());

    right.add_widget(Label::new("vertical end").shared());
    right.add_widget(vertical_test_widget().align(Alignment::End).shared());

    app.add_widget(
        WBox::horizontal().with(vec![
            left.shared(),
            right.shared(),
        ])
        .spacing(30)
        .shared()
    );

    app.run();
}
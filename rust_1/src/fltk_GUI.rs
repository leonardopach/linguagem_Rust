use fltk::{
    app, button,
    enums::{Color, Font, FrameType},
    prelude::{GroupExt, WidgetExt},
    window,
};

use fltk_theme::{widget_themes, ThemeType, WidgetTheme};
fn print_fltk() {
    let a = app::App::default();
    let theme = WidgetTheme::new(ThemeType::Aero);
    theme.apply();
    let mut win = window::Window::default()
        .with_size(400, 300)
        .with_label("My window");

    // win.set_color(Color::White);

    let mut btn = button::Button::default()
        .with_size(80, 80)
        .center_of_parent()
        .with_label("Click");

    btn.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
    // theme_button(&mut btn);
    win.end();
    win.show();
    btn.set_callback(move |b| {
        b.set_label("Clicked");
        win.set_label("clicked");
    });
    a.run().unwrap();
}

// fn theme_button(btn: &mut button::Button) {
//     btn.clear_visible_focus();
//     btn.set_color(Color::from_rgb(255, 0, 0).lighter());
//     btn.set_selection_color(Color::from_rgb(255, 0, 0).darker());
//     btn.set_frame(FrameType::RFlatBox)
// }

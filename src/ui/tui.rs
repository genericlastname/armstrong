use cursive::Cursive;
use cursive::view::{Nameable, Resizable};
use cursive::views::{Dialog, LinearLayout, TextView};

pub fn configure_callbacks(app: &mut Cursive) {
    app.add_global_callback('q', |s| s.quit());
}

pub fn configure_default_view(app: &mut Cursive) {
    app.add_layer(Dialog::text("This is a survey\nPress <Next> when ready.")
        .title("Important")
        .button("Next", show_next));
}

fn show_next(app: &mut Cursive) {
    app.pop_layer();
    app.add_layer(Dialog::text("The thing")
        .title("Question")
        .button("Yes!", |s| show_answer(s, "I knew it! Well done!"))
        .button("No!", |s| show_answer(s, "I knew you couldn't be trusted!"))
        .button("Uh?", |s| s.add_layer(Dialog::info("Try again!"))));
}

fn show_answer(s: &mut Cursive, msg: &str) {
    s.pop_layer();
    s.add_layer(Dialog::text(msg)
        .title("Results")
        .button("Finish", |s| s.quit()));
}

// Is triggered when config file is not found, obstensibly on a fresh install.


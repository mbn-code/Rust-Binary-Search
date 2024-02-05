extern crate druid;
use druid::widget::{Button, Flex, Label};
use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc};

fn main() -> Result<(), PlatformError> {
    // Create the main window description
    let main_window = WindowDesc::new(ui_builder());

    // Initialize your data
    let data = 0_u32;

    // Launch the app
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(data)
}

// Define the UI structure
fn ui_builder() -> impl Widget<u32> {
    // The label text will be computed dynamically based on the current locale and count
    let text =
        LocalizedString::new("hello-counter").with_arg("count", |data: &u32, _env| (*data).into());

    // Create a label widget
    let label = Label::new(text).padding(5.0).center();

    // Create a button widget
    let button = Button::new("increment")
        .on_click(|_ctx, data, _env| *data += 1)
        .padding(5.0);

    // Arrange the label and button in a vertical column
    Flex::column().with_child(label).with_child(button)
}

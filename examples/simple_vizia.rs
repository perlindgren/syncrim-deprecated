use vizia::prelude::*;

const STYLE: &str = r#"

    .modal {
        space: 1s;
        background-color: white;
        border-radius: 3px;
        border-width: 1px;
        border-color: #999999;
        outer-shadow: 0 3 10 #00000055;
        overflow: visible;
        child-space: 10px;
    }

    .modal>vstack>label {
        width: auto;
        height: auto;
        space: 5px;
        child-space: 1s;
    }

    .modal button {
        border-radius: 3px;
        child-space: 1s;
    }

    .modal hstack {
        child-space: 1s;
        col-between: 20px;
    }
"#;

#[derive(Lens)]
pub struct AppData {
    is_saved: bool,
    show_dialog: bool,
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            // Intercept WindowClose event to show a dialog if not 'saved'.
            WindowEvent::WindowClose => {
                if !self.is_saved {
                    self.show_dialog = true;
                    meta.consume();
                }
            }
            _ => {}
        });

        event.map(|app_event, _| match app_event {
            AppEvent::HideModal => {
                self.show_dialog = false;
            }

            AppEvent::Save => {
                println!("Save");

                let path = std::env::current_dir().unwrap();

                let res = rfd::FileDialog::new()
                    .add_filter("text", &["txt", "rs"])
                    .add_filter("rust", &["rs", "toml"])
                    .set_directory(&path)
                    .pick_files();

                println!("The user choose: {:#?}", res);
                self.is_saved = true;
            }

            AppEvent::SaveAndClose => {
                println!("Save and Close");

                self.is_saved = true;
                cx.emit(WindowEvent::WindowClose);
            }

            AppEvent::Cancel => {
                self.is_saved = false;
            }
        });
    }
}

pub enum AppEvent {
    HideModal,
    Save,
    SaveAndClose,
    Cancel,
}

fn main() {
    Application::new(|cx| {
        cx.add_theme(STYLE);
        AppData {
            is_saved: false,
            show_dialog: false,
        }
        .build(cx);

        HStack::new(cx, |cx| {
            Button::new(
                cx,
                |cx| cx.emit(WindowEvent::WindowClose),
                |cx| Label::new(cx, "Close"),
            );
            Button::new(
                cx,
                |cx| cx.emit(AppEvent::Save),
                |cx| Label::new(cx, "Save"),
            );
        })
        .col_between(Pixels(10.0))
        .space(Pixels(20.0));

        Popup::new(cx, AppData::show_dialog, true, |cx| {
            VStack::new(cx, |cx| {
                Label::new(cx, "Save before close?")
                    .width(Stretch(1.0))
                    .child_space(Stretch(1.0));
                HStack::new(cx, |cx| {
                    Button::new(
                        cx,
                        |cx| cx.emit(AppEvent::SaveAndClose),
                        |cx| Label::new(cx, "Save & Close"),
                    )
                    .width(Pixels(120.0))
                    .class("accent");

                    Button::new(
                        cx,
                        |cx| cx.emit(AppEvent::HideModal),
                        |cx| Label::new(cx, "Cancel"),
                    )
                    .width(Pixels(120.0));
                });
            })
            .row_between(Pixels(20.0))
            .height(Auto);
        })
        .on_blur(|cx| cx.emit(AppEvent::HideModal))
        .width(Pixels(300.0))
        .height(Auto)
        .row_between(Pixels(20.0))
        .class("modal");
    })
    .run();
}

use vizia::fonts::icons_names::{DOWN, MINUS, UP};
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

    modal>popup>label {
        width: auto;
        height: auto;
        space: 5px;
        child-space: 1s;
    }

    button {
        border-radius: 3px;
        child-space: 1s;
    }

    hstack {
        child-space: 1s;
        col-between: 20px;
    }
"#;

fn main() {
    Application::new(|cx| {
        cx.add_theme(STYLE);

        AppData { show_modal: false }.build(cx);

        Button::new(
            cx,
            |cx| cx.emit(AppEvent::ShowModal),
            |cx| Label::new(cx, "Show Modal"),
        )
        .width(Pixels(150.0))
        .position_type(PositionType::SelfDirected)
        .top(Pixels(100.0))
        .left(Pixels(100.0));

        Popup::new(cx, AppData::show_modal, true, |cx| {
            Label::new(cx, "This is a message").width(Stretch(1.0));
            HStack::new(cx, |cx| {
                Button::new(
                    cx,
                    |cx| cx.emit(AppEvent::HideModal),
                    |cx| Label::new(cx, "Ok"),
                )
                .width(Pixels(100.0))
                .class("accent");

                Button::new(
                    cx,
                    |cx| cx.emit(AppEvent::HideModal),
                    |cx| Label::new(cx, "Cancel"),
                )
                .width(Pixels(100.0));
            });
        })
        .on_blur(|cx| cx.emit(AppEvent::HideModal))
        .width(Pixels(300.0))
        .height(Auto)
        .row_between(Pixels(10.0))
        .class("modal");
    })
    .title("Modal")
    .run();
}

#[derive(Debug)]
pub enum AppEvent {
    ShowModal,
    HideModal,
}

#[derive(Lens)]
pub struct AppData {
    show_modal: bool,
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::ShowModal => {
                self.show_modal = true;
            }
            AppEvent::HideModal => {
                self.show_modal = false;
            }
        });
    }
}

use std::fmt::{self, Debug, Display};

#[derive(Lens, Clone)]
pub struct Port {}

impl<T> Display for Port
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

#[derive(Debug)]
pub enum PortEvent {
    SetData,
}

impl Model for Port {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        // event.map(|port_event, _| match app_event {
        //     AppEvent::ShowModal => {
        //         self.show_modal = true;
        //     }
        //     AppEvent::HideModal => {
        //         self.show_modal = false;
        //     }
        // });
    }
}

impl Port {
    pub fn new<'a, A, L>(
        cx: &'a mut Context,
        action: A,
        lens: L,
        name: &str,
        data: T,
    ) -> Handle<'a, Self>
    where
        A: 'static + Fn(&mut EventContext),
        L: Lens<Target = Port>,
    {
        // Self { data }.build(cx, |cx| {
        //     Label::new(cx, MINUS).class("icon");
        // })
        Self {}.build(cx, |cx| {})
    }
}

impl<T> View for Port {}

// #[derive(Lens, Clone, Debug)]
// pub struct Port<T>
// where
//     T: Display + Clone + 'static + Debug,
// {
//     data: T,
// }

// impl<T> Display for Port<T>
// where
//     T: Display + Clone + 'static + Debug,
// {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.data)
//     }
// }

// #[derive(Debug)]
// pub enum PortEvent<T>
// where
//     T: Display + Clone + 'static + Debug,
// {
//     SetData(T),
// }

// impl<T> Model for Port<T>
// where
//     T: Display + Clone + 'static + Debug,
// {
//     fn event(&mut self, _: &mut EventContext, event: &mut Event) {
//         // event.map(|port_event, _| match app_event {
//         //     AppEvent::ShowModal => {
//         //         self.show_modal = true;
//         //     }
//         //     AppEvent::HideModal => {
//         //         self.show_modal = false;
//         //     }
//         // });
//     }
// }

// impl<T> Port<T>
// where
//     T: Display + Clone + 'static + Debug,
// {
//     pub fn new<'a, A, L>(
//         cx: &'a mut Context,
//         action: A,
//         lens: L,
//         name: &str,
//         data: T,
//     ) -> Handle<'a, Self>
//     where
//         A: 'static + Fn(&mut EventContext),
//         L: Lens<Target = Port<T>>,
//     {
//         // Self { data }.build(cx, |cx| {
//         //     Label::new(cx, MINUS).class("icon");
//         // })
//     }
// }

// impl<T> View for Port<T> where T: Display + Clone + 'static + Debug {}

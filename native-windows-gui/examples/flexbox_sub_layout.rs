/*!
    A very simple application that show how to use a flexbox layout.

    Requires the following features: `cargo run --example flexbox --features "flexbox"`
*/

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;
use nwg::stretch::{
    geometry::Rect,
    geometry::Size,
    style::{Dimension as D, FlexDirection},
};
extern crate chrono;
// extern crate curl;
// use std::io::{stdout, Write};

// use curl::easy::Easy;
use chrono::Local;

const DATE_FORMAT_STR: &'static str = "%Y-%m-%d][%H:%M:%S";
use curl::easy::List;
use nwg::NativeUi;
use std::{cell::RefCell, rc::Rc};
use std::process::Command;
use stretch::style::*;

#[derive(Default)]
pub struct FlexBoxApp {
    flex_ui: Rc<flexbox_app_ui::FlexBoxAppUi>,
    window: nwg::Window,
    layout: nwg::FlexboxLayout,
    layout_1: nwg::GridLayout,

    layout_common: nwg::FlexboxLayout,
    layout_custom: nwg::FlexboxLayout,
    layout_history: nwg::FlexboxLayout,
    input_down_bandwidth: nwg::TextInput,
    input_down_jitter: nwg::TextInput,
    input_down_delay: nwg::TextInput,
    input_down_loss: nwg::TextInput,

    input_up_bandwidth: nwg::TextInput,
    input_up_jitter: nwg::TextInput,
    input_up_delay: nwg::TextInput,
    input_up_loss: nwg::TextInput,

    button_turn_on: nwg::Button,
    button_turn_off: nwg::Button,
    button_update: nwg::Button,

    button_add_common: nwg::Button,

    label_up: nwg::Label,
    label_down: nwg::Label,

    label_up_bandwidth: nwg::Label,
    label_up_jitter: nwg::Label,
    label_up_delay: nwg::Label,
    label_up_loss: nwg::Label,

    label_down_bandwidth: nwg::Label,
    label_down_jitter: nwg::Label,
    label_down_delay: nwg::Label,
    label_down_loss: nwg::Label,

    buttons: RefCell<Vec<nwg::Button>>,
    handlers: RefCell<Vec<nwg::EventHandler>>,
    settings: RefCell<Settings>,
}
use serde::{Deserialize, Serialize};
use std::io::{stdout, Read, Write};
#[derive(Serialize, Deserialize, Debug, Default)]
struct Delay {
    delay: u32,
    jitter: u32,
    correlation: u32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Loss {
    percentage: u32,
    correlation: u32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Reorder {
    percentage: u32,
    correlation: u32,
    gap: u32,
}
#[derive(Serialize, Deserialize, Debug, Default)]
struct Corruption {
    percentage: u32,
    correlation: u32,
}
#[derive(Serialize, Deserialize, Debug, Default)]
struct Network {
    rate: u32,
    delay: Delay,
    loss: Loss,
    reorder: Reorder,
    corruption: Corruption,
    iptables_options: Vec<u32>,
}
#[derive(Serialize, Deserialize, Debug, Default)]
struct Settings {
    down: Network,
    up: Network,
}
use std::rc::Weak;
impl FlexBoxApp {
    fn create_custom(data: &mut FlexBoxApp) -> Result<(), nwg::NwgError> {
        // Controls
        nwg::Window::builder()
            .size((1200, 800))
            .position((0, 0))
            .title("Flexbox example")
            .build(&mut data.window)?;

        nwg::Button::builder()
            .text("Turn on")
            .parent(&data.window)
            .focus(true)
            .build(&mut data.button_turn_on)?;

        nwg::Button::builder()
            .text("Turn off")
            .parent(&data.window)
            .focus(true)
            .build(&mut data.button_turn_off)?;

        nwg::Button::builder()
            .text("Add case")
            .parent(&data.window)
            .focus(true)
            .build(&mut data.button_add_common)?;
        // nwg::Button::builder()
        //     .text("Update shading")
        //     .parent(&data.window)
        //     .focus(true)
        //     .build(&mut data.button_update)?;
        nwg::Label::builder()
            .text("UP:")
            .h_align(nwg::HTextAlign::Left)
            .parent(&data.window)
            .build(&mut data.label_up)?;
        nwg::Label::builder()
            .text("bandwidth:")
            .h_align(nwg::HTextAlign::Center)
            .parent(&data.window)
            .build(&mut data.label_up_bandwidth)?;

        nwg::Label::builder()
            .text("jitter:")
            .h_align(nwg::HTextAlign::Center)
            .parent(&data.window)
            .build(&mut data.label_up_jitter)?;

        nwg::Label::builder()
            .text("delay:")
            .h_align(nwg::HTextAlign::Center)
            .parent(&data.window)
            .build(&mut data.label_up_delay)?;

        nwg::Label::builder()
            .text("loss:")
            .h_align(nwg::HTextAlign::Center)
            .parent(&data.window)
            .build(&mut data.label_up_loss)?;

        nwg::TextInput::builder()
            .text("0")
            .parent(&data.window)
            .build(&mut data.input_up_bandwidth)?;

        nwg::TextInput::builder()
            .text("0")
            .flags(nwg::TextInputFlags::VISIBLE | nwg::TextInputFlags::NUMBER)
            .parent(&data.window)
            .build(&mut data.input_up_jitter)?;

        nwg::TextInput::builder()
            .text("0")
            .parent(&data.window)
            .build(&mut data.input_up_delay)?;

        nwg::TextInput::builder()
            .text("0")
            .parent(&data.window)
            .build(&mut data.input_up_loss)?;
        nwg::Label::builder()
            .text("DOWN:")
            .h_align(nwg::HTextAlign::Left)
            .parent(&data.window)
            .build(&mut data.label_down)?;
        nwg::Label::builder()
            .text("bandwidth:")
            .h_align(nwg::HTextAlign::Center)
            .parent(&data.window)
            .build(&mut data.label_down_bandwidth)?;

        nwg::Label::builder()
            .text("jitter:")
            .h_align(nwg::HTextAlign::Center)
            .parent(&data.window)
            .build(&mut data.label_down_jitter)?;

        nwg::Label::builder()
            .text("delay:")
            .h_align(nwg::HTextAlign::Center)
            .parent(&data.window)
            .build(&mut data.label_down_delay)?;
        nwg::Label::builder()
            .text("loss:")
            .h_align(nwg::HTextAlign::Center)
            .parent(&data.window)
            .build(&mut data.label_down_loss)?;
        nwg::TextInput::builder()
            .text("0")
            .parent(&data.window)
            .build(&mut data.input_down_bandwidth)?;

        nwg::TextInput::builder()
            .text("0")
            .flags(nwg::TextInputFlags::VISIBLE | nwg::TextInputFlags::NUMBER)
            .parent(&data.window)
            .build(&mut data.input_down_jitter)?;

        nwg::TextInput::builder()
            .text("0")
            .parent(&data.window)
            .build(&mut data.input_down_delay)?;

        nwg::TextInput::builder()
            .text("0")
            .parent(&data.window)
            .build(&mut data.input_down_loss)?;
        nwg::GridLayout::builder()
            .parent(&data.window)
            .spacing(1)
            .max_size([400, 800])
            .min_size([100, 30])
            .child(1, 0, &data.label_up_bandwidth)
            .child(1, 1, &data.label_up_jitter)
            .child(1, 2, &data.label_up_delay)
            .child(1, 3, &data.label_up_loss)
            .child(2, 0, &data.input_up_bandwidth)
            .child(2, 1, &data.input_up_jitter)
            .child(2, 2, &data.input_up_delay)
            .child(2, 3, &data.input_up_loss)
            .child(0, 1, &data.label_up)
            .child(0, 5, &data.label_down)
            .child(1, 4, &data.label_down_bandwidth)
            .child(1, 5, &data.label_down_jitter)
            .child(1, 6, &data.label_down_delay)
            .child(1, 7, &data.label_down_loss)
            .child(2, 4, &data.input_down_bandwidth)
            .child(2, 5, &data.input_down_jitter)
            .child(2, 6, &data.input_down_delay)
            .child(2, 7, &data.input_down_loss)
            .child(2, 8, &data.button_turn_on)
            .child(1, 8, &data.button_turn_off)
            .child(0, 8, &data.button_add_common)
            .build(&mut data.layout_1)?;

        Ok(())
    }
    fn add_common(&self, settings: ShapeSetting) {
        let title: String = settings.up.bandwidth.unwrap().to_string()
            + " " + &settings.up.jitter.unwrap().to_string()
            + " " + &settings.up.delay.unwrap().to_string()
            + " " + &settings.up.loss.unwrap().to_string();
        let content = "world";

        let mut new_button = Default::default();
        nwg::Button::builder()
            .text(&title)
            .parent(&self.window)
            .build(&mut new_button)
            .expect("Failed to build button");

        let mut buttons = self.buttons.borrow_mut();
        let mut handlers = self.handlers.borrow_mut();

        let blen = buttons.len() as u32;
        let (x, y) = (blen % 6, blen / 6);
        let style = Style {
            size: Size {
                width: Dimension::Auto,
                height: Dimension::Points(100.0),
            },
            justify_content: JustifyContent::Center,
            ..Default::default()
        };
        self.layout_common.add_child(&new_button, style);
        // You can share controls handle with events handlers
        let new_button_handle = new_button.handle;
        let handler = nwg::bind_event_handler(
            &new_button.handle,
            &self.window.handle,
            move |evt, _evt_data, handle| match evt {
                nwg::Event::OnButtonClick => {
                    if handle == new_button_handle {
                        nwg::simple_message(&title, &content);
                        
                    }
                }
                _ => {}
            },
        );

        buttons.push(new_button);
        handlers.push(handler);
    }

    fn plot(date: String) {
        let cmd_str = "python E:\\workspace\\MindLinker\\MaxME\\plot_time.py C:\\Users\\1602\\AppData\\Roaming\\MaxME\\maxme.log".to_string();
        let cmd_str = cmd_str + " " + &date;
        println!("{}", cmd_str);
        let output = Command::new("cmd")
            .arg("/c")
            .arg(cmd_str)
            .output()
            .expect("cmd exec error!");
        let output_str = String::from_utf8_lossy(&output.stdout);
    }
    fn shape(&self, turn_on: bool) {
        {
            let mut settings = self.settings.borrow_mut();
            settings.down.rate = self.input_down_bandwidth.text().parse().unwrap();
            settings.down.delay = Delay {
                jitter: self.input_down_jitter.text().parse().unwrap(),
                delay: self.input_down_delay.text().parse().unwrap(),
                correlation: 0,
            };
            settings.down.loss = Loss {
                percentage: self.input_down_loss.text().parse().unwrap(),
                correlation: 0,
            };

            settings.up.rate = self.input_up_bandwidth.text().parse().unwrap();
            settings.up.delay = Delay {
                jitter: self.input_up_jitter.text().parse().unwrap(),
                delay: self.input_up_delay.text().parse().unwrap(),
                correlation: 0,
            };
            settings.up.loss = Loss {
                percentage: self.input_up_loss.text().parse().unwrap(),
                correlation: 0,
            };
        }
        let mut list = List::new();
        list.append("content-type: application/json").unwrap();
        let serialized = serde_json::to_string(&self.settings).unwrap();
        let mut data_body = serialized.as_bytes();
        let mut handle = Easy::new();
        let mut method = String::from("POST");
        if !turn_on {
            method = String::from("DELETE");
        }
        handle.url("http://192.168.7.1:8000/api/v1/shape/").unwrap();
        handle.custom_request(&method);
        // handle.post(true).unwrap();
        handle.post_field_size(data_body.len() as u64).unwrap();
        handle.post_fields_copy(data_body);
        handle.http_headers(list).unwrap();
        let mut data = Vec::new();
        {
            let mut transfer = handle.transfer();
            transfer
                .read_function(|into| Ok(data_body.read(into).unwrap()))
                .unwrap();
            transfer
                .write_function(|new_data| {
                    data.extend_from_slice(new_data);
                    Ok(new_data.len())
                })
                .unwrap();
            transfer.perform().unwrap();
        }

        println!("{:?}", String::from_utf8_lossy(&data));

        // let serialized = serde_json::to_string(&settings).unwrap();

        // println!("serialized = {}", serialized);
        // let deserialized: Settings = serde_json::from_str(&serialized).unwrap();
        // println!("deserialized = {:?}", deserialized);

        let date = Local::now();
        let format_date = date.format("%Y-%m-%d %H:%M:%S:%3f");

        let mut title = date.format("%H:%M:%S").to_string()
            + " u:"
            + &self.input_up_bandwidth.text()
            + " "
            + &self.input_up_jitter.text()
            + " "
            + &self.input_up_delay.text()
            + " "
            + &self.input_up_loss.text()
            + " d:"
            + &self.input_down_bandwidth.text()
            + " "
            + &self.input_down_jitter.text()
            + " "
            + &self.input_down_delay.text()
            + " "
            + &self.input_down_loss.text();
        if !turn_on {
            title = date.format("%H:%M:%S").to_string() + " off";
        }
        let mut new_button = Default::default();
        nwg::Button::builder()
            .text(&title)
            .parent(&self.window)
            .build(&mut new_button)
            .expect("Failed to build button");

        let mut buttons = self.buttons.borrow_mut();
        let mut handlers = self.handlers.borrow_mut();

        let blen = buttons.len() as u32;
        let (x, y) = (blen % 6, blen / 6);
        let style = Style {
            size: Size {
                width: Dimension::Auto,
                height: Dimension::Points(100.0),
            },
            justify_content: JustifyContent::Center,
            ..Default::default()
        };
        self.layout_history.add_child(&new_button, style);

        // You can share controls handle with events handlers
        let new_button_handle = new_button.handle;
        let handler = nwg::bind_event_handler(
            &new_button.handle,
            &self.window.handle,
            move |evt, _evt_data, handle| match evt {
                nwg::Event::OnButtonClick => {
                    if handle == new_button_handle {
                        // nwg::simple_message(&title, &content);
                        FlexBoxApp::plot(format_date.to_string());
                    }
                }
                _ => {}
            },
        );

        buttons.push(new_button);
        handlers.push(handler);
    }
    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

//
// ALL of this stuff is handled by native-windows-derive
//
mod flexbox_app_ui {
    use super::*;
    use native_windows_gui as nwg;
    use std::cell::RefCell;
    use std::ops::Deref;
    use std::rc::Rc;

    #[derive(Default)]
    pub struct FlexBoxAppUi {
        inner: Rc<FlexBoxApp>,
        default_handler: RefCell<Option<nwg::EventHandler>>,
    }

    impl nwg::NativeUi<Box<FlexBoxAppUi>> for FlexBoxApp {
        fn build_ui(mut data: FlexBoxApp) -> Result<Box<FlexBoxAppUi>, nwg::NwgError> {
            use nwg::Event as E;
            FlexBoxApp::create_custom(&mut data)?;
            // Wrap-up
            let ui = Box::new(FlexBoxAppUi {
                inner: Rc::new(data),
                default_handler: Default::default(),
            });
            // Events
            let evt_ui = Rc::downgrade(&ui.inner);
            let handle_events = move |evt, _evt_data, handle| {
                if let Some(evt_ui) = evt_ui.upgrade() {
                    match evt {
                        E::OnWindowClose => {
                            if &handle == &evt_ui.window {
                                FlexBoxApp::exit(&evt_ui);
                            }
                        }
                        E::OnButtonClick => {
                            if &handle == &evt_ui.button_turn_on {
                                FlexBoxApp::shape(&evt_ui, true);
                            } else if &handle == &evt_ui.button_add_common {
                                FlexBoxApp::add_common(&evt_ui, Default::default());
                            } else if &handle == &evt_ui.button_turn_off {
                                FlexBoxApp::shape(&evt_ui, false);
                            }
                        }
                        _ => {}
                    }
                }
            };

            *ui.default_handler.borrow_mut() = Some(nwg::full_bind_event_handler(
                &ui.window.handle,
                handle_events,
            ));

            // Layout

            // nwg::FlexboxLayout::builder()
            //     .parent(&ui.window)
            //     .flex_direction(FlexDirection::Column)
            //     .child(&ui.button_switch)
            //     .child_size(Size {
            //         width: D::Auto,
            //         height: D::Points(200.0),
            //     })
            //     .child(&ui.input_down_jitter)
            //     .child_flex_grow(2.0)
            //     .child_size(Size {
            //         width: D::Auto,
            //         height: D::Auto,
            //     })
            //     .child(&ui.input_down_delay)
            //     .child_flex_grow(2.0)
            //     .child_size(Size {
            //         width: D::Auto,
            //         height: D::Auto,
            //     })
            //     .build_partial(&ui.layout_custom)?;

            nwg::FlexboxLayout::builder()
                .parent(&ui.window)
                .flex_direction(FlexDirection::Column)
                // .child(&ui.button4)
                // .child_size(Size {
                //     width: D::Auto,
                //     height: D::Points(200.0),
                // })
                // .child(&ui.button5)
                // .child_flex_grow(2.0)
                // .child_size(Size {
                //     width: D::Auto,
                //     height: D::Auto,
                // })
                .build_partial(&ui.layout_common)?;

            nwg::FlexboxLayout::builder()
                .parent(&ui.window)
                .flex_direction(FlexDirection::Column)
                // .child(&ui.button6)
                // .child_flex_grow(2.0)
                // .child_size(Size {
                //     width: D::Auto,
                //     height: D::Auto,
                // })
                .build_partial(&ui.layout_history)?;
            nwg::FlexboxLayout::builder()
                .padding(Rect {
                    start: D::Points(400.0),
                    end: D::Auto,
                    top: D::Auto,
                    bottom: D::Auto,
                })
                .parent(&ui.window)
                .flex_direction(FlexDirection::Row)
                .child_layout(&ui.layout_common)
                .child_size(Size {
                    width: D::Points(400.0),
                    height: D::Auto,
                })
                // .child_layout(&ui.layout_custom)
                // .child_size(Size {
                //     width: D::Points(400.0),
                //     height: D::Auto,
                // })
                .child_layout(&ui.layout_history)
                .child_size(Size {
                    width: D::Points(500.0),
                    height: D::Auto,
                })
                .build(&ui.layout)?;
            let file_path = "config.toml";
            let mut file = match File::open(file_path) {
                Ok(f) => f,
                Err(e) => panic!("no such file {} exception:{}", file_path, e),
            };
            let mut str_val = String::new();
            match file.read_to_string(&mut str_val) {
                Ok(s) => s,
                Err(e) => panic!("Error Reading file: {}", e),
            };
            let config: Conf = toml::from_str(&str_val).unwrap();

            for x in config.settings.unwrap() {
                ui.add_common(x);
            }
            return Ok(ui);
        }
    }

    impl Drop for FlexBoxAppUi {
        /// To make sure that everything is freed without issues, the default handler must be unbound.
        fn drop(&mut self) {
            let handler = self.default_handler.borrow();
            if handler.is_some() {
                nwg::unbind_event_handler(handler.as_ref().unwrap());
            }
        }
    }

    impl Deref for FlexBoxAppUi {
        type Target = FlexBoxApp;

        fn deref(&self) -> &FlexBoxApp {
            &self.inner
        }
    }
}
#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize, Debug)]
struct NetworkConfig {
    bandwidth: Option<u32>,
    jitter: Option<u32>,
    delay: Option<u32>,
    loss: Option<u32>,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        NetworkConfig {
            bandwidth: Some(0),
            jitter: Some(0),
            delay: Some(0),
            loss: Some(0),
        }
    }
}

#[derive(Deserialize, Debug, Default)]
struct ShapeSetting {
    up: NetworkConfig,
    down: NetworkConfig,
}

#[derive(Deserialize, Debug)]
struct Conf {
    settings: Option<Vec<ShapeSetting>>,
}
use curl::easy::Easy;
fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _ui = FlexBoxApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}

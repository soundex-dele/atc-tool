/*!
    A very simple application that show your name in a message box.
    See `basic_d` for the derive version
*/

extern crate native_windows_gui as nwg;
use nwg::NativeUi;
use std::process::Command;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct BasicApp {
    window: nwg::Window,
    name_edit: nwg::TextInput,
    copy_dll_button: nwg::Button,
    install_apk: nwg::Button,
    pull_log: nwg::Button,
}

impl BasicApp {
    fn copy_dll(&self) {
        let cmd_str = "copy /Y E:\\workspace\\MindLinker\\sdk\\MaxME\\build\\win32\\bin\\Release\\MaxMediaEngine.dll  C:\\Program Files (x86)\\MindLinker\\MindLinker\\MindLinker_5.11.0.9865\\resources\\extraResources\\MLMeeting".to_string();
        let output = Command::new("cmd")
            .arg("/c")
            .arg(cmd_str)
            .output()
            .expect("cmd exec error!");
        let output_str = String::from_utf8_lossy(&output.stdout);

        nwg::modal_info_message(&self.window, "Result", &format!("{}", output_str));
    }

    fn install_apk(&self) {
        let output_str = "install apk";
        nwg::modal_info_message(&self.window, "Result", &format!("{}", output_str));
    }

    fn pull_log(&self) {
        let output_str = "pull log";
        nwg::modal_info_message(&self.window, "Result", &format!("{}", output_str));
    }

    fn say_goodbye(&self) {
        nwg::modal_info_message(
            &self.window,
            "Goodbye",
            &format!("Goodbye {}", self.name_edit.text()),
        );
        nwg::stop_thread_dispatch();
    }
}

//
// ALL of this stuff is handled by native-windows-derive
//
mod basic_app_ui {
    use super::*;
    use native_windows_gui as nwg;
    use nwg::ControlHandle;
    use std::cell::RefCell;
    use std::ops::Deref;
    use std::rc::Rc;
    pub struct AppState {
        pub last_handle: ControlHandle,
    }

    impl AppState {
        fn new() -> Self {
            Self {
                last_handle: ControlHandle::NoHandle,
            }
        }
    }
    pub struct BasicAppUi {
        inner: Rc<BasicApp>,
        state: Arc<Mutex<AppState>>,
        default_handler: RefCell<Option<nwg::EventHandler>>,
    }

    impl nwg::NativeUi<BasicAppUi> for BasicApp {
        fn build_ui(mut data: BasicApp) -> Result<BasicAppUi, nwg::NwgError> {
            use nwg::Event as E;

            // Controls
            nwg::Window::builder()
                .flags(nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE)
                .size((800, 600))
                .position((300, 300))
                .title("ToolBox")
                .build(&mut data.window)?;

            nwg::TextInput::builder()
                .size((280, 35))
                .position((10, 10))
                .text("Written by Rust")
                .parent(&data.window)
                .focus(true)
                .build(&mut data.name_edit)?;

            nwg::Button::builder()
                .size((70, 70))
                .position((10, 50))
                .text("copy dll")
                .parent(&data.window)
                .build(&mut data.copy_dll_button)?;

            nwg::Button::builder()
                .size((80, 70))
                .position((90, 50))
                .text("install apk")
                .parent(&data.window)
                .build(&mut data.install_apk)?;

            nwg::Button::builder()
                .size((80, 70))
                .position((180, 50))
                .text("pull log")
                .parent(&data.window)
                .build(&mut data.pull_log)?;
            // Wrap-up
            let ui = BasicAppUi {
                inner: Rc::new(data),
                state: Arc::new(Mutex::new(AppState::new())),
                default_handler: Default::default(),
            };

            // Events
            let evt_ui = Rc::downgrade(&ui.inner);
            let app_state = Arc::clone(&ui.state);
            let handle_events = move |evt, _evt_data, handle| {
                if let Some(ui) = evt_ui.upgrade() {
                    match evt {
                        E::OnButtonClick => {
                            if &handle == &ui.copy_dll_button {
                                BasicApp::copy_dll(&ui);
                            } else if &handle == &ui.install_apk {
                                BasicApp::install_apk(&ui);
                            } else if &handle == &ui.pull_log {
                                BasicApp::pull_log(&ui);
                            }
                        }
                        E::OnWindowClose => {
                            if &handle == &ui.window {
                                BasicApp::say_goodbye(&ui);
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

            return Ok(ui);
        }
    }

    impl Drop for BasicAppUi {
        /// To make sure that everything is freed without issues, the default handler must be unbound.
        fn drop(&mut self) {
            let handler = self.default_handler.borrow();
            if handler.is_some() {
                nwg::unbind_event_handler(handler.as_ref().unwrap());
            }
        }
    }

    impl Deref for BasicAppUi {
        type Target = BasicApp;

        fn deref(&self) -> &BasicApp {
            &self.inner
        }
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _ui = BasicApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}

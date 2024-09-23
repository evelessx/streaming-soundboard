/// Application for reading, displaying, and responding to chats from various sources.
extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwd::NwgUi;
use nwg::NativeUi;

#[derive(Default, NwgUi)]
pub struct SoundbotApp {
    #[nwg_control(size: (300, 115), position: (300, 300), title: "Soundbot", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [nwg::stop_thread_dispatch()] )]
    window: nwg::Window,

    #[nwg_control(text: "Type message here!", size: (280, 25), position: (10, 10))]
    message_edit: nwg::TextInput,

    #[nwg_control(text: "Send chat (test)", size: (280, 60), position: (10, 40))]
    #[nwg_events( OnButtonClick: [SoundbotApp::say_hello] )]
    hello_button: nwg::Button,
}

impl SoundbotApp {
    fn say_hello(&self) {
        nwg::simple_message(
            "Welcome to my soundbot",
            &format!(
                "Pretend this came from twitch chat: {}",
                self.message_edit.text()
            ),
        );
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let _app = SoundbotApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}

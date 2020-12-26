

#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;

use super::*;
use butterscotch::{EventSystem, WindowEvent, WindowSettings, interop::EventSystem as EventSystemTrait, open_window};
use std::rc::Rc;

pub enum GameEvent {
    
}

pub fn engine_entry() {
    let event_system = Rc::new(EventSystem::<GameEvent>::new());
    let mut engine   = Engine::new(event_system.clone());

    open_window(get_base_config(), move |event| {
         
        match event {
            WindowEvent::Open(controller)   => { engine.open(controller);   },
            WindowEvent::Update(controller) => { event_system.process(&mut |_, _|{}); engine.update(controller); },
            WindowEvent::Redraw(controller) => { engine.render(controller); },
            WindowEvent::Resize(controller) => { engine.resize(controller); },
            WindowEvent::Close(controller)  => { engine.close(controller);  },
            WindowEvent::Title(controller)  => { engine.update_title(controller); },
            WindowEvent::Quit(controller)   => { engine.quit(controller); },
            WindowEvent::Cleanup(_controller) => { /*engine.lateUpdate();*/ }
        }
    });
}

cfg_if::cfg_if!{if #[cfg(target_arch = "wasm32")] {
    fn get_base_config() -> WindowSettings {
        WindowSettings::default().with_canvas({
            use web_sys::window;
            use wasm_bindgen::JsCast;
            window().unwrap().document().unwrap().get_element_by_id("game_canvas").unwrap().dyn_into::<HtmlCanvasElement>().unwrap()
        })
    }
} else {
    fn get_base_config() -> WindowSettings {
        WindowSettings::default()
    }
}}

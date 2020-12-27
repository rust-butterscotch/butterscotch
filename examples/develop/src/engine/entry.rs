/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use std::{cell::Cell, rc::Rc};

#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;

use butterscotch::{event::EventSystem, render::Renderer, window::{
        WindowEvent, 
        WindowSettings, 
        open_window
    }};

use super::Engine;

pub enum GameEvent {
    RendererCreated(Cell<Option<Renderer>>),
}

pub fn engine_entry() {
    let mut engine = Engine::new();
    open_window(get_base_config(), move |event| { match event {
        WindowEvent::Open(controller)     => { engine.open(controller);   },
        WindowEvent::Update(controller)   => { engine.update(controller); },
        WindowEvent::Redraw(controller)   => { engine.render(controller); },
        WindowEvent::Resize(controller)   => { engine.resize(controller); },
        WindowEvent::Close(controller)    => { engine.close(controller);  },
        WindowEvent::Title(controller)    => { engine.update_title(controller); },
        WindowEvent::Quit(controller)     => { engine.quit(controller); },
        WindowEvent::Cleanup(_controller) => { /*engine.lateUpdate();*/ }
    }});
}

cfg_if::cfg_if!{if #[cfg(target_arch = "wasm32")] {
    fn get_base_config() -> WindowSettings {
        WindowSettings::default()
            .with_canvas({
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

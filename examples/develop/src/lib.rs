#[cfg(target_arch = "wasm32")]
mod engine;

#[cfg(target_arch = "wasm32")]
use butterscotch::{EventSystem, interop::WindowEvent, WindowSettings, run_event_loop};

#[cfg(target_arch = "wasm32")]
use engine::*;

#[cfg(target_arch = "wasm32")]
enum GameEvent {
    Window(WindowEvent)
}

#[cfg(target_arch = "wasm32")]
impl From<WindowEvent> for GameEvent {
    #[inline(always)] fn from(e: WindowEvent) -> Self { GameEvent::Window(e) }
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main() {
    let event_system = EventSystem::<GameEvent>::new();
    let mut engine   = Engine::new();

    run_event_loop(WindowSettings::default().with_canvas({
        use web_sys::window;
        use wasm_bindgen::JsCast;
        window().unwrap().document().unwrap().get_element_by_id("game_canvas").unwrap().dyn_into::<HtmlCanvasElement>().unwrap()
    }), event_system, move |_, event| {
        match event {
            GameEvent::Window(WindowEvent::Init(controller)) => {
                engine.init(controller.as_ref());
            },
            GameEvent::Window(WindowEvent::Update(controller)) => {
                engine.update(controller.as_ref());
            },
            GameEvent::Window(WindowEvent::Redraw(controller)) => {
                engine.render(controller.as_ref());
            },
            GameEvent::Window(WindowEvent::Close(controller)) => {
                engine.close(controller.as_ref());
            },
            GameEvent::Window(WindowEvent::Quit) => {
                engine.quit();
            },
            GameEvent::Window(WindowEvent::TitleSync(controller)) => {
                engine.update_title(controller.as_ref());
            }
        }
    });
}
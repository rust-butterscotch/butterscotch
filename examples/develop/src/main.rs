mod engine;

use butterscotch::{EventSystem, interop::WindowEvent, WindowSettings, run_event_loop};
use engine::*;

enum GameEvent {
    Window(WindowEvent)
}

impl From<WindowEvent> for GameEvent {
    #[inline(always)] fn from(e: WindowEvent) -> Self { GameEvent::Window(e) }
}

fn main() {
    let event_system = EventSystem::<GameEvent>::new();
    let mut engine   = Engine::new();

    run_event_loop(WindowSettings::default(), event_system, move |_, event| {
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
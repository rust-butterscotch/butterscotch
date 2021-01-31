/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use std::{cell::Cell, rc::Rc};

#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;

use butterscotch::{chrono::{
        Accumulator, 
        TimerSmooth
    }, dpi::PixelsRaw, event::{Event, EventSimple, EventSystem}, future::{
        IncrementalLocalExecutor
    }, interop::WindowHandle, render::{
        Renderer,
        create_debug_pipeline
    }, util::{UnionRef, TinyString}, window::{
        WindowEvent, 
        WindowSettings, 
        open_window
    }};

const SAMPLE_WINDOW: usize = 10;

#[allow(dead_code)]
pub enum GameEvent {
    RendererCreated(Cell<Option<Renderer>>),
    Update(f64),
    Redraw(f64),
}

type DeltaTimeEvent = EventSimple<f64>;

const UPDATE_EVENT_ID: TinyString = TinyString::from_str("EngineUpdate");
const REDRAW_EVENT_ID: TinyString = TinyString::from_str("EngineRedraw");

//static event_system:   EventSystem<GameEvent>                      = Default::default();
//static event_executor: IncrementalLocalExecutor<Option<GameEvent>> = Default::default();

pub fn engine_entry() {
    let mut accum_update  = Accumulator::new(1.0/60.0, 10);
    let mut request_close = false;
    let event_system      = EventSystem::default();
    let event_executor    = IncrementalLocalExecutor::<Option<GameEvent>>::default();
    let mut renderer      = None;
    let mut pipeline      = None;
    let mut timer_update  = TimerSmooth::<{SAMPLE_WINDOW}>::default();
    let mut timer_frame   = TimerSmooth::<{SAMPLE_WINDOW}>::default();

    let update_event = Rc::new(DeltaTimeEvent::new(&UPDATE_EVENT_ID, accum_update.dt_fixed()));
    let redraw_event = Rc::new(DeltaTimeEvent::new(&REDRAW_EVENT_ID, 0.0)));

    open_window(get_base_config(), move |event| { match event {
        WindowEvent::Open(controller) => { 
            event_executor.exec(create_renderer(controller.get_window_handle(), controller.get_size_raw()));
        },
        WindowEvent::Update(controller)   => { 
            accum_update.accumulate();

            let mut should_render = true;
            if accum_update.has_accumulated() {

                event_executor.proccess(&mut |v| match v {
                    Some(v) => event_system.broadcast(v),
                    None    => {}
                });

                event_system.broadcast(update_event);

                event_system.process(&mut |_, event|{
                    match event.id() {
                        GameEvent::RendererCreated(event) => renderer = event.replace(None),
                        GameEvent::Update(dt) => {
                            event_executor.exec((async ||{
                                return None;
                            })());
                            event_system.broadcast(GameEvent::Update(*dt));
                        },
                        _ => {}
                    }
                    //self.notify(event);
                });

                timer_update.end_start();
                accum_update.consume();
                should_render = !accum_update.has_accumulated();
            }

            if should_render {
                controller.request_redraw();
                //self.frame_update();
            }

            if request_close {
            // TODO check if engine is allowed to close
            //     self.request_close = false;
            //     window.prevent_close();
            }
        },
        WindowEvent::Redraw(controller)   => { 
            timer_frame.end_start();
            let renderer = match renderer.as_mut() { Some(v) => v, _ => { println!("No renderer!"); return;} };
    
            if pipeline.is_none() {
                pipeline = Some(create_debug_pipeline(&renderer));
            }
            
            renderer.render(controller.get_size_raw(), pipeline.as_ref().unwrap());
        },
        WindowEvent::Resize(controller)   => { 
            let renderer = match renderer.as_mut() { Some(v) => v, _ => { println!("No renderer!"); return;} };
            renderer.resize(controller.get_size_raw());
        },
        WindowEvent::Close(_controller) => {
            request_close = true;
        },
        WindowEvent::Title(controller)    => { 
            controller.set_title(&format!(
                "fps: {}, tps: {}",
                timer_frame.tps_average().round(),
                timer_update.tps_average().round()
            ));
        },
        WindowEvent::Quit(_controller)     => { },
        WindowEvent::Cleanup(_controller) => { }
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


async fn create_renderer(window_handle: WindowHandle, window_size: PixelsRaw) -> Option<GameEvent> {
    return Some(
        GameEvent::RendererCreated(
            Cell::new(Some(Renderer::new(window_handle, window_size).await))
        )
    );
}

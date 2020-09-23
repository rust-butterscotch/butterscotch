/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use std::fmt;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder
};

use crate::engine::Engine;

pub fn butterscotch_run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut engine = Engine::new();
    engine.init();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {event: WindowEvent::CloseRequested, ..} => {
                engine.request_close();
            },
            Event::MainEventsCleared => {
                if engine.update() {
                    println!(
                        "fps: {}, tps: {}",
                        engine.frame_timer().tps_average().round().into_i64(1),
                        engine.update_timer().tps_average().round().into_i64(1)
                    );
                }

                if engine.should_redraw() {
                    window.request_redraw();
                }
            },
            Event::RedrawRequested(_) => {
                engine.render();
            },
            _ => ()
        }

        if engine.should_close() {
            *control_flow = ControlFlow::Exit
        }


        /*window.set_title(&format!(
            "Frame: {}ms, Ticks: {}ms",
            engine.time_frame().into_i64(1000),
            engine.time_update().into_i64(1000)
        ));*/
    });

}








































//
use crate::{Display, State};
use anyhow::Result;
use std::time::Instant;
use winit::dpi::LogicalSize;
use winit::event::*;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

pub struct Runtime;

impl Runtime {
    pub fn Execute<T: State>() -> Result<()> {
        env_logger::init();

        let event_loop = EventLoop::new();

        let window = WindowBuilder::new()
            .with_title("Alnilam")
            .with_inner_size(LogicalSize::new(1280, 720))
            .build(&event_loop)?;

        let mut display = pollster::block_on(Display::New(window))?;

        let mut app = T::Init(&display)?;

        let mut last_update = Instant::now();

        let mut is_resumed = true;
        let mut is_focused = true;
        let mut is_redraw_requested = true;

        event_loop.run(move |event, _, control_flow| {
            *control_flow = if is_resumed && is_focused {
                ControlFlow::Poll
            } else {
                ControlFlow::Wait
            };

            match event {
                Event::Resumed => is_resumed = true,
                Event::Suspended => is_resumed = false,
                Event::RedrawRequested(window_id) => {
                    if window_id == display.window.id() {
                        let now = Instant::now();
                        let delta = now - last_update;
                        last_update = now;

                        app.Update(&display, delta);

                        match app.Draw(&mut display) {
                            Ok(_) => {}
                            // Reconfigure the surface if lost
                            Err(wgpu::SurfaceError::Lost) => {
                                let size = display.window.inner_size();
                                display.Resize(size.width, size.height);
                                app.Resize(&display);
                            }
                            // The system is out of memory, we should probably quit
                            Err(wgpu::SurfaceError::OutOfMemory) => {
                                *control_flow = ControlFlow::Exit
                            }
                            // All other errors (Outdated, Timeout) should be resolved by the next frame
                            Err(e) => eprintln!("{:?}", e),
                        }

                        is_redraw_requested = false;
                    }
                }
                Event::MainEventsCleared => {
                    if is_focused && is_resumed && !is_redraw_requested {
                        display.window.request_redraw();
                        is_redraw_requested = true;
                    } else {
                        // Freeze time while the app is not in the foreground
                        last_update = Instant::now();
                    }
                }
                Event::WindowEvent { event, window_id } if window_id == display.window.id() => {
                    if !app.Input(&display, &event) {
                        match event {
                            WindowEvent::CloseRequested
                            | WindowEvent::KeyboardInput {
                                input:
                                    KeyboardInput {
                                        state: ElementState::Pressed,
                                        virtual_keycode: Some(VirtualKeyCode::Escape),
                                        ..
                                    },
                                ..
                            } => *control_flow = ControlFlow::Exit,
                            WindowEvent::Focused(focused) => is_focused = focused,
                            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                display.Resize(new_inner_size.width, new_inner_size.height);
                                app.Resize(&display);
                            }
                            WindowEvent::Resized(physical_size) => {
                                display.Resize(physical_size.width, physical_size.height);
                                app.Resize(&display);
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        })
    }
}

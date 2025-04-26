use env_logger;
use log::info;
use std::sync::Arc;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop}, // ActiveEventLoop might be unused with .run()
    window::Window,
};
use softbuffer::{Context, Surface};

fn main() {
    env_logger::init(); // Initialize logger

    // 1. Define event_loop
    let event_loop = EventLoop::new().expect("Failed to create event loop");

    // 2. Define window and surface as Option types before the loop
    let mut window: Option<Arc<Window>> = None;
    let mut surface: Option<Surface<Arc<Window>, Arc<Window>>> = None;

    // 3. Now run the event loop (Note: .run() is deprecated)
    event_loop
        .run(move |event, event_loop_window_target| {
            event_loop_window_target.set_control_flow(ControlFlow::Poll);

            match event {
                Event::NewEvents(_) => {
                    // Use window and surface (check if None)
                    if window.is_none() {
                        info!("Creating window and surface");
                        let window_attributes = Window::default_attributes()
                            .with_title("Softbuffer Example")
                            .with_inner_size(winit::dpi::LogicalSize::new(800, 600));
                        let new_window = Arc::new(
                            event_loop_window_target
                                .create_window(window_attributes)
                                .expect("Failed to create window"),
                        );

                        let context = Context::new(new_window.clone())
                            .expect("Failed to create softbuffer context");
                        // Pass context by reference (&)
                        let new_surface = Surface::new(&context, new_window.clone())
                            .expect("Failed to create surface");

                        // Assign to window and surface
                        window = Some(new_window);
                        surface = Some(new_surface);
                    }
                }
                // ... other event handlers using window and surface ...
                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    window_id,
                } => {
                    // Check window and surface exist before using
                    if let (Some(win), Some(surf)) = (window.as_ref(), surface.as_mut()) {
                        if window_id == win.id() {
                            // ... (redraw logic using win and surf) ...
                            info!("RedrawRequested");
                            let (width, height) = {
                                let size = win.inner_size();
                                (size.width, size.height)
                            };

                            if let (Some(non_zero_width), Some(non_zero_height)) = (
                                std::num::NonZeroU32::new(width),
                                std::num::NonZeroU32::new(height),
                            ) {
                                surf.resize(non_zero_width, non_zero_height)
                                    .expect("Failed to resize surface");

                                let mut buffer = surf.buffer_mut().expect("Failed to get buffer");
                                for index in 0..(width * height) as usize {
                                    buffer[index] = 0xFF00_8000; // Opaque dark green (ARGB)
                                }
                                buffer.present().expect("Failed to present buffer");
                            } else {
                                info!("Skipping redraw for zero size window");
                            }
                        }
                    }
                }
                 // ... other event handlers like Resized, CloseRequested, AboutToWait ...
                 Event::WindowEvent {
                    event: WindowEvent::Resized(new_size),
                    window_id,
                } => {
                     if let Some(win) = window.as_ref() {
                         if window_id == win.id() {
                            info!("Window resized to: {:?}", new_size);
                            win.request_redraw();
                         }
                     }
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } => {
                    if let Some(win) = window.as_ref() {
                        if window_id == win.id() {
                            info!("Close requested");
                            event_loop_window_target.exit();
                        }
                    }
                }
                Event::LoopExiting => {
                    info!("Exiting event loop");
                }
                Event::AboutToWait => {
                    if let Some(win) = window.as_ref() {
                         win.request_redraw();
                    }
                }
                _ => {}
            }
        })
        .expect("Event loop failed");
}
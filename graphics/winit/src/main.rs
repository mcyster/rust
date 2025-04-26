use env_logger;
use log::info;
use std::{num::NonZeroU32, sync::Arc};
use winit::{
    application::ApplicationHandler, // Import the trait
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId}, // Import WindowId
};
use softbuffer::{Context, Surface};

// Struct to hold application state
struct ApplicationState {
    window: Option<Arc<Window>>,
    context: Option<Context<Arc<Window>>>, // Store context for potential recreation
    surface: Option<Surface<Arc<Window>, Arc<Window>>>,
}

impl ApplicationState {
    // Helper to redraw the window content
    fn redraw(&mut self) {
        if let (Some(window), Some(surface)) = (self.window.as_ref(), self.surface.as_mut()) {
            let (width, height) = {
                let size = window.inner_size();
                (size.width, size.height)
            };

            if let (Some(non_zero_width), Some(non_zero_height)) = (
                NonZeroU32::new(width),
                NonZeroU32::new(height),
            ) {
                surface
                    .resize(non_zero_width, non_zero_height)
                    .expect("Failed to resize surface");

                let mut buffer = surface.buffer_mut().expect("Failed to get buffer");
                for index in 0..(width * height) as usize {
                    buffer[index] = 0xFF00_8000; // Opaque dark green (ARGB)
                }
                buffer.present().expect("Failed to present buffer");
                info!("Redraw complete");
            } else {
                info!("Skipping redraw for zero size window");
            }
        }
    }
}

// Implement the ApplicationHandler trait for our state struct
impl ApplicationHandler for ApplicationState {
    // This method runs once when the application resumes (often at startup)
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            info!("Resumed: Creating window and surface");
            let window_attributes = Window::default_attributes()
                .with_title("Softbuffer Example (run_app)")
                .with_inner_size(winit::dpi::LogicalSize::new(800, 600));
            let window = Arc::new(
                event_loop
                    .create_window(window_attributes)
                    .expect("Failed to create window"),
            );

            let context = Context::new(window.clone()).expect("Failed to create softbuffer context");
            let surface =
                Surface::new(&context, window.clone()).expect("Failed to create surface");

            self.window = Some(window);
            self.context = Some(context);
            self.surface = Some(surface);
        } else {
            info!("Resumed: Window already exists");
        }
        // Ensure the window is drawn at least once after resuming/creation
        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
        }
    }

    // Handles events specific to a window
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        target_window_id: WindowId,
        event: WindowEvent,
    ) {
        // Make sure the event is for our window
        if let Some(window) = self.window.as_ref() {
            if window.id() != target_window_id {
                return; // Event not for our window
            }
        } else {
            return; // No window exists yet
        }

        match event {
            WindowEvent::CloseRequested => {
                info!("Close requested");
                event_loop.exit();
            }
            WindowEvent::Resized(new_size) => {
                info!("Window resized to: {:?}", new_size);
                // Redraw will handle surface resize
                if let Some(window) = self.window.as_ref() {
                    window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                info!("RedrawRequested event received");
                self.redraw(); // Call our redraw helper
            }
            _ => {}
        }
    }

    // Called when the event loop is about to block and wait for events
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        // Can be used for continuous rendering by always requesting redraw
        // if let Some(window) = self.window.as_ref() {
        //     window.request_redraw();
        // }
    }

    // Called just before the application exits
    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        info!("Exiting event loop");
    }
}

fn main() {
    env_logger::init(); // Initialize logger

    let event_loop = EventLoop::new().expect("Failed to create event loop");
    // Set the control flow behavior (optional, Poll is default for run_app)
    event_loop.set_control_flow(ControlFlow::Poll);

    // Initialize application state
    let mut app_state = ApplicationState {
        window: None,
        context: None,
        surface: None,
    };

    // Run the application using run_app
    event_loop
        .run_app(&mut app_state) // Pass mutable reference to state
        .expect("Event loop failed");
}
// examples/async-wrapper-runtime/src/main.rs
use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;
use tao::event::{Event, StartCause, UserEvent};
use tao::event_loop::{ControlFlow, EventLoop};
use wry::application::window::WindowBuilder;

// Define our custom events: setter and getter
enum AppEvent {
    SetWindowTitle(String),
    GetWindowTitle(oneshot::Sender<String>),
}

fn main() -> wry::Result<()> {
    // Shared state: the window title
    let title_state = Arc::new(Mutex::new(String::from("Initial Title")));

    // Create an EventLoop that carries AppEvent
    let event_loop: EventLoop<AppEvent> = EventLoop::with_user_event();

    // Build a window (unused UI, just required by Wry)
    let window = WindowBuilder::new()
        .with_title("Async Wrapper Example")
        .build(&event_loop)?;

    // Create a proxy to send AppEvent into the loop
    let proxy = event_loop.create_proxy();

    // Initialize a Tokio runtime
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Failed to start Tokio runtime");

    // Clone for the async task
    let proxy_task = proxy.clone();
    let title_state_task = title_state.clone();

    // Spawn an async task to demonstrate setter/getter
    runtime.spawn(async move {
        // 1. Set the window title asynchronously
        proxy_task.send_event(AppEvent::SetWindowTitle("Hello from Tokio!".into()))
            .expect("Failed to send SetWindowTitle event");

        // 2. Wait a bit (simulate work)
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        // 3. Get the window title asynchronously
        let (resp_tx, resp_rx) = oneshot::channel();
        proxy_task.send_event(AppEvent::GetWindowTitle(resp_tx))
            .expect("Failed to send GetWindowTitle event");

        // Await the response
        match resp_rx.await {
            Ok(title) => println!("Async Getter received title: {}", title),
            Err(e) => eprintln!("Failed to receive title: {}", e),
        }
    });

    // Run the Tao event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                println!("Event loop initialized");
            }
            Event::UserEvent(app_event) => {
                match app_event {
                    AppEvent::SetWindowTitle(new_title) => {
                        // Update shared state
                        *title_state.lock().unwrap() = new_title.clone();
                        window.set_title(&new_title);
                        println!("Title set to: {}", new_title);
                    }
                    AppEvent::GetWindowTitle(responder) => {
                        // Read shared state and respond
                        let current = title_state.lock().unwrap().clone();
                        let _ = responder.send(current);
                    }
                }
            }
            Event::WindowEvent { event, .. } => {
                // Optional: handle close
                if let tao::event::WindowEvent::CloseRequested = event {
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => ()
        }
    });
}

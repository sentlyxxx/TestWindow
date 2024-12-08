use winit::{
	event::{Event, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	window::WindowBuilder,
};

pub struct WindowManager {
	event_loop: EventLoop<()>,
	window: winit::window::Window,
}

impl WindowManager {
	pub fn new(title: &str) -> Result<Self, Box<dyn std::error::Error>> {
		let event_loop = EventLoop::new();
		let window = WindowBuilder::new()
				.with_title(title)
				.build(&event_loop)?;

		Ok(Self {event_loop, window})
	}

	pub fn run<F>(self, mut event_handler: F)
	where
			F: 'static + FnMut(Event<()>),
		{
			let event_loop = self.event_loop;
			event_loop.run(move |event, _, control_flow| {
				*control_flow = ControlFlow::Poll;

				match &event {
					Event::WindowEvent { event, .. } => match event {
						WindowEvent::CloseRequested => {
							*control_flow = ControlFlow::Exit;
						}

						_ => {}
					},

					_ => {}
				}

				event_handler(event);
			});
		}
}
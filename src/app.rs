use crate::window::WindowManager;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
	let window_manager = WindowManager::new("Test")?;
	window_manager.run(|event| {
			println!("{:?}", event);
	});

	Ok(())
}
use pixels::{Pixels, SurfaceTexture};
use winit::window::Window;

pub struct Renderer {
	pixels: Pixels,
}

impl Renderer {
	pub fn new(window: &Window) -> Result<Self, Box<dyn std::error::Error>> {
		let size = window.inner_size();
		let surface_texture = SurfaceTexture::new(size.width, size.height, window);
		let pixels = Pixels::new(size.width, size.height, surface_texture)?;

		Ok(Self { pixels })
	}

	pub fn render(&mut self) -> Result<(), Box<dyn std::error::Error>> {
		self.pixels.render()?;
		Ok(())
	}
}
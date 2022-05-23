mod input;
mod painter;
pub use input::Input;
use painter::Painter;
use std::{cell::RefCell, ops::Deref, rc::Rc};
use ggez::graphics::{self};

pub use egui;

/// Contains a copy of [`egui::Context`] and a mutable reference for the paint_jobs vector from [`Painter`].
///
/// When is droped automatically will call [`egui::Context::end_frame`] function and update the paint_jobs
pub struct EguiContext {
	context  : egui::Context,
	painter  : Rc<RefCell<Painter>>,
	clipboard: Rc<RefCell<String >>,
}

impl Deref for EguiContext {
	type Target = egui::Context;
	fn deref(&self) -> &Self::Target {
		&self.context
	}
}

impl Drop for EguiContext {
	fn drop(&mut self) {
		let egui::FullOutput {
			platform_output,
			needs_repaint: _,
			textures_delta,
			shapes,
		} = self.context.end_frame();

		if !platform_output.copied_text.is_empty() {
			*self.clipboard.borrow_mut() = platform_output.copied_text;
		}
		self.painter.borrow_mut().paint_jobs = self.context.tessellate(shapes);
		self.painter.borrow_mut().textures_delta.push_front(textures_delta);
	}
}

/// Contains and handles everything related to [`egui`]
#[derive(Default)]
pub struct EguiBackend {
	context: egui::Context,
	pub input: Input,
	painter: Rc<RefCell<Painter>>,
}

impl EguiBackend {
	/// Create a [`EguiBackend`] with extra information for use the [`Input::set_scale_factor`]
	pub fn new(ctx: &ggez::Context) -> Self {
		let mut input = Input::default();
		let (w, h) = ctx.gfx.drawable_size();
		input.set_scale_factor(1.0, (w, h));
		
		Self {
			input,
			..Default::default()
		}
	}

	/// Return an [`EguiContext`] for update the gui
	pub fn ctx(&mut self) -> EguiContext {
		self.context.begin_frame(self.input.take());
		EguiContext {
			context: self.context.clone(),
			painter: self.painter.clone(),
			clipboard: self.input.clipboard.clone(),
		}
	}

	/// Draws the GUI on the canvas
	/// this funtion comes from [`Drawable`] trait that allow the struct use the function [`ggez::graphics::draw`]
	/// * Example
	/// ```
	/// struct State {
	/// 	egui_backend: EguiBackend
	/// }
	/// 
	/// impl EventHandler<ggez::GameError> for State {
	/// 	fn draw(&mut self, ctx: &mut ggez::Context) -> {
	/// 		ggez::graphics::draw(ctx, &self.egui_backend, ([0.0, 0.0],))?;
	/// 		...
	/// 	} 
	/// 	...
	/// }
	/// ```
	pub fn draw(&mut self, ctx: &mut ggez::Context, canvas: &mut graphics::Canvas) -> ggez::GameResult {
		let drawable = ctx.gfx.drawable_size();
		self.input.set_scale_factor(1.0, (drawable.0, drawable.1));
		self.painter.borrow_mut().draw(ctx, canvas, self.context.pixels_per_point(), self.input.scale_factor)
	}
}

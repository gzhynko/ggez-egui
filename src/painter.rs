use std::collections::{HashMap, LinkedList};

use ggez::graphics;
use wgpu;
use egui_wgpu;

#[derive(Default, Clone)]
pub struct Painter {
	render_pass: Rc<egui_wgpu::renderer::RenderPass>,
	pub(crate) paint_jobs: Vec<egui::ClippedPrimitive>,
	pub(crate) textures_delta: LinkedList<egui::TexturesDelta>,
	textures: HashMap<egui::TextureId, graphics::Image>,
}

impl Painter {
	pub fn new(ctx: &ggez::Context) -> Self {
		Self {

		}
	}

	pub fn draw(&mut self, ctx: &mut ggez::Context, canvas: &mut graphics::Canvas, pixels_per_point: f32, scale_factor: f32) -> ggez::GameResult {
		// Create and free textures
		if let Some(textures_delta) = self.textures_delta.pop_front() {
			self.update_textures(ctx, textures_delta)?;
		}

		let drawable_size = ctx.gfx.drawable_size();
		let width_in_points = drawable_size.0 / pixels_per_point;
        let height_in_points = drawable_size.1 / pixels_per_point;

		let wgpu = ctx.gfx.wgpu();
		let output_frame = match wgpu.surface.get_current_texture() {
			Ok(frame) => frame,
			Err(wgpu::SurfaceError::Outdated) => {
				// This error occurs when the app is minimized on Windows.
				// Silently return here to prevent spamming the console with:
				// "The underlying surface has changed, and therefore the swap chain must be updated"
				return Ok(());
			}
			Err(e) => {
				eprintln!("Dropped frame with error: {}", e);
				return Ok(());
			}
		};
		let output_view = output_frame
		.texture
		.create_view(&wgpu::TextureViewDescriptor::default());

		

		canvas.set_blend_mode(ggez::graphics::BlendMode::PREMULTIPLIED);
/*
		// drawing meshes
		for egui::ClippedPrimitive { primitive, clip_rect } in self.paint_jobs.as_slice() {
			match primitive {
				egui::epaint::Primitive::Mesh(mesh) => {
					let vertices = mesh.vertices.iter().map(|v| {
						let pos_x = 2.0 * v.pos.x / width_in_points - 1.0;
						let pos_y = 1.0 - 2.0 * v.pos.y / height_in_points;
						//print!("pos_x: {}, pos_y: {}", &pos_x, pos_y);
						graphics::Vertex {
							position: [pos_x, pos_y],
							uv: [v.uv.x, v.uv.y],
							color: egui::Rgba::from(v.color).to_array(),
						}
					}).collect::<Vec<_>>();

					let mesh_data = graphics::MeshData {
						vertices: &vertices,
						indices: &mesh.indices,
					};
					let ggez_mesh = graphics::Mesh::from_data(
						&ctx.gfx,
						mesh_data,
					);

					// Transform clip rect to physical pixels:
					let clip_min_x = pixels_per_point * clip_rect.min.x;
					let clip_min_y = pixels_per_point * clip_rect.min.y;
					let clip_max_x = pixels_per_point * clip_rect.max.x;
					let clip_max_y = pixels_per_point * clip_rect.max.y;

					// Make sure clip rect can fit within a `u32`:
					let clip_min_x = clip_min_x.clamp(0.0, drawable_size.0 as f32);
					let clip_min_y = clip_min_y.clamp(0.0, drawable_size.1 as f32);
					let clip_max_x = clip_max_x.clamp(clip_min_x, drawable_size.0 as f32);
					let clip_max_y = clip_max_y.clamp(clip_min_y, drawable_size.1 as f32);

					let ggez_clip_rect = ggez::graphics::Rect::new(clip_min_x, clip_min_y, clip_max_x - clip_min_x, clip_max_y - clip_min_y);

					println!("draw mesh with {} vertices and {} indices; clip rect: {:?}", ggez_mesh.vertex_count(), ggez_mesh.index_count(), ggez_clip_rect);
					//canvas.set_scissor_rect(ggez_clip_rect)?;
					canvas.draw_textured_mesh(
						ggez_mesh,
						self.textures.get(&mesh.texture_id).map(|t| t.clone()).unwrap(),
						graphics::DrawParam::default().scale([1., 1.])
					);
					canvas.reset_scissor_rect();
				}
				egui::epaint::Primitive::Callback(_) => {
					panic!("Custom rendering callbacks are not implemented yet");
				}
			}

		}
		*/

		Ok(())
	}

	pub fn update_textures(&mut self, ctx: &mut ggez::Context, textures_delta: egui::TexturesDelta) -> ggez::GameResult {
		// set textures
		for (id, delta) in &textures_delta.set {
			let image = match &delta.image {
				egui::ImageData::Color(image) => {
					image.into_image(ctx)
				}
				egui::ImageData::Font(image) => {
					image.into_image(ctx)
				}
			}?;

			self.textures.insert(*id, image);
		}

		// free textures
		for id in &textures_delta.free {
			self.textures.remove(id);
		}

		Ok(())
	}
}

fn egui_rect_to_ggez_rect(egui_rect: egui::Rect) -> ggez::graphics::Rect {
	ggez::graphics::Rect::new(egui_rect.left(), egui_rect.top(), egui_rect.width(), egui_rect.height())
}

// Generate ggez Image from egui Texture
trait Image {
	fn into_image(&self, ctx: &mut ggez::Context) -> ggez::GameResult<graphics::Image>;
}

impl Image for egui::ColorImage {
	fn into_image(&self, ctx: &mut ggez::Context) -> ggez::GameResult<graphics::Image> {
		assert_eq!(
			self.width() * self.height(),
			self.pixels.len(),
			"Mismatch between texture size and texel count"
		);

		let mut pixels: Vec<u8> = Vec::with_capacity(self.pixels.len() * 4);

		for pixel in &self.pixels {
			pixels.extend(pixel.to_array());
		}

		Ok(graphics::Image::from_pixels(
			&ctx.gfx,
			&pixels,
			ggez::graphics::ImageFormat::Rgba8UnormSrgb,
			self.width() as u32,
			self.height() as u32,
		))
	}
}

impl Image for egui::FontImage {
	fn into_image(&self, ctx: &mut ggez::Context) -> ggez::GameResult<graphics::Image> {
		assert_eq!(
			self.width() * self.height(),
			self.pixels.len(),
			"Mismatch between texture size and texel count"
		);

		let mut pixels: Vec<u8> = Vec::with_capacity(self.pixels.len() * 4);

		let gamma = 1.0;
		for pixel in self.srgba_pixels(gamma) {
			pixels.extend(pixel.to_array());
		}

		Ok(graphics::Image::from_pixels(
			&ctx.gfx,
			&pixels,
			ggez::graphics::ImageFormat::Rgba8UnormSrgb,
			self.width() as u32,
			self.height() as u32,
		))
	}
}

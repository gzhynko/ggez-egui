use ggez::{
	*,
	event::*, mint::Vector2,
};
use ggez_egui::{egui, EguiBackend};

fn main() -> GameResult {
	let mut conf = ggez::conf::Conf::new();
	conf.window_mode.resizable = true;
	let (ctx, event_loop) = ContextBuilder::new("game_id", "author")
	.default_conf(conf)
	.build()?;

	let my_game = MyGame {
		egui_backend: EguiBackend::default(),
	};

	event::run(ctx, event_loop, my_game)
}

struct MyGame {
	egui_backend: EguiBackend
}

impl EventHandler<GameError> for MyGame {
	fn update(&mut self, ctx: &mut Context) -> GameResult {
		let egui_ctx = self.egui_backend.ctx();
		egui::Window::new("egui-window").show(&egui_ctx, |ui| {
			ui.label("a very nice gui :3");
			if ui.button("print \"hello world\"").clicked() {
				println!("hello world");
			}
			if ui.button("quit").clicked() {
				ggez::event::request_quit(ctx);
			}
		});
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		let mut canvas = graphics::Canvas::from_frame(
            &ctx.gfx,
            graphics::CanvasLoadOp::Clear(graphics::Color::WHITE),
        );

		//canvas.set_screen_coordinates(graphics::Rect { x: 0., y: 0., w: ctx.gfx.drawable_size().0, h: ctx.gfx.drawable_size().0 });
		self.egui_backend.draw(ctx, &mut canvas)?;
		let mesh = graphics::Mesh::from_data(&ctx.gfx, graphics::MeshBuilder::new().circle(graphics::DrawMode::Fill(graphics::FillOptions::DEFAULT), Vector2 { x: 0., y: 0. }, 100., 3., graphics::Color::BLACK)?.build());
		//canvas.draw(&mesh, graphics::DrawParam::default().dest(Vector2 { x: 200., y: 200. }));

        canvas.finish(&mut ctx.gfx)?;

        Ok(())
	}

	fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) -> Result<(), GameError> {
		self.egui_backend.input.mouse_button_down_event(button);
		Ok(())
	}

	fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) -> Result<(), GameError> {
		self.egui_backend.input.mouse_button_up_event(button);
		Ok(())
	}

	fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) -> Result<(), GameError> {
		self.egui_backend.input.mouse_motion_event(x, y);
		Ok(())
	}
}

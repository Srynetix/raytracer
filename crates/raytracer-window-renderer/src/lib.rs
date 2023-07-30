use pixels::{Pixels, SurfaceTexture};
use raytracer_core::{Image, Renderer};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, WindowButtons},
};

#[derive(Default)]
pub struct WindowRenderer;

impl WindowRenderer {
    pub fn new() -> Self {
        Default::default()
    }

    fn render_image(image: &Image, pixels: &mut [u8]) {
        for (idx, p) in pixels.chunks_exact_mut(4).enumerate() {
            let pixel = image.pixels()[idx];
            p.copy_from_slice(&pixel.to_u8x4());
        }
    }
}

impl Renderer for WindowRenderer {
    fn render(&mut self, image: &raytracer_core::Image) -> Result<(), std::io::Error> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(image.width(), image.height()))
            .with_resizable(false)
            .with_title("raytracer-window-renderer")
            .with_enabled_buttons(WindowButtons::CLOSE | WindowButtons::MINIMIZE)
            .build(&event_loop)
            .unwrap();

        let mut pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(window_size.width, window_size.height, surface_texture).unwrap()
        };

        // Initial draw
        Self::render_image(image, pixels.frame_mut());
        pixels.render().unwrap();

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            if let Event::RedrawRequested(_) = event {
                pixels.render().unwrap();
            }

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == window.id() => *control_flow = ControlFlow::Exit,
                _ => (),
            }
        });
    }
}

use std::convert::TryInto;

use glutin::event_loop::EventLoop;
use glutin::dpi;
use glutin::dpi::PhysicalSize;
use glutin::window::Window as GlutinWindow;
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use glutin::event::Event;
use glutin::event::WindowEvent;
use glutin::event_loop::ControlFlow;
use glutin::ContextWrapper;
use glutin::GlProfile;
use glutin::PossiblyCurrent;

use skia_safe::gpu::gl::Format;
use skia_safe::gpu::gl::FramebufferInfo;
use skia_safe::gpu::BackendRenderTarget;
use skia_safe::gpu::DirectContext;
use skia_safe::gpu::SurfaceOrigin;
use skia_safe::ColorType;
use skia_safe::Surface;

pub type WindowedContext = ContextWrapper<PossiblyCurrent, GlutinWindow>;

pub struct Window {
    pub handle: WindowedContext,
    pub context: DirectContext,
    pub surface: Surface,
}

fn main() {
    let event_loop = EventLoop::new();

    // Build the window.
    let inner_size = dpi::Size::new(dpi::LogicalSize::new(512, 256));
    let builder = WindowBuilder::new()
        .with_title("help me")
        .with_inner_size(inner_size);

    // Get a handle to the window.
    let handle = ContextBuilder::new()
        .with_depth_buffer(0)
        .with_stencil_buffer(8)
        .with_pixel_format(24, 8)
        .with_double_buffer(Some(true))
        .with_gl_profile(GlProfile::Core)
        .build_windowed(builder, &event_loop)
        .expect("Failed creating window context");
    let handle = unsafe { handle.make_current().unwrap() };

    // Setup an opengl context.
    gl::load_with(|addr| handle.get_proc_address(&addr));
    let mut context = DirectContext::new_gl(None, None).expect("Failed creating gl context");

    // Setup the framebuffer.
    let fb_info = {
        let mut fboid: gl::types::GLint = 0;
        unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };

        FramebufferInfo {
            fboid: fboid.try_into().unwrap(),
            format: Format::RGBA8.into(),
        }
    };

    // Create a surface to draw on.
    let pixel_format = handle.get_pixel_format();
    let scale_factor = handle.window().scale_factor();
    let inner_size: PhysicalSize<u32> = inner_size.to_physical(scale_factor);
    let backend_render_target = BackendRenderTarget::new_gl(
        (
            inner_size.width.try_into().unwrap(),
            inner_size.height.try_into().unwrap(),
        ),
        pixel_format
            .multisampling
            .map(|samp| samp.try_into().unwrap()),
        pixel_format.stencil_bits.try_into().unwrap(),
        fb_info,
    );
    let surface = Surface::from_backend_render_target(
        &mut context,
        &backend_render_target,
        SurfaceOrigin::TopLeft,
        ColorType::RGBA8888,
        None,
        None,
    )
    .expect("Failed creating skia surface");

    // Run the event loop.
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::RedrawRequested(_) => {
                println!("A: {:?}", handle);
                println!("B: {:?}", context.resource_cache_limit());
                println!("C: {:?}", surface.height());
            }

            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    println!("Close requested");
                    *control_flow = ControlFlow::Exit;
                }

                _ => (),
            },

            _ => (),
        }
    })
}

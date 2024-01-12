use std::borrow::Cow;

use crate::nodes::BaseNode;
use crate::core::node::{Node};
use crate::styles::{Unit, Position, Style, Bounds};

use wgpu::{ColorTargetState, CompareFunction, DepthStencilState, Extent3d, LoadOp, MultisampleState, Operations, RenderPassDepthStencilAttachment, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};
use wgpu::util::DeviceExt;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use bytemuck::{Pod, Zeroable};
use glyphon::{
    Attrs, Buffer, Color, FontSystem, Metrics, Resolution, Shaping, SwashCache, TextArea,
    TextAtlas, TextBounds, TextRenderer,
};

use crate::core::{Context, PositionPx, Vertex};
use crate::core::render::Primitive;
use crate::styles::preset::color::COLOR_WHITE;


// Define the uniform data
#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct Uniforms {
    screen_size: [f32; 2],
}

#[derive(Default, Debug)]
pub struct AppBuilder {
    min_width: Option<usize>,
    min_height: Option<usize>,

    width: Option<usize>,
    height: Option<usize>,

    max_width: Option<usize>,
    max_height: Option<usize>,

    title: String,

    resizable: Option<bool>
}

impl AppBuilder {
    pub fn new(title: String) -> Self {
        Self {
            title,
            .. Default::default()
        }
    }

    pub fn with_size(mut self, width: usize, height: usize) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn with_min_size(mut self, min_width: usize, min_height: usize) -> Self {
        self.min_width = Some(min_width);
        self.min_height = Some(min_height);
        self
    }

    pub fn with_max_size(mut self, max_width: usize, max_height: usize) -> Self {
        self.max_width = Some(max_width);
        self.max_height = Some(max_height);
        self
    }

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = Some(resizable);
        self
    }

    pub fn build<F: FnOnce(&mut Vec<Box<dyn Node>>)>(self, add_child: F) -> App {
        App::new(
            self.title,
            self.width.unwrap_or(1280),
            self.height.unwrap_or(720),
            self.resizable.unwrap_or(true),
            self.min_width,
            self.min_height,
            self.max_width,
            self.max_height,
            add_child,
        )
    }
}

pub struct App {
    root: Box<dyn Node>,

    title: String,

    resizable: bool,

    width: usize,
    height: usize,

    min_width: Option<usize>,
    min_height: Option<usize>,

    max_width: Option<usize>,
    max_height: Option<usize>,
}

impl App {
    pub fn new<F: FnOnce(&mut Vec<Box<dyn Node>>)>(
        title: String,
        width: usize,
        height: usize,
        resizable: bool,
        min_width: Option<usize>,
        min_height: Option<usize>,
        max_width: Option<usize>,
        max_height: Option<usize>,
        add_child: F
    ) -> Self {
        Self {
            root: BaseNode::new(Style {
                position: Position::Absolute(Unit::Px(0), Unit::Px(0)),
                content: Bounds::new(Unit::Px(width as isize), Unit::Px(height as isize)),
                background_color: COLOR_WHITE,
                ..Default::default()
            }, add_child).build(),
            title,
            width,
            height,
            min_width,
            min_height,
            max_width,
            max_height,
            resizable
        }
    }
    
    pub fn start(self) {
        // Default fonts
        // FontDb::get_mut().load("Inter", FontWeight::Thin, Path::new("../../assets/fonts/Inter-Thin.ttf")).expect("Unable to load font");
        // FontDb::get_mut().load("Inter", FontWeight::ExtraLight, Path::new("../../assets/fonts/Inter-ExtraLight.ttf")).expect("Unable to load font");
        // FontDb::get_mut().load("Inter", FontWeight::Light, Path::new("../../assets/fonts/Inter-Light.ttf")).expect("Unable to load font");
        // FontDb::get_mut().load("Inter", FontWeight::Normal, Path::new("../../assets/fonts/Inter-Normal.ttf")).expect("Unable to load font");
        // FontDb::get_mut().load("Inter", FontWeight::Medium, Path::new("../../assets/fonts/Inter-Medium.ttf")).expect("Unable to load font");
        // FontDb::get_mut().load("Inter", FontWeight::SemiBold, Path::new("../../assets/fonts/Inter-SemiBold.ttf")).expect("Unable to load font");
        // FontDb::get_mut().load("Inter", FontWeight::Bold, Path::new("../../assets/fonts/Inter-Bold.ttf")).expect("Unable to load font");
        // FontDb::get_mut().load("Inter", FontWeight::ExtraBold, Path::new("../../assets/fonts/Inter-ExtraBold.ttf")).expect("Unable to load font");
        // FontDb::get_mut().load("Inter", FontWeight::Black, Path::new("../../assets/fonts/Inter-Black.ttf")).expect("Unable to load font");

        let event_loop = EventLoop::new();
        #[allow(unused_mut)]
        let mut builder = winit::window::WindowBuilder::new()
            .with_inner_size(LogicalSize::new(self.width as f32, self.height as f32))
            .with_title(self.title.clone())
            .with_resizable(self.resizable);

        if let (Some(min_width), Some(min_height)) = (self.min_width.clone(), self.min_height.clone()) {
            builder = builder.with_min_inner_size(LogicalSize::new(min_width as f32, min_height as f32));
        }

        if let (Some(max_width), Some(max_height)) = (self.max_width.clone(), self.max_height.clone()) {
            builder = builder.with_max_inner_size(LogicalSize::new(max_width as f32, max_height as f32));
        }

        let window = builder.build(&event_loop).unwrap();

        #[cfg(not(target_arch = "wasm32"))]
        {
            env_logger::init();
            pollster::block_on(self.run(event_loop, window));
        }
    }

    async fn run(mut self, event_loop: EventLoop<()>, window: Window) {
        let size = window.inner_size();

        let instance = wgpu::Instance::default();

        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                // Request an adapter which can render to our surface
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");

        // Create the logical device and command queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                    limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        // Text texture
        let depth_texture = device.create_texture(&TextureDescriptor {
            label: Some("Texture for text"),
            size: Extent3d {
                width: size.width,
                height: size.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Depth32Float,
            usage: TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let depth_view = depth_texture.create_view(&Default::default());

        // Load the shaders from disk
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("../../assets/shaders/basic.wgsl"))),
        });

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let vs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Vertex Shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("../../assets/shaders/basic.wgsl"))),
        });

        let fs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Fragment Shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("../../assets/shaders/basic.wgsl"))),
        });

        // Create a uniform buffer
        let uniforms = Uniforms { screen_size: [size.width as f32, size.height as f32] };
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::bytes_of(&uniforms),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Create a bind group to bind the uniform buffer to the pipeline
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("uniform_bind_group_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(8),
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("uniform_bind_group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vs_module,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x4,
                        },
                        wgpu::VertexAttribute {
                            offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                            shader_location: 1,
                            format: wgpu::VertexFormat::Float32x4,
                        },
                    ],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &fs_module,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format: TextureFormat::Bgra8UnormSrgb,
                    write_mask: wgpu::ColorWrites::ALL,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: Default::default(),
                conservative: false,
            },
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: CompareFunction::Less,
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let mut config = wgpu::SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: swapchain_capabilities.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        let mut font_system = FontSystem::new();
        let mut cache = SwashCache::new();
        let mut atlas = TextAtlas::new(&device, &queue, swapchain_format);
        let mut text_renderer = TextRenderer::new(&mut atlas, &device, MultisampleState::default(), Some(DepthStencilState {
            format: TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: CompareFunction::Less,
            stencil: Default::default(),
            bias: Default::default(),
        }));

        event_loop.run(move |event, _, control_flow| {
            // Have the closure take ownership of the resources.
            // `event_loop.run` never returns, therefore we must do this to ensure
            // the resources are properly cleaned up.
            let _ = (&instance, &adapter, &shader, &pipeline_layout);

            let mut ctx = Context::default();

            *control_flow = ControlFlow::Wait;
            match event {
                Event::WindowEvent {
                    event: WindowEvent::Resized(size),
                    ..
                } => {
                    // Reconfigure the surface with the new size
                    config.width = size.width;
                    config.height = size.height;
                    surface.configure(&device, &config);
                    // On macos the window needs to be redrawn manually after resizing
                    window.request_redraw();
                }
                Event::WindowEvent {
                    event: WindowEvent::CursorMoved{ position, ..},
                    ..
                } => {
                    self.root.on_cursor_move(&PositionPx {
                        x: position.x as isize,
                        y: position.y as isize
                    });

                    window.request_redraw();
                }
                Event::RedrawRequested(_) => {
                    self.root.calculate_size();
                    self.root.render(None, &mut ctx);

                    let mut vertex_data = ctx.get_vertex_data();
                    vertex_data.reverse();

                    let mut text_buffer_data = vec![];

                    for text in ctx.get_text_data() {
                        match text {
                            Primitive::Text { x, y, z, value, width, height, family, font_size, weight, line_height, color } => {
                                let mut buffer = Buffer::new(
                                    &mut font_system,
                                    Metrics::new(*font_size, *line_height));

                                buffer.set_size(
                                    &mut font_system,
                                    *width,
                                    *height
                                );
                                buffer.set_text(
                                    &mut font_system,
                                    value,
                                    Attrs::new().family(family.as_family()).weight(*weight).metadata(*z as usize),
                                    Shaping::Advanced);
                                buffer.shape_until_scroll(&mut font_system);

                                text_buffer_data.push((buffer, *x, *y, *z, *width, *height, *color));
                            },
                            _ => {}
                        }
                    }

                    let text_areas = text_buffer_data.iter().map(|(buffer, x, y, z, width, height, color)|
                        TextArea {
                            buffer,
                            left: *x,
                            top: *y,
                            scale: 1.0,
                            bounds: TextBounds {
                                left: 0,
                                top: 0,
                                right: (*x + *width) as i32,
                                bottom: (*y + *height) as i32,
                            },
                            default_color: Color::rgba(color.red(), color.green(), color.blue(), color.alpha()),
                        }
                    ).collect::<Vec<_>>();

                    text_renderer
                        .prepare_with_depth(
                            &device,
                            &queue,
                            &mut font_system,
                            &mut atlas,
                            Resolution {
                                width: config.width,
                                height: config.height,
                            },
                            text_areas,
                            &mut cache,
                            |z| { z as f32 / 10000.0  }
                        )
                        .unwrap();

                    let your_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Vertex Buffer"),
                        contents: bytemuck::cast_slice(&vertex_data),
                        usage: wgpu::BufferUsages::VERTEX,
                    });

                    let frame = surface
                        .get_current_texture()
                        .expect("Failed to acquire next swap chain texture");
                    let view = frame
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());
                    let mut encoder =
                        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                    {
                        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: None,
                            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                view: &view,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                    store: true,
                                },
                            })],
                            depth_stencil_attachment: Some(RenderPassDepthStencilAttachment{
                                view: &depth_view,
                                depth_ops: Some(Operations {
                                    load: LoadOp::Clear(1.0),
                                    store: true,
                                }),
                                stencil_ops: None,
                            }),
                        });
                        rpass.set_bind_group(0, &bind_group, &[]);

                        rpass.set_pipeline(&render_pipeline);
                        rpass.set_vertex_buffer(0, your_vertex_buffer.slice(..));
                        rpass.draw(0..vertex_data.len() as u32, 0..1);

                        text_renderer.render(&atlas, &mut rpass).unwrap();
                    }

                    queue.submit(Some(encoder.finish()));
                    frame.present();

                    atlas.trim();
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => *control_flow = ControlFlow::Exit,
                _ => {}
            }
        });
    }
}

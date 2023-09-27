use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit::window::Window;
use wgpu::util::DeviceExt;
use device_query::{DeviceState, DeviceQuery};
use crate::{ Vertex, Plane, ReferencePoint };

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    device_state: DeviceState,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    // The window must be declared after the surface so
    // it gets dropped after it as the surface contains
    // unsafe references to the window's resources.
    window: Window,

    plane: Plane,
    ref_point: ReferencePoint,

    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
}

impl State {
    // Creating some of the wgpu types requires async code
    async fn new(window: Window, plane: Plane, ref_point: ReferencePoint) -> Self {
        let mesh = plane.mesh_from_ref(&ref_point);
        
        let vertices: &[Vertex] = &mesh.vertices;
        let indices: &[i16] = &mesh.indices;
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });
        
        // # Safety
        //
        // The surface needs to live as long as the window that created it.
        // State owns the window so this should be safe.
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance
    .enumerate_adapters(wgpu::Backends::all())
    .find(|adapter| {
        // Check if this adapter supports our surface
        adapter.is_surface_supported(&surface)
    })
    .unwrap();


        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None, // Trace path
        ).await.unwrap();
        
        let device_state = DeviceState::new();

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())            
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let render_pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main", // 1.
                buffers: &[
                    Vertex::desc(),
                ], // 2.
            },
            fragment: Some(wgpu::FragmentState { // 3.
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState { // 4.
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState {
                count: 1, // 2.
                mask: !0, // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None, // 5.
        });

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        let num_indices = indices.len() as u32;

        Self {
            window,
            surface,
            device,
            device_state,
            queue,
            config,
            size,
            plane,
            ref_point,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(virtual_keycode),..
                },..
            } => match virtual_keycode {
                VirtualKeyCode::Up => {
                    self.ref_point.position = (self.ref_point.position.0, self.ref_point.position.1 + 1.0);
                    self.update();
                    true
                },
                VirtualKeyCode::Right => {
                    self.ref_point.position = (self.ref_point.position.0 + 1.0, self.ref_point.position.1);
                    self.update();
                    true
                },
                VirtualKeyCode::Down => {
                    self.ref_point.position = (self.ref_point.position.0, self.ref_point.position.1 - 1.0);
                    self.update();
                    true
                },
                VirtualKeyCode::Left => {
                    self.ref_point.position = (self.ref_point.position.0 - 1.0, self.ref_point.position.1);
                    self.update();
                    true
                },
                _ => false
                //println!("state: {0}, key: {1}", *state == ElementState::Pressed, match virtual_keycode {
                //    VirtualKeyCode::Up => "up_key",
                //    _ => "unkn"
                //});
            },
            _ => return false
        }
    }

    fn update(&mut self) {
        self.plane.paint_antialiased_filled_circle(self.ref_point.position.1, self.ref_point.position.0, 3.0);
        println!("{}", self.ref_point.position.1);
        let mut mesh = self.plane.mesh_from_ref(&self.ref_point);

        //mesh.scale(0.25);
        
        let vertices: &[Vertex] = &mesh.vertices;
        let indices: &[i16] = &mesh.indices;

        let vertex_buffer = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let index_buffer = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        let num_indices = indices.len() as u32;

        self.vertex_buffer = vertex_buffer;
        self.index_buffer = index_buffer;
        self.num_indices = num_indices;
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let _keys = self.device_state.get_keys();
        let mouse = self.device_state.get_mouse();

        let window_pos = match self.window.inner_position() {
            Ok(i) => (i.x as f32 + 1.0, i.y as f32 + 1.0),
            _ => (0.0, 0.0)
        };

        let window_size = match self.window.inner_size() {
            i => (i.width as f32, i.height as f32),
        };

        let _mouse_pos = (((mouse.coords.0 as f32 - window_pos.0)/(window_size.0)), ((mouse.coords.1 as f32 - window_pos.1)/(window_size.1)));
        
        if ((mouse.coords.0 as f32) <= window_pos.0 + window_size.0) && (mouse.coords.0 as f32 >= window_pos.0)
        && ((mouse.coords.1 as f32) <= window_pos.1 + window_size.1) && (mouse.coords.1 as f32 >= window_pos.1) {
            //println!("mouse: {0}, {1}", mouse_pos.0, mouse_pos.1);
        }

        let output = self.surface.get_current_texture()?;

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline); // 2.
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16); // 1.
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1); // 3.
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    
        Ok(())
    }

    fn mouse_pos(&self) -> (f32, f32) {
        let mouse = self.device_state.get_mouse();

        let window_pos = match self.window.inner_position() {
            Ok(i) => (i.x as f32 + 1.0, i.y as f32 + 1.0),
            _ => (0.0, 0.0)
        };

        let window_size = match self.window.inner_size() {
            i => (i.width as f32, i.height as f32),
        };

        (((mouse.coords.0 as f32 - window_pos.0)/(window_size.0)), ((mouse.coords.1 as f32 - window_pos.1)/(window_size.1)))
    }
}

pub async fn run(plane: Plane) {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let reference = ReferencePoint {
        position: (0.0, 0.0),
        render_dist: 32.0
    };

    //let mesh = plane.mesh_from_ref(reference);

    //let vertices: &[Vertex] = &mesh.vertices;
    //let indices: &[i16] = &mesh.indices;
    
    let mut state = State::new(window, plane, reference).await;

    //state.update(vertices, indices);
    
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window().id() => if !state.input(event) { // UPDATED!
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
            Event::RedrawRequested(window_id) if window_id == state.window().id() => {

                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                state.window().request_redraw();
            }
            _ => {}
        }
    });
}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                }
            ]
        }
    }
}

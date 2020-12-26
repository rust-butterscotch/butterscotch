use butterscotch_common::{dpi::PixelsRaw, interop::WindowHandle};

/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

#[allow(dead_code)]
pub struct Renderer {
    adapter: wgpu::Adapter,

    device: wgpu::Device,
    queue: wgpu::Queue,

    instance: wgpu::Instance,
    surface: wgpu::Surface,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain
}

pub fn create_debug_pipeline(renderer: &Renderer) -> wgpu::RenderPipeline {
    let device = &renderer.device;
    let vs_module = device.create_shader_module(wgpu::include_spirv!("../../../examples/develop/output/subfolder/shader.vert.spv"));
    let fs_module = device.create_shader_module(wgpu::include_spirv!("../../../examples/develop/output/subfolder/shader.frag.spv"));

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    return device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex_stage: wgpu::ProgrammableStageDescriptor {
            module: &vs_module,
            entry_point: "main",
        },
        fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
            module: &fs_module,
            entry_point: "main",
        }),
        // Use the default rasterizer state: no culling, no depth bias
        rasterization_state: None,
        primitive_topology: wgpu::PrimitiveTopology::TriangleList,
        color_states: &[renderer.sc_desc.format.into()],
        depth_stencil_state: None,
        vertex_state: wgpu::VertexStateDescriptor {
            index_format: wgpu::IndexFormat::Uint16,
            vertex_buffers: &[],
        },
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false,
    });
}

impl Renderer {

    pub async fn new(
        window_handle: WindowHandle,
        window_size: PixelsRaw,
        //window: &dyn butterscotch_common::interop::WindowController
    ) -> Renderer {

        let swapchain_format = wgpu::TextureFormat::Bgra8Unorm;

        let instance = wgpu::Instance::new(wgpu::BackendBit::all());
        let surface = unsafe { instance.create_surface(&window_handle) };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                // Request an adapter which can render to our surface
                compatible_surface: None,//Some(&surface),
            })
            .await
            .expect("Failed to find an appropiate adapter");

        
        // Create the logical device and command queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    shader_validation: true,
                },
                None,
            )
            .await
            .expect("Failed to create device");


        let size = window_size.raw_u64();

        // Create swapchain
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT, // TODO will become RENDER_ATTACHMENT
            format: swapchain_format,  // TODO sRGB option
            width: size.0 as u32,
            height: size.1 as u32,
            present_mode: wgpu::PresentMode::Mailbox,
        };

        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        return Renderer{
            adapter,
            device,
            queue,
            instance,
            surface,
            sc_desc,
            swap_chain
        };
    }

    pub fn resize(&mut self, window_size: PixelsRaw) {
        let raw = window_size.raw_u64();
        self.sc_desc.width  = raw.0 as u32;
        self.sc_desc.height = raw.1 as u32;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    pub fn render(&mut self, window_size: PixelsRaw, pipeline: &wgpu::RenderPipeline) {

        let frame = match self.swap_chain.get_current_frame() {
            Ok(frame) => frame,
            Err(_) => {
                self.resize(window_size);
                self.swap_chain.get_current_frame().expect("Failed to acquire next swap chain texture!")
            }
        }.output;

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
            rpass.set_pipeline(pipeline);
            rpass.draw(0..3, 0..1);
        }

        self.queue.submit(Some(encoder.finish()));
    }

}
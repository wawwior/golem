use std::{num::NonZero, sync::Arc};

use crate::{app::AppState, data::FaceData};

pub const VERTEX_BUFFER_SIZE: u64 = 16 * 16 * 16;

pub trait Render {
    fn render(&mut self);
}

#[derive(Debug)]
pub struct FaceGroup {
    pub bind_group_layout: wgpu::BindGroupLayout,
    vertex_buffer: Arc<wgpu::Buffer>,
    bind_group: Option<wgpu::BindGroup>,
    face_data: [Vec<FaceData>; 6],
    invalidated: bool,
}

impl FaceGroup {
    pub fn new(device: &wgpu::Device, vertex_buffer: Arc<wgpu::Buffer>) -> Self {
        Self {
            bind_group_layout: device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Face Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: Some(
                                NonZero::new(std::mem::size_of::<u32>() as u64 * 7)
                                    .expect("size_of::<u32>() > 0"),
                            ),
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: false },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            }),
            vertex_buffer,
            bind_group: Default::default(),
            face_data: Default::default(),
            invalidated: true,
        }
    }

    fn bind_group(&mut self, device: &wgpu::Device) -> &wgpu::BindGroup {
        if self.invalidated {
            use wgpu::util::DeviceExt;
            self.invalidated = false;
            self.bind_group = Some(
                device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("Face Bind Group"),
                    layout: &self.bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: device
                                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                                    label: Some("Face Offset Buffer"),
                                    contents: bytemuck::cast_slice(
                                        &[self.face_data.as_slice(), &[vec![FaceData::default()]]]
                                            .concat()
                                            .iter()
                                            .map(|_| self.face_data.clone())
                                            .enumerate()
                                            .map(|(n, l)| {
                                                l.iter().take(n + 1).map(|v| v.len() as u32).sum()
                                            })
                                            .collect::<Vec<u32>>(),
                                    ),
                                    usage: wgpu::BufferUsages::STORAGE,
                                })
                                .as_entire_binding(),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: device
                                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                                    label: Some("Face Data Buffer"),
                                    contents: &[
                                        bytemuck::cast_slice(&self.face_data.concat()),
                                        // Disallowing zero length buffers
                                        &[0],
                                    ]
                                    .concat(),
                                    usage: wgpu::BufferUsages::STORAGE,
                                })
                                .as_entire_binding(),
                        },
                        wgpu::BindGroupEntry {
                            binding: 2,
                            resource: self.vertex_buffer.as_entire_binding(),
                        },
                    ],
                }),
            )
        }
        self.bind_group
            .as_ref()
            .expect("FaceGroup is invalidated at creation.")
    }
}

impl Render for AppState {
    fn render(&mut self) {
        let surface_texture = self
            .surface
            .get_current_texture()
            .expect("failed to get current texture");

        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                format: Some(self.surface_format.add_srgb_suffix()),
                ..Default::default()
            });

        let mut encoder = self.device.create_command_encoder(&Default::default());

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Compute Pass"),
                timestamp_writes: None,
            });

            compute_pass.set_pipeline(&self.compute_pipeline);
            compute_pass.set_bind_group(0, self.face_group.bind_group(&self.device), &[]);
            // TODO: Variables
            compute_pass.dispatch_workgroups(1, 6, 1);
        }

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            // TODO: Variables
            render_pass.draw(0..6, 0..1);
        }

        self.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();
        surface_texture.present();
    }
}

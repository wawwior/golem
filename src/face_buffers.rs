use crate::{data::FaceData, direction::Direction};

#[derive(Debug)]
pub struct FaceBuffers {
    pub up: wgpu::Buffer,
    pub down: wgpu::Buffer,
    pub left: wgpu::Buffer,
    pub right: wgpu::Buffer,
    pub front: wgpu::Buffer,
    pub back: wgpu::Buffer,
    pub mask: u32,
}

impl FaceBuffers {
    pub fn new(
        device: &wgpu::Device,
        up: &[FaceData],
        down: &[FaceData],
        left: &[FaceData],
        right: &[FaceData],
        front: &[FaceData],
        back: &[FaceData],
    ) -> Self {
        let mut buffers = Self {
            up: Self::create_buffer(device, Direction::UP, up),
            down: Self::create_buffer(device, Direction::DOWN, down),
            left: Self::create_buffer(device, Direction::LEFT, left),
            right: Self::create_buffer(device, Direction::RIGHT, right),
            front: Self::create_buffer(device, Direction::FRONT, front),
            back: Self::create_buffer(device, Direction::BACK, back),
            mask: 0,
        };
        buffers.update_mask();
        buffers
    }

    pub fn empty(device: &wgpu::Device) -> Self {
        Self::new(device, &[], &[], &[], &[], &[], &[])
    }

    pub fn create_buffer(
        device: &wgpu::Device,
        direction: Direction,
        contents: &[FaceData],
    ) -> wgpu::Buffer {
        use wgpu::util::DeviceExt;
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{} Face Buffer", direction.name())),
            contents: bytemuck::cast_slice(contents),
            usage: wgpu::BufferUsages::STORAGE,
        })
    }

    pub fn update_mask(&mut self) {
        let bit_up = if self.up.size() == 0 { 0 } else { 1 };
        let bit_down = if self.down.size() == 0 { 0 } else { 1 };
        let bit_left = if self.left.size() == 0 { 0 } else { 1 };
        let bit_right = if self.right.size() == 0 { 0 } else { 1 };
        let bit_front = if self.front.size() == 0 { 0 } else { 1 };
        let bit_back = if self.back.size() == 0 { 0 } else { 1 };

        self.mask = 0
            | bit_up
            | bit_down << 1
            | bit_left << 2
            | bit_right << 3
            | bit_front << 4
            | bit_back << 5;
    }

    pub fn create_face_group(
        &self,
        device: &wgpu::Device,
        face_group_layout: &wgpu::BindGroupLayout,
        vertex_buffer: &wgpu::Buffer,
    ) -> wgpu::BindGroup {
        let mut up = self.up.clone();
        let mut down = self.down.clone();
        let mut left = self.left.clone();
        let mut right = self.right.clone();
        let mut front = self.front.clone();
        let mut back = self.back.clone();

        if self.up.size() == 0 {
            up = Self::create_buffer(device, Direction::UP, &[FaceData::default()])
        }
        if self.down.size() == 0 {
            down = Self::create_buffer(device, Direction::DOWN, &[FaceData::default()])
        }
        if self.left.size() == 0 {
            left = Self::create_buffer(device, Direction::LEFT, &[FaceData::default()])
        }
        if self.right.size() == 0 {
            right = Self::create_buffer(device, Direction::RIGHT, &[FaceData::default()])
        }
        if self.front.size() == 0 {
            front = Self::create_buffer(device, Direction::FRONT, &[FaceData::default()])
        }
        if self.back.size() == 0 {
            back = Self::create_buffer(device, Direction::BACK, &[FaceData::default()])
        }
        use wgpu::util::DeviceExt;
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Face Bind Group"),
            layout: &face_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: vertex_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: device
                        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: Some("Face Mask Buffer"),
                            contents: bytemuck::cast_slice(&[self.mask]),
                            usage: wgpu::BufferUsages::UNIFORM,
                        })
                        .as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: up.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: down.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: left.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: right.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 6,
                    resource: front.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 7,
                    resource: back.as_entire_binding(),
                },
            ],
        })
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct FaceData {
    position: u16,
    u: u8,
    v: u8,
}

impl FaceData {
    pub fn new(x: u8, y: u8, z: u8, u: u8, v: u8) -> Self {
        Self {
            position: 0u16 | (x as u16 & 15u16) | (y as u16 & 15u16 << 4) | (z as u16 & 15u16 << 8),
            u,
            v,
        }
    }
}

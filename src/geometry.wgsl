
@group(0)
@binding(0)
var<storage> face_offsets: array<u32, 7>;

@group(0)
@binding(1)
var<storage> face_buffer: array<u32>;

@group(0)
@binding(2)
var<storage, read_write> vertex_buffer: array<f32>;

const mats: array<mat4x3<f32>, 6> = array(
    // Up
    mat4x3(
        vec3<f32>(0, 0, 0),
        vec3<f32>(0, 0, 0),
        vec3<f32>(0, 0, 0),
        vec3<f32>(0, 0, 0),
    ),
    // Down
    mat4x3(
        vec3<f32>(0, 0, 0),
        vec3<f32>(0, 0, 0),
        vec3<f32>(0, 0, 0),
        vec3<f32>(0, 0, 0),
    ),
    // Left
    mat4x3(
        vec3<f32>(0, 0, 0),
        vec3<f32>(0, 0, 0),
        vec3<f32>(0, 0, 0),
        vec3<f32>(0, 0, 0),
    ),
    // Right
    mat4x3(
        vec3<f32>(0, 0, 0),
        vec3<f32>(0, 0, 0),
        vec3<f32>(0, 0, 0),
        vec3<f32>(0, 0, 0),
    ),
    // Front
    mat4x3(
        vec3<f32>(0, 0, 0),
        vec3<f32>(0.5, 0, 0),
        vec3<f32>(0, 0.5, 0),
        vec3<f32>(0.5, 0.5, 0),
    ),
    // Back
    mat4x3(
        vec3<f32>(0, 0, 0),
        vec3<f32>(0, 0.5, 0),
        vec3<f32>(0.5, 0, 0),
        vec3<f32>(0.5, 0.5, 0),
    ),
);

@compute
@workgroup_size(1)
fn main(
    @builtin(global_invocation_id) id: vec3<u32>
) {
    let offset = face_offsets[id.x];
    let len = face_offsets[id.x + 1] - offset;
    if id.y < len {
        let face_index = (offset +  id.y); // Counted in u32
        let vertex_index = (offset + id.y) * 6; // Counted in f32x3

        let face_pos = vec3<f32>(
            f32(face_buffer[face_index] & 15),
            f32(face_buffer[face_index] & (15 << 4) >> 4),
            f32(face_buffer[face_index] & (15 << 8) >> 8)
        );

        let mat = mat4x3<f32>(face_pos, face_pos, face_pos, face_pos);

        let vertices = mat + mats[id.x];

        set_vertex(vertex_index, vertices[0]);
        set_vertex(vertex_index, vertices[1]);
        set_vertex(vertex_index, vertices[2]);
        set_vertex(vertex_index, vertices[2]);
        set_vertex(vertex_index, vertices[1]);
        set_vertex(vertex_index, vertices[3]);
    }
}

fn set_vertex(index: u32, vertex: vec3<f32>) {
    vertex_buffer[index * 3] = vertex.x;
    vertex_buffer[index * 3 + 1] = vertex.y;
    vertex_buffer[index * 3 + 2] = vertex.z;
}

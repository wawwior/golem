struct VertexInput {
    @location(0) position: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.color = (model.position + vec3<f32>(1.0, 1.0, 1.0)) / 2;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}

@group(0)
@binding(0)
var<storage, read_write> vertex_buffer: array<f32>;

@group(0)
@binding(1)
var<storage, read> face_buffer: array<u32>;

@compute
@workgroup_size(1)
fn cp_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let x = f32(face_buffer[id.x] & 15);
    let y = f32((face_buffer[id.x] >> 4) & 15);
    let z = f32((face_buffer[id.x] >> 8) & 15);

    let u = (face_buffer[id.x] >> 16) & 255;
    let v = (face_buffer[id.x] >> 24) & 255;

    write_vertex(id.x * 6, vec3<f32>(x , y, z));
    write_vertex(id.x * 6 + 1, vec3<f32>(x + 0.5, y, z));
    write_vertex(id.x * 6 + 2, vec3<f32>(x, y + 0.5, z));
    write_vertex(id.x * 6 + 3, vec3<f32>(x + 0.5, y, z));
    write_vertex(id.x * 6 + 4, vec3<f32>(x + 0.5, y + 0.5, z));
    write_vertex(id.x * 6 + 5, vec3<f32>(x, y + 0.5, z));

} 


fn write_vertex(index: u32, vert: vec3<f32>) {
    vertex_buffer[index * 3] = vert.x;
    vertex_buffer[index * 3 + 1] = vert.y;
    vertex_buffer[index * 3 + 2] = vert.z;
}

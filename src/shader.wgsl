// Vertex shader

[[block]] // 1.
struct CameraUniform {
    projection: mat4x4<f32>;
};
[[block]]
struct SpriteUniform {
    model: mat4x4<f32>;
};

[[group(1), binding(0)]]
var<uniform> camera: CameraUniform;
[[group(2), binding(0)]]
var<uniform> sprite: SpriteUniform;

struct VertexInput {
    [[location(0)]] position: vec2<f32>;
    [[location(1)]] tex_coords: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] tex_coords: vec2<f32>;
};

[[stage(vertex)]]
fn vs_main(
    vertex: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = vertex.tex_coords;
    out.clip_position = camera.projection * sprite.model * vec4<f32>(vertex.position, 0.0, 1.0);
    return out;
}

// Fragment shader

[[group(0), binding(0)]]
var t_diffuse: texture_2d<f32>;
[[group(0), binding(1)]]
var s_diffuse: sampler;

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
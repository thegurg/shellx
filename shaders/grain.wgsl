// Subtle film grain over the rectangle fill.
// values[0].x = intensity (0..1), values[0].y = scale
@fragment
fn main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let intensity = select(0.06, values[0].x, values[0].x > 0.0);
    let scale = select(1.0, values[0].y, values[0].y > 0.0);
    let p = uv * size * scale;
    // cheap hash noise
    let n = fract(sin(dot(p + vec2<f32>(time * 60.0, time * 27.0), vec2<f32>(12.9898, 78.233))) * 43758.5453);
    let g = (n - 0.5) * intensity;
    // neutral grain as overlay: slight light/dark variation
    let c = 0.5 + g;
    return vec4<f32>(c, c, c, intensity * 0.55);
}

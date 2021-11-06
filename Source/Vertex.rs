pub trait Vertex {
    fn GetDescriptor<'a>() -> wgpu::VertexBufferLayout<'a>;
}

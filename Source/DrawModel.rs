use crate::{Material, Mesh, Model};
use std::ops::Range;

pub trait DrawModel<'a> {
    fn DrawMesh(
        &mut self,
        mesh: &'a Mesh,
        material: &'a Material,
        camera_bind_group: &'a wgpu::BindGroup,
    );
    fn DrawMeshInstanced(
        &mut self,
        mesh: &'a Mesh,
        material: &'a Material,
        instances: Range<u32>,
        camera_bind_group: &'a wgpu::BindGroup,
    );
    fn DrawModel(&mut self, model: &'a Model, camera_bind_group: &'a wgpu::BindGroup);
    fn DrawModelInstanced(
        &mut self,
        model: &'a Model,
        instances: Range<u32>,
        camera_bind_group: &'a wgpu::BindGroup,
    );
}

impl<'a> DrawModel<'a> for wgpu::RenderPass<'a> {
    fn DrawMesh(
        &mut self,
        mesh: &'a Mesh,
        material: &'a Material,
        camera_bind_group: &'a wgpu::BindGroup,
    ) {
        self.DrawMeshInstanced(mesh, material, 0..1, camera_bind_group);
    }

    fn DrawMeshInstanced(
        &mut self,
        mesh: &'a Mesh,
        material: &'a Material,
        instances: Range<u32>,
        camera_bind_group: &'a wgpu::BindGroup,
    ) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.set_bind_group(0, &material.bind_group, &[]);
        self.set_bind_group(1, camera_bind_group, &[]);
        self.draw_indexed(0..mesh.num_elements, 0, instances);
    }

    fn DrawModel(&mut self, model: &'a Model, camera_bind_group: &'a wgpu::BindGroup) {
        self.DrawModelInstanced(model, 0..1, camera_bind_group);
    }

    fn DrawModelInstanced(
        &mut self,
        model: &'a Model,
        instances: Range<u32>,
        camera_bind_group: &'a wgpu::BindGroup,
    ) {
        for mesh in &model.meshes {
            let material = &model.materials[mesh.material];
            self.DrawMeshInstanced(mesh, material, instances.clone(), camera_bind_group);
        }
    }
}

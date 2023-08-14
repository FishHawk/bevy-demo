use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::{Indices, MeshVertexBufferLayout},
        render_resource::{
            AsBindGroup, PrimitiveTopology, RenderPipelineDescriptor, ShaderRef,
            SpecializedMeshPipelineError,
        },
    },
    sprite::{Material2d, Material2dKey},
};

use super::{
    create_light2d_fragment_target, LIGHT2D_FALLOFF_LOOKUP_IMAGE_HANDLE,
    LIGHT2D_FREEFORM_MATERIAL_SHADER_HANDLE,
};

#[derive(AsBindGroup, Reflect, Debug, Clone, TypeUuid)]
#[reflect(Debug)]
#[uuid = "910394c9-b37b-4996-80fb-43f65f0c84c4"]
pub struct Light2dFreeformMaterial {
    #[uniform(0)]
    pub color: Color,
    #[uniform(0)]
    pub intensity: f32,
    #[uniform(0)]
    pub falloff: f32,
    #[texture(1)]
    #[sampler(2)]
    pub falloff_lookup: Handle<Image>,
}

impl Default for Light2dFreeformMaterial {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            intensity: 1.0,
            falloff: 0.5,
            falloff_lookup: LIGHT2D_FALLOFF_LOOKUP_IMAGE_HANDLE.clone().typed(),
        }
    }
}

impl Material2d for Light2dFreeformMaterial {
    fn fragment_shader() -> ShaderRef {
        LIGHT2D_FREEFORM_MATERIAL_SHADER_HANDLE.typed().into()
    }

    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        _: &MeshVertexBufferLayout,
        _: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        if let Some(ref mut fragment) = descriptor.fragment {
            fragment.targets = vec![Option::Some(create_light2d_fragment_target())];
            fragment.shader_defs.push("VERTEX_COLORS".into());
        }
        Ok(())
    }
}

pub fn freeform_polygon_mesh(positions: Vec<Vec2>, extend: f32) -> Mesh {
    let sides = positions.len();
    debug_assert!(sides > 2, "RegularPolygon requires at least 3 sides.");

    let mut positions_inner = Vec::with_capacity(sides);
    let mut colors = Vec::with_capacity(sides);

    for i in 0..sides {
        let pos = positions[i];
        positions_inner.push([pos.x, pos.y, 0.0]);
        colors.push([0.0, 0.0, 0.0, 1.0])
    }

    let mut indices = Vec::with_capacity((sides - 2) * 3);
    for i in 1..(sides as u32 - 1) {
        indices.extend_from_slice(&[0, i + 1, i]);
    }

    if extend > 0.0 {
        for i in 0..sides {
            let pos = positions[i];
            let v1 = (pos - positions[(i + sides - 1) % sides]).normalize();
            let v2 = (pos - positions[(i + 1) % sides]).normalize();

            let x1 = v1.x;
            let y1 = v1.y;
            let x2 = v2.x;
            let y2 = v2.y;

            let x = (x1 * (x2 * x2 + y2 * y2) + x2 * (x1 * x1 + y1 * y1)) / (x1 * y2 - y1 * x2);
            let y = -(y1 * (x2 * x2 + y2 * y2) + y2 * (x1 * x1 + y1 * y1)) / (y1 * x2 - x1 * y2);
            let pos = pos + Vec2::new(x, y) * extend;

            positions_inner.push([pos.x, pos.y, 0.0]);
            colors.push([0.0, 0.0, 0.0, 0.0])
        }

        for i in 0..sides {
            let sides = sides as u32;
            let i = i as u32;
            let j = (i + 1) % sides;
            indices.extend_from_slice(&[i, j, j + sides]);
            indices.extend_from_slice(&[i, j + sides, i + sides]);
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions_inner);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh
}

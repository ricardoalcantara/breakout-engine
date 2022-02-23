use super::vertex::{QuadOrigin, Vertex, CENTER_QUAD, TOP_LEFT_QUAD};
use super::{RenderQuad, RenderTexture, RenderVertices};
use crate::render::texture::Texture;
use crate::shapes::rectangle::Rect;
use std::rc::Rc;

pub const MAX_QUAD_COUNT: usize = 100000;
pub const MAX_VERTEX_COUNT: usize = MAX_QUAD_COUNT * 4;
pub const MAX_INDEX_COUNT: usize = MAX_QUAD_COUNT * 6;
pub const MAX_TEXTURE_COUNT: usize = 32;

pub enum RenderItem {
    RenderQuad(RenderQuad),
    RenderTexture(RenderTexture),
    RenderVertices(RenderVertices),
}

pub struct TextureBind {
    pub texture_bind_group: wgpu::BindGroup,
    pub from: u64,
    pub to: u64,
}

pub struct RenderStep {
    pub buffer_vertices: Vec<Vertex>,
    pub texture_binds: Vec<TextureBind>,
}

pub struct Render2dData {
    white_texture: Rc<Texture>,
    render_items: Vec<RenderItem>,
    texture_max: usize,
}

impl Render2dData {
    pub fn new(texture_max: usize, white_texture: Texture) -> Render2dData {
        assert!(
            texture_max <= MAX_TEXTURE_COUNT,
            "texture_max {} is higher than MAX_TEXTURE_COUNT {}",
            texture_max,
            MAX_TEXTURE_COUNT
        );

        let white_texture = Rc::new(white_texture);

        Render2dData {
            white_texture,
            texture_max,
            render_items: Vec::new(),
        }
    }

    pub fn add_render_item(&mut self, render_item: RenderItem) {
        self.render_items.push(render_item);
    }

    pub fn begin_batch(&mut self) {
        self.render_items.clear();
    }

    pub fn get_render_vertices_and_textures(
        &mut self,
        device: &wgpu::Device,
        texture_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> RenderStep {
        let mut render_steps = RenderStep {
            buffer_vertices: Vec::new(),
            texture_binds: Vec::new(),
        };

        let mut textures = Vec::new();
        let mut from = 0;

        for render_item in self.render_items.drain(..) {
            let (mut vertices, texture) = match render_item {
                RenderItem::RenderQuad(_render_quad) => {
                    todo!();
                }
                RenderItem::RenderTexture(_render_texture) => {
                    todo!();
                }
                RenderItem::RenderVertices(render_vertices) => (
                    render_vertices.raw_vertices(),
                    render_vertices
                        .texture
                        .unwrap_or(self.white_texture.clone()),
                ),
            };

            let mut tex_index = None;
            for (i, t) in textures.iter().enumerate() {
                if Rc::ptr_eq(t, &texture) {
                    tex_index = Some(i as u32);
                    break;
                }
            }

            if tex_index.is_none() {
                if textures.len() < self.texture_max {
                    textures.push(texture);
                    tex_index = Some(textures.len() as u32 - 1);
                } else {
                    let mut textures_bind_group_entries = Vec::new();

                    for (i, texture) in textures.iter().enumerate() {
                        if i == 0 {
                            textures_bind_group_entries.push(wgpu::BindGroupEntry {
                                binding: 0,
                                resource: wgpu::BindingResource::Sampler(&texture.sampler),
                            });
                        }

                        textures_bind_group_entries.push(wgpu::BindGroupEntry {
                            binding: i as u32 + 1,
                            resource: wgpu::BindingResource::TextureView(&texture.view),
                        })
                    }
                    let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                        layout: &texture_bind_group_layout,
                        entries: &textures_bind_group_entries,
                        label: Some("texture_bind_group"),
                    });

                    render_steps.texture_binds.push(TextureBind {
                        texture_bind_group,
                        from,
                        to: render_steps.buffer_vertices.len() as u64,
                    });

                    from = render_steps.buffer_vertices.len() as u64;
                    textures = Vec::new();
                    textures.push(texture);
                    tex_index = Some(textures.len() as u32 - 1);
                }
            }

            if let Some(tex_index) = tex_index {
                for v in &mut vertices {
                    v.tex_index = tex_index;
                }
            }

            render_steps.buffer_vertices.extend_from_slice(&vertices);
        }

        if render_steps.buffer_vertices.len() as u64 > from {
            for _ in textures.len()..self.texture_max {
                textures.push(self.white_texture.clone());
            }

            let mut textures_bind_group_entries = Vec::new();

            for (i, texture) in textures.iter().enumerate() {
                if i == 0 {
                    textures_bind_group_entries.push(wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::Sampler(&texture.sampler),
                    });
                }

                textures_bind_group_entries.push(wgpu::BindGroupEntry {
                    binding: i as u32 + 1,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                })
            }
            let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                entries: &textures_bind_group_entries,
                label: Some("texture_bind_group"),
            });

            render_steps.texture_binds.push(TextureBind {
                texture_bind_group,
                from,
                to: render_steps.buffer_vertices.len() as u64,
            });
        }

        render_steps
    }

    // pub fn add_vertices(
    //     &mut self,
    //     vertices: &[glam::Vec3; 4],
    //     color: &glam::Vec4,
    //     texture_coords: &[glam::Vec2; 4],
    //     tex_index: u32,
    // ) {
    //     let offset = self.quad_count as usize * 4;

    //     self.vertices[offset].position = vertices[0];
    //     self.vertices[offset + 1].position = vertices[1];
    //     self.vertices[offset + 2].position = vertices[2];
    //     self.vertices[offset + 3].position = vertices[3];

    //     self.vertices[offset].color = *color;
    //     self.vertices[offset + 1].color = *color;
    //     self.vertices[offset + 2].color = *color;
    //     self.vertices[offset + 3].color = *color;

    //     self.vertices[offset].texture_coords = texture_coords[0];
    //     self.vertices[offset + 1].texture_coords = texture_coords[1];
    //     self.vertices[offset + 2].texture_coords = texture_coords[2];
    //     self.vertices[offset + 3].texture_coords = texture_coords[3];

    //     self.vertices[offset].tex_index = tex_index;
    //     self.vertices[offset + 1].tex_index = tex_index;
    //     self.vertices[offset + 2].tex_index = tex_index;
    //     self.vertices[offset + 3].tex_index = tex_index;

    //     self.quad_count += 1;
    // }

    // pub fn add_quad(
    //     &mut self,
    //     position: glam::Vec2,
    //     texture_size: glam::Vec2,
    //     sub_tex_rect: Option<Rect>,
    //     scale: glam::Vec2,
    //     rotate: f32,
    //     color: glam::Vec4,
    //     origin: QuadOrigin,
    //     tex_index: u32,
    // ) {
    //     let offset = self.quad_count as usize * 4;
    //     let render_rect_size = if let Some(r) = sub_tex_rect {
    //         r.size().into()
    //     } else {
    //         texture_size
    //     };

    //     let quad = match origin {
    //         QuadOrigin::TopLeft => &TOP_LEFT_QUAD,
    //         QuadOrigin::Center => &CENTER_QUAD,
    //     };

    //     let transform = if rotate == 0.0 {
    //         glam::Mat4::from_translation(position.extend(0.0))
    //             * glam::Mat4::from_scale(render_rect_size.extend(0.0) * scale.extend(0.0))
    //     } else {
    //         glam::Mat4::from_scale_rotation_translation(
    //             render_rect_size.extend(0.0) * scale.extend(0.0),
    //             glam::Quat::from_rotation_z(rotate),
    //             position.extend(0.0),
    //         )
    //     };

    //     self.vertices[offset].position = (transform * quad[0]).truncate();
    //     self.vertices[offset + 1].position = (transform * quad[1]).truncate();
    //     self.vertices[offset + 2].position = (transform * quad[2]).truncate();
    //     self.vertices[offset + 3].position = (transform * quad[3]).truncate();

    //     self.vertices[offset].color = color;
    //     self.vertices[offset + 1].color = color;
    //     self.vertices[offset + 2].color = color;
    //     self.vertices[offset + 3].color = color;

    //     if let Some(rect) = sub_tex_rect {
    //         let width = texture_size.x;
    //         let height = texture_size.y;
    //         self.vertices[offset].texture_coords = glam::vec2(
    //             (rect.x + rect.width) / width,
    //             (rect.y + rect.height) / height,
    //         ); // Top Right
    //         self.vertices[offset + 1].texture_coords =
    //             glam::vec2(rect.right() / width, rect.y / height); // Bottom Right
    //         self.vertices[offset + 2].texture_coords =
    //             glam::vec2((rect.x + 0.5) / width, rect.y / height); // Bottom Left
    //         self.vertices[offset + 3].texture_coords =
    //             glam::vec2((rect.x + 0.5) / width, rect.bottom() / height); // Top Left
    //     } else {
    //         self.vertices[offset].texture_coords = glam::vec2(1.0, 1.0);
    //         self.vertices[offset + 1].texture_coords = glam::vec2(1.0, 0.0);
    //         self.vertices[offset + 2].texture_coords = glam::vec2(0.0, 0.0);
    //         self.vertices[offset + 3].texture_coords = glam::vec2(0.0, 1.0);
    //     }

    //     self.vertices[offset].tex_index = tex_index;
    //     self.vertices[offset + 1].tex_index = tex_index;
    //     self.vertices[offset + 2].tex_index = tex_index;
    //     self.vertices[offset + 3].tex_index = tex_index;

    //     self.quad_count += 1;
    // }

    // pub fn vertices(&self) -> &[Vertex] {
    //     &self.vertices[0..self.vertices_count() as usize]
    // }

    // pub fn vertices_count(&self) -> i32 {
    //     self.quad_count * 4
    // }

    // pub fn indices_count(&self) -> i32 {
    //     self.quad_count * 6
    // }

    // pub fn bind_textures(
    //     &self,
    //     texture_bind_group_layout: &wgpu::BindGroupLayout,
    //     device: &wgpu::Device,
    // ) -> wgpu::BindGroup {
    //     let mut textures_bind_group_entries = Vec::new();
    //     textures_bind_group_entries.push(wgpu::BindGroupEntry {
    //         binding: 0,
    //         resource: wgpu::BindingResource::Sampler(
    //             &self.texture_slots[0].as_ref().unwrap().sampler,
    //         ),
    //     });
    //     for (i, t) in self.texture_slots.iter().enumerate() {
    //         if let Some(texture) = t {
    //             textures_bind_group_entries.push(wgpu::BindGroupEntry {
    //                 binding: i as u32 + 1,
    //                 resource: wgpu::BindingResource::TextureView(&texture.view),
    //             })
    //         }
    //     }

    //     device.create_bind_group(&wgpu::BindGroupDescriptor {
    //         layout: &texture_bind_group_layout,
    //         entries: &textures_bind_group_entries,
    //         label: Some("diffuse_bind_group"),
    //     })
    // }
}

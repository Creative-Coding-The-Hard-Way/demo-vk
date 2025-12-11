use {
    super::Material,
    ash::vk,
    nalgebra::{Matrix4, Vector3},
    std::sync::Arc,
};

#[repr(C, align(16))]
#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub uv_x: f32,
    pub color: [f32; 4],
    pub texture_index: i32,
    pub uv_y: f32,
}

impl Vertex {
    pub fn new(
        pos: [f32; 3],
        uv: [f32; 2],
        color: [f32; 4],
        texture_index: i32,
    ) -> Self {
        Self {
            pos,
            uv_x: uv[0],
            color,
            texture_index,
            uv_y: uv[1],
        }
    }
}

/// A Mesh is the minimal unit of rendering.
///
/// Data is streamed from the CPU to the GPU each frame from each Mesh.
pub trait Mesh {
    fn vertices(&self) -> &[Vertex];
    fn indices(&self) -> &[u32];
    fn material(&self) -> &Arc<Material>;
    fn transform(&self) -> &Matrix4<f32>;
    fn scissor(&self) -> vk::Rect2D;
}

/// This mesh supports drawing arbitrary triangles and quads in three
/// dimensions.
pub struct TrianglesMesh {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    material: Arc<Material>,
    transform: Matrix4<f32>,
    scissor: vk::Rect2D,
}

impl Mesh for TrianglesMesh {
    fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    fn indices(&self) -> &[u32] {
        &self.indices
    }

    fn material(&self) -> &Arc<Material> {
        &self.material
    }

    fn transform(&self) -> &Matrix4<f32> {
        &self.transform
    }

    fn scissor(&self) -> vk::Rect2D {
        self.scissor
    }
}

impl TrianglesMesh {
    /// Creates a new empty Mesh with pre-allocated internal memory for
    /// vertex data.
    pub fn new(initial_capacity: usize, material: Arc<Material>) -> Self {
        Self {
            vertices: Vec::with_capacity(initial_capacity),
            indices: Vec::with_capacity(initial_capacity),
            material,
            transform: Matrix4::identity(),
            scissor: vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: vk::Extent2D {
                    width: 1,
                    height: 1,
                },
            },
        }
    }

    pub fn set_scissor(&mut self, rect: vk::Rect2D) {
        self.scissor = rect;
    }

    /// Set the matrix transformation matrix.
    pub fn set_transform(&mut self, projection: Matrix4<f32>) {
        self.transform = projection;
    }

    /// Clears all geometry from the Mesh while retaining any allocated memory.
    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
    }

    pub fn indexed_triangles<V, I>(&mut self, vertices: V, indices: I)
    where
        V: Iterator<Item = Vertex>,
        I: Iterator<Item = u32>,
    {
        let base_index = self.vertices.len() as u32;
        self.vertices.extend(vertices);
        self.indices.extend(indices.map(|index| base_index + index));
    }

    /// Adds a triangle to the mesh.
    ///
    /// Note: triangles must be in clockwise winding order, else they will be
    /// culled.
    pub fn triangle(
        &mut self,
        color: [f32; 4],
        texture_index: i32,
        p1: Vector3<f32>,
        p2: Vector3<f32>,
        p3: Vector3<f32>,
    ) {
        let base_index = self.vertices.len() as u32;

        self.vertices.extend_from_slice(&[
            Vertex::new(p1.data.0[0], [0.0, 0.0], color, texture_index),
            Vertex::new(p2.data.0[0], [0.0, 0.0], color, texture_index),
            Vertex::new(p3.data.0[0], [0.0, 0.0], color, texture_index),
        ]);
        self.indices.extend_from_slice(&[
            base_index,
            base_index + 1,
            base_index + 2,
        ]);
    }

    /// Adds a quad to the mesh.
    ///
    /// Note: corners should be specified in a clockwise winding order, else the
    /// triangles that make up the resulting quad may be culled.
    pub fn quad(
        &mut self,
        color: [f32; 4],
        texture_index: i32,
        top_left: Vector3<f32>,
        top_right: Vector3<f32>,
        bot_right: Vector3<f32>,
        bot_left: Vector3<f32>,
    ) {
        let base_index = self.vertices.len() as u32;

        self.vertices.extend_from_slice(&[
            Vertex::new(top_left.data.0[0], [0.0, 0.0], color, texture_index),
            Vertex::new(top_right.data.0[0], [1.0, 0.0], color, texture_index),
            Vertex::new(bot_right.data.0[0], [1.0, 1.0], color, texture_index),
            Vertex::new(bot_left.data.0[0], [0.0, 1.0], color, texture_index),
        ]);
        self.indices.extend_from_slice(&[
            // triangle 1
            base_index,
            base_index + 1,
            base_index + 2,
            // triangle 2
            base_index,
            base_index + 2,
            base_index + 3,
        ]);
    }
}

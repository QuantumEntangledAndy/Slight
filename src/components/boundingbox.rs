use amethyst::{
    core::math::{Vector2, Vector4},
    core::Transform,
    ecs::{Component, DenseVecStorage},
};

pub struct BoundingBox {
    width: f32,
    height: f32,
}

impl BoundingBox {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
        }
    }

    pub fn hit_ww_trans(&self, transform: &Transform, point: &Vector2<f32>) -> bool {
        let mat = transform.global_view_matrix();
        let point4 = Vector4::new(point[0], point[1], 0., 1.);
        let centered_point4 = mat*point4;
        let centered_point2 = Vector2::new(centered_point4[0], centered_point4[1]);
        self.hit(&centered_point2)
    }

    pub fn hit(&self, point: &Vector2<f32>) -> bool {
        ((-self.width/2.)..(self.width/2.)).contains(&point[0]) && ((-self.height/2.)..(self.height/2.)).contains(&point[1])
    }
}

impl Component for BoundingBox {
    type Storage = DenseVecStorage<Self>;
}

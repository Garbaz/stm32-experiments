use core::fmt::Debug;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::Point,
    primitives::{Line, PrimitiveStyle, StyledDrawable as _},
};
use nalgebra::{point, vector, Matrix4, Perspective3, Point3, Vector3};

pub struct Shape3D<const V: usize, const E: usize> {
    pub vertices: [Point3<f32>; V],
    pub edges: [(usize, usize); E],
}

pub const ARROW: Shape3D<6, 9> = {
    let length = 1.0;
    let head_width = 0.25;
    let head_height = 0.5;
    Shape3D {
        vertices: [
            point![0.0, 0.0, 0.0],
            point![0.0, length, 0.0],
            point![-head_width / 2.0, length - head_height, -head_width / 2.0],
            point![head_width / 2.0, length - head_height, -head_width / 2.0],
            point![head_width / 2.0, length - head_height, head_width / 2.0],
            point![-head_width / 2.0, length - head_height, head_width / 2.0],
        ],
        edges: [
            (0, 1),
            (1, 2),
            (1, 3),
            (1, 4),
            (1, 5),
            (2, 3),
            (3, 4),
            (4, 5),
            (5, 2),
        ],
    }
};

pub const CUBOID: Shape3D<8, 12> = {
    let width = 1.0;
    let depth = 1.25;
    let height = 0.125;

    Shape3D {
        vertices: [
            point![-width / 2.0, -height / 2.0, -depth / 2.0],
            point![width / 2.0, -height / 2.0, -depth / 2.0],
            point![width / 2.0, -height / 2.0, depth / 2.0],
            point![-width / 2.0, -height / 2.0, depth / 2.0],
            point![-width / 2.0, height / 2.0, -depth / 2.0],
            point![width / 2.0, height / 2.0, -depth / 2.0],
            point![width / 2.0, height / 2.0, depth / 2.0],
            point![-width / 2.0, height / 2.0, depth / 2.0],
        ],
        edges: [
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 0),
            (4, 5),
            (5, 6),
            (6, 7),
            (7, 4),
            (0, 4),
            (1, 5),
            (2, 6),
            (3, 7),
        ],
    }
};

impl<const V: usize, const E: usize> Shape3D<V, E> {
    pub fn draw<D>(
        &self,
        display: &mut D,
        line_style: PrimitiveStyle<D::Color>,
        position: &Point3<f32>,
        face_towards: &Point3<f32>,
    ) where
        D: DrawTarget,
        D::Error: Debug,
    {
        let model = Matrix4::face_towards(position, face_towards, &Vector3::y())
            * Matrix4::from_axis_angle(&Vector3::x_axis(), core::f32::consts::FRAC_PI_2);

        let view = Matrix4::look_at_rh(
            &Point3::new(0.0, 0.0, -2.0),
            &Point3::new(0.0, 0.0, 0.0),
            &Vector3::y(),
        );
        let proj = Perspective3::new(2.0, 60f32.to_radians(), 0.01, 100.0).to_homogeneous();
        // let proj = Orthographic3::new(-2.0, 2.0, -1.0, 1.0, 0.01, 100.0).to_homogeneous();
        let screen = Matrix4::new_nonuniform_scaling(&vector![128.0 / 2.0, 64.0 / 2.0, 1.0])
            .prepend_translation(&vector![1.0, 1.0, 0.0]);

        let spvm = screen * proj * view * model;

        let transform = |p: Point3<f32>| -> Point {
            let pss = spvm.transform_point(&p);
            Point::new(pss.x as i32, pss.y as i32)
        };

        for (f, t) in self.edges {
            let vf = transform(self.vertices[f]);
            let vt = transform(self.vertices[t]);

            Line::new(vf, vt).draw_styled(&line_style, display).unwrap();
        }
    }
}

use cgmath::prelude::*;
use renderer::{self, instance};

fn main() {
    let mut rndr = renderer::renderer::WindowRenderer::new();

    const SPACE_BETWEEN: f32 = 5.0;
    let instances = (0..100)
        .flat_map(|i| {
            (0..100).filter_map(move |j| {
                if i == j {
                    return None;
                }
                let x = SPACE_BETWEEN * (i as f32 - 10 as f32 / 2.0);
                let z = SPACE_BETWEEN * (j as f32 - 10 as f32 / 2.0);

                let position = cgmath::Vector3 { x, y: 0.0, z };

                let rotation = if position.is_zero() {
                    cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0))
                } else {
                    cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(45.0))
                };

                Some(instance::Instance { position, rotation })
            })
        })
        .collect::<Vec<_>>();

    rndr.rendr
        .as_ref()
        .unwrap()
        .borrow_mut()
        .push_instances_model("/res/cube.obj", instances);

    rndr.run();
}

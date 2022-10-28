use std::ops::Add;
use super::{model, resources, transform};

pub fn push_actor(actors: &mut Vec<actor::Actor>, m: model::Model, transf: transform::Transform) {
    actors.push(actor::Actor { transform: transf, model: m });
}

pub fn push_model(models: &mut Vec<model::Model>, obj_path: &str, color: Option<[f32; 4]>) {
    let m = load_model(obj_path, color);

    models.push(m);
}

pub fn load_model(obj_path: &str, color: Option<[f32; 4]>) -> model::Model {
    log::debug!("loading model");

    resources::load_model(&path_with_out_dir(obj_path), color).unwrap()
}

fn path_with_out_dir(obj_path: &str) -> String {
    env!("OUT_DIR").to_string().add(obj_path)
}

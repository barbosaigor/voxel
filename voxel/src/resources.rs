use super::model;
use std::io::{BufReader, Cursor};

pub fn load_model(
    file_name: &str,
    color: Option<[f32; 4]>,
) -> anyhow::Result<model::Model> {
    let obj_text = load_string(file_name)?;
    let obj_cursor = Cursor::new(obj_text);
    let mut obj_reader = BufReader::new(obj_cursor);

    let (models, _) = tobj::load_obj_buf(
        &mut obj_reader,
        &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
        |p| {
            let mat_text = load_string(p.to_str().unwrap()).unwrap();
            tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(mat_text)))
        },
    )?;

    let mesh = {
        let m = &models[0];
        // iterate of array with x,y,z vertice data aside
        let vertices = (0..m.mesh.positions.len() / 3)
            .map(|i| model::MeshVertex {
                position: [
                    m.mesh.positions[i * 3],
                    m.mesh.positions[i * 3 + 1],
                    m.mesh.positions[i * 3 + 2],
                ],
                color: color.unwrap_or_default(),
            })
            .collect::<Vec<_>>();

        model::Mesh {
            id: file_name.to_string(),
            buffers: None,
            indices: m.mesh.indices.clone(),
            vertices,
        }
    };

    Ok(model::Model { mesh, color })
}

pub fn load_string(file_name: &str) -> anyhow::Result<String> {
    let p = env!("OUT_DIR");
    let path = std::path::Path::new(p).join("res").join(file_name);
    let txt = std::fs::read_to_string(path)?;

    Ok(txt)
}

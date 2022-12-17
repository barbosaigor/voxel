mod scene;

fn main() {
    let game = scene::Scene::new();
    voxel::run(game);
}

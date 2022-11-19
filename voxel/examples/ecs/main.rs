mod scene;

fn main() {
    let game = scene::Scene::new();
    let app = voxel::app::App::new(game);
    app.run();
}

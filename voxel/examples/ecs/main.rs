mod scene;

fn main() {
    let game = Box::new(scene::Scene::new());
    let app = voxel::app::App::new(game);
    app.run();
}

struct Query;

//

struct Res<R: Resource> {

}

trait Resource : Send + Sync { }

#[derive(Resuorce)]
struct AssetManager {
    requests: Vec<String>,
}

impl AssetManager {
    pub fn request(id: String) {
        requests.push(id);
    }
}

//

fn start_player_asset_loading(
    asset_manager: Res<AssetManager>,
) {
    asset_manager.request
}
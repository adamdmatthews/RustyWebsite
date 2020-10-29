mod endpoint;
mod root;
use endpoint::Endpoint;

use rocket::Rocket;

fn get_endpoints() -> Vec<Endpoint> {
    let root = root::get_routes();
    vec![root]
}

fn mount_endpoint(rocket: Rocket, endpoint: &Endpoint) -> Rocket {
    let path = &endpoint.path;
    let routes = endpoint.routes.clone();
    rocket.mount(path, routes)
}

pub fn mount_endpoints(rocket: Rocket) -> Rocket {
    get_endpoints().iter().fold(rocket, mount_endpoint)
}

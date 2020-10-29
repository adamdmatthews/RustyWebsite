use rocket::Route;

pub struct Endpoint {
    pub path: String,
    pub routes: Vec<Route>,
}

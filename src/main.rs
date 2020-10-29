#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod api;

fn main() {
    let mut rocket = rocket::ignite();
    rocket = api::mount_endpoints(rocket);
    rocket.launch();
}

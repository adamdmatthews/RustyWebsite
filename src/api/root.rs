use super::Endpoint;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub fn get_routes() -> Endpoint {
    Endpoint {
        path: String::from("/"),
        routes: routes![index],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world() {
        let expected = "Hello, world!";
        let actual = index();
        assert_eq!(expected, actual)
    }
}

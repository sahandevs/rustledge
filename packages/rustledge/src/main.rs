use gotham::router::Router;
use gotham::router::builder::*;
use gotham::state::State;

const HELLO_WORLD: &'static str = "Hello World!";


pub fn search(state: State) -> (State, &'static str) {
    (state, HELLO_WORLD)
}

fn router() -> Router {
    build_simple_router(|route| {
        route.post("/search").to(search);
    })
}

pub fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router());
    println!("test");
}

#[cfg(test)]
mod tests {
    use super::*;
    use gotham::hyper::StatusCode;
    use gotham::test::TestServer;
    use mime;

    #[test]
    fn receive_hello_world_response() {
        let test_server = TestServer::new(router()).unwrap();
        let response = test_server
            .client()
            .post("http://localhost/search", "test", mime::TEXT_HTML_UTF_8)
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.read_body().unwrap();
        assert_eq!(&body[..], b"Hello World!");
    }
}
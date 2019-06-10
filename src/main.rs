#[macro_use]
extern crate rouille;

use rouille::Request;
use rouille::Response;

fn main() {
    rouille::start_server("127.0.0.1:4545", move |request| {
        handle_request(&request)
    })
}

fn handle_request(request: &Request) -> Response {
    rouille::router!(request,
        (GET) (/v1/get_all_stat_for_month/year/{ year: i32 }/month/{ month: i32 }) => {
            Response::text("Json should be here, but not now.")
        },
        _ => Response::empty_404()
    )
}

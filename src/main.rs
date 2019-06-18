extern crate postgres;
#[macro_use]
extern crate rouille;

use std::collections::HashMap;
use std::sync::Mutex;

use postgres::Connection;
use postgres::TlsMode;
use postgres::transaction::Transaction;
use rouille::Request;
use rouille::Response;

//example https://github.com/tomaka/rouille/blob/master/examples/database.rs

fn main() {
    let guarded_connection = {
        let db = Connection::connect(
            "postgres://time_fix_stat_user:time_fix_stat@localhost:5432/time_fix_stat",
            TlsMode::None,
        );
        Mutex::new(db.expect("Failed to connect to database"))
    };

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

const SELECT: &str = "SELECT activity_name, day, minute FROM time_fix_stat.time_fix_stat \
                        WHERE user_id = $1";

fn construct_stat(conn: &Connection, user_id: u32) -> HashMap<String, HashMap<u8, u16>> {
    let mut result: HashMap<String, HashMap<u8, u16>> = HashMap::new();
    for row in conn.query(SELECT, &[user_id]).expect("Can't execute select") {
        let activity_name: String = row.get(0);
        let entry = result.entry(activity_name);
    }

    result
}

extern crate postgres;
//#[macro_use]
//extern crate rouille;

//use std::collections::HashMap;
//use std::sync::Mutex;

//use postgres::transaction::Transaction;
//use rouille::Request;
//use rouille::Response;
use std::collections::HashMap;

use postgres::{Connection, Error};
use postgres::TlsMode;

//example https://github.com/tomaka/rouille/blob/master/examples/database.rs

//fn main() {
//    let guarded_connection = {
//        let db = Connection::connect(
//            "postgres://time_fix_stat_user:time_fix_stat@localhost:5432/time_fix_stat",
//            TlsMode::None,
//        );
//        Mutex::new(db.expect("Failed to connect to database"))
//    };
//
//    rouille::start_server("127.0.0.1:4545", move |request| {
//        handle_request(&request)
//    })
//}

//fn handle_request(request: &Request) -> Response {
//    rouille::router!(request,
//        (GET) (/v1/get_all_stat_for_month/year/{ year: i32 }/month/{ month: i32 }) => {
//            Response::text("Json should be here, but not now.")
//        },
//        _ => Response::empty_404()
//    )
//}

fn main() {
    let conn = Connection::connect("postgres://time_fix_stat_user:time_fix_stat@localhost:5432/time_fix_stat", TlsMode::None)
        .unwrap();

    let result = construct_stat(&conn, 1, 1);
    println!("{}", result.len());
    for (action, day_to_minute) in result {
        println!("{}", day_to_minute.len());
        for (day, minute) in day_to_minute {
            println!("{} {} {}", action, day, minute);
        };
    };
}

const SELECT: &str = "SELECT activity_name, EXTRACT(DAY from day)::INTEGER, minute FROM time_fix_stat.time_fix_stat \
                        WHERE user_id = $1 and EXTRACT(MONTH from day)::INTEGER = $2";

fn construct_stat(conn: &Connection, user_id: i32, month: i32) -> HashMap<String, HashMap<i32, i32>> {
    let mut result: HashMap<String, HashMap<i32, i32>> = HashMap::new();

    for row in conn.query(SELECT, &[&user_id, &month]).expect("Can't execute select").iter() {
        let day_to_minutes = result
            .entry(row.get(0))
            .or_insert(HashMap::new());
        day_to_minutes.insert(row.get(1), row.get(2));
    };

    result
}

// Populate a set of dummy download statistics for a specific version in the
// database.
//
// Usage:
//      cargo run --bin populate version_id1 version_id2 ...

#![deny(warnings)]

extern crate cargo_registry;
extern crate diesel;
extern crate rand;

use diesel::prelude::*;
use rand::{Rng, StdRng};
use std::env;

use cargo_registry::schema::version_downloads;

fn main() {
    let conn = cargo_registry::db::connect_now().unwrap();
    conn.transaction(|| update(&conn)).unwrap();
}

fn update(conn: &PgConnection) -> QueryResult<()> {
    use diesel::dsl::*;

    let ids = env::args()
        .skip(1)
        .filter_map(|arg| arg.parse::<i32>().ok());
    for id in ids {
        let mut rng = StdRng::new().unwrap();
        let mut dls = rng.gen_range(5000i32, 10000);

        for day in 0..90 {
            dls += rng.gen_range(-100, 100);

            diesel::insert_into(version_downloads::table)
                .values((
                    version_downloads::version_id.eq(id),
                    version_downloads::downloads.eq(dls),
                    version_downloads::date.eq(date(now - day.days())),
                ))
                .execute(conn)?;
        }
    }
    Ok(())
}

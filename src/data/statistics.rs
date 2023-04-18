mod schema {
    diesel::table! {
        statistics(id) {
            id ->  Nullable<Integer>,
            action_id ->  Integer,
            count ->  Integer,
            create_at ->  Nullable<BigInt>,
            update_at ->  Nullable<BigInt>,
        }
    }
}

use diesel::{self, result::QueryResult, prelude::*};
use diesel::query_builder::QueryFragment;

use self::schema::statistics;
use rocket::serde::{Serialize, Deserialize};
use crate::NotesConnection;
use crate::utils::get_file_list::get_epoch_ms;

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = statistics)]
pub struct Statistics {
    pub id: Option<i32>,
    pub action_id: i32,
    pub count: i32,
    #[serde(skip_deserializing, skip_serializing)]
    pub create_at: Option<i64>,
    #[serde(skip_deserializing, skip_serializing)]
    pub update_at: Option<i64>,
}

impl Statistics {
    pub async fn all(conn: &NotesConnection) -> QueryResult<Vec<Statistics>> {
        conn.run(|c| {
            statistics::table.load::<Statistics>(c)
        }).await
    }
    pub async fn update(v: i32, conn: &NotesConnection) -> QueryResult<usize> {
        conn.run(move |c| {
            let s = statistics::table.filter(statistics::action_id.eq(&v)).get_result::<Statistics>(c).unwrap();

            let size = (get_epoch_ms() / 1000) as i64;
            let updated_statistics = diesel::update(statistics::table.filter(statistics::action_id.eq(&v)));
            updated_statistics.set((
                statistics::count.eq(&(s.count + 1)),
                statistics::update_at.eq(&size))
            ).execute(c)
        }).await
    }
    pub async fn insert(action_id: i32, conn: &NotesConnection) -> QueryResult<usize> {

        conn.run(move  |c| {

            let t = Statistics {
                id: None,
                action_id,
                count: 0,
                create_at: Some((get_epoch_ms() / 1000) as i64),
                update_at: Some((get_epoch_ms() / 1000) as i64),
            };
            //println!("{:?}", diesel::insert_into(statistics::table).values(&t));
            diesel::insert_into(statistics::table).values(&t).execute(c)
        }).await
    }
}

use crate::NotesConnection;
use rocket::http::Status;
use rocket::serde::json::serde_json;
use rocket::serde::Serialize;
use diesel::{self, result::QueryResult, prelude::*};

mod schema {
    diesel::table! {
        notes(_id) {
            _id ->  Nullable<Integer>,
            title ->  Text,
            content ->  Nullable<Text>,
            create_at ->  BigInt,
            update_at ->  BigInt,
        }
    }
}

use self::schema::notes;
use self::schema::notes::dsl::{notes as all_notes};

#[derive(Serialize, Queryable, Insertable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
#[table_name = "notes"]
pub struct Notes {
    pub _id: Option<i32>,
    pub title: String,
    pub content: Option<String>,
    pub create_at: i64,
    pub update_at: i64,
}

impl Notes {
    pub async fn all(conn: &NotesConnection) -> QueryResult<Vec<Notes>> {
        conn.run(|c| {
            notes::table
                .select((
                    notes::_id,
                    notes::title,
                    notes::update_at,
                ))
                .order(notes::update_at.desc()).get_result(c)
        }).await
    }
}

#[get("/api/notes")]
pub async fn get_notes(conn: NotesConnection) -> Result<String, Status> {
    match Notes::all(&conn).await {
        Ok(v) => {
            Ok(serde_json::to_string::<Vec<Notes>>(&v).unwrap())
        }
        Err(e) => {
            Err(Status::InternalServerError)
        }
    }
    // conn.0
    //     .query_row(
    //         "SELECT COUNT(*) FROM game_ratings WHERE timestamp > ?
    //                 AND (value_a < 1200 OR value_b < 1300)",
    //         params!["1"],
    //         |r| r.get(0),
    //     )
    //     .unwrap();
}
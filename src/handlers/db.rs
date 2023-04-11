use std::path::Path;
use crate::NotesConnection;
use rocket::http::Status;
use rocket::serde::json::serde_json;
use rocket::serde::{Serialize, Deserialize};
use diesel::{self, result::QueryResult, prelude::*};
use std::time::{SystemTime, UNIX_EPOCH};

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
    diesel::table! {
        snippet {
            id ->  Nullable<Integer>,
            prefix ->  Text,
            body ->  Text,
            language ->  Nullable<Text>,
            count ->  Nullable<Integer>,
            create_at ->  Integer,
            update_at ->  Integer,
        }
    }
}

use self::schema::notes;
use self::schema::notes::dsl::{notes as all_notes};
use self::schema::snippet;
use self::schema::snippet::dsl::{snippet as all_snippets};
use diesel::prelude::*;
use rocket::data::FromData;
use rocket::form::Form;
use crate::seek_stream::SeekStream;

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

#[derive(Serialize, Queryable)]
pub struct Note {
    pub _id: Option<i32>,
    pub title: String,
    pub update_at: i64,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
#[table_name = "snippet"]
pub struct Snippet {
    pub id: Option<i32>,
    pub prefix: String,
    pub body: String,
    pub language: Option<String>,
    #[serde(skip_deserializing, skip_serializing)]
    pub count: Option<i32>,
    #[serde(skip_deserializing, skip_serializing)]
    pub create_at: i32,
    #[serde(skip_deserializing, skip_serializing)]
    pub update_at: i32,
}

impl Notes {
    pub async fn all(conn: &NotesConnection) -> QueryResult<Vec<Note>> {
        conn.run(|c| {
            notes::table
                .select((notes::_id, notes::title, notes::update_at))
                .order(notes::update_at.desc()).load::<Note>(c)
        }).await
    }
}

fn get_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

impl Snippet {
    pub async fn all(conn: &NotesConnection) -> QueryResult<Vec<String>> {
        conn.run(|c| {
            snippet::table
                .select(snippet::prefix)
                .order(snippet::update_at.desc()).load::<String>(c)
        }).await
    }
    pub async fn insert(snippet: Snippet, conn: &NotesConnection) -> QueryResult<usize> {
        conn.run(|c| {
            let t = Snippet {
                id: None,
                prefix: snippet.prefix,
                body: snippet.body,
                language: snippet.language,
                count: Some(0),
                create_at: (get_epoch_ms() / 1000) as i32,
                update_at: (get_epoch_ms() / 1000) as i32,
            };
            diesel::insert_into(snippet::table).values(&t).execute(c)
        }).await
    }
    pub async fn delete_with_id(id: i32, conn: &NotesConnection) -> QueryResult<usize> {
        conn.run(move |c| diesel::delete(snippet::table)
            .filter(snippet::id.eq(&id))
            .execute(c))
            .await
    }
    pub async fn query_body(prefix: String, conn: &NotesConnection) -> QueryResult<String> {
        conn.run(move |c| {
            let v = snippet::table.filter(snippet::prefix.eq(&prefix)).get_result::<Snippet>(c);
            return match v {
                Ok(v) => {
                    let updated_snippet = diesel::update(snippet::table.filter(snippet::id.eq(v.id)));
                    updated_snippet.set(snippet::count.eq(v.count.unwrap_or(0) + 1)).execute(c);
                    Ok(v.body)
                }
                Err(err) => {
                    Ok(String::new())
                }
            };
        }).await
    }
}

#[get("/api/notes")]
pub async fn get_notes(conn: NotesConnection) -> Result<String, Status> {
    match Notes::all(&conn).await {
        Ok(v) => {
            Ok(serde_json::to_string::<Vec<Note>>(&v).unwrap())
        }
        Err(e) => {
            Err(Status::InternalServerError)
        }
    }
}

#[get("/api/snippet?<prefix..>")]
pub async fn get_snippet(prefix: Option<String>, conn: NotesConnection) -> Result<String, Status> {
    match prefix {
        None => {
            match Snippet::all(&conn).await {
                Ok(v) => {
                    Ok(serde_json::to_string::<Vec<String>>(&v).unwrap())
                }
                Err(e) => {
                    Err(Status::InternalServerError)
                }
            }
        }
        Some(v) => {
            match Snippet::query_body(v, &conn).await {
                Ok(v) => {
                    Ok(v)
                }
                Err(e) => {
                    Err(Status::InternalServerError)
                }
            }
        }
    }
}

#[post("/api/snippet/insert", data = "<snippet_form>")]
pub async fn insert_snippet(snippet_form: String, conn: NotesConnection) -> Result<String, Status> {
    match serde_json::from_str::<Snippet>(&snippet_form) {
        Ok(v) => {
            if let Err(e) = Snippet::insert(v, &conn).await {
                println!("{}", e);
                return Err(Status::InternalServerError);
            }
        }
        Err(e) => {
            println!("{}", e);
            return Err(Status::InternalServerError);
        }
    }
    Ok("Success".to_string())
}

#[get("/api/snippet/delete?<id>")]
pub async fn delete_snippet(id: i32, conn: NotesConnection) -> Result<String, Status> {
    if let Err(e) = Snippet::delete_with_id(id, &conn).await {
        println!("{}", e);
        return Err(Status::InternalServerError);
    }
    Ok("Success".to_string())
}

#[get("/notes/notes")]
pub fn get_notes_page<'a>() -> std::io::Result<SeekStream<'a>> {
    let p = Path::new("assets/notes/notes.html");
    SeekStream::from_path(p)
}
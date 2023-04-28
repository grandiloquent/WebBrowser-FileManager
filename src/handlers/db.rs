use std::path::Path;
use crate::NotesConnection;
use rocket::http::Status;
use rocket::serde::json::serde_json;
use rocket::serde::{Serialize, Deserialize};
use diesel::{self, result::QueryResult, prelude::*};
use diesel::prelude::*;
use regex::Regex;
use rocket::data::FromData;
use rocket::form::Form;
use crate::data::notes::{Note, NoteContent, Notes};
use crate::data::snippet::Snippet;
use crate::data::statistics::Statistics;
use crate::seek_stream::SeekStream;

#[get("/api/note?<id..>")]
pub async fn get_notes(id: Option<i32>, conn: NotesConnection) -> Result<String, Status> {
    match id {
        None => {
            match Notes::all(&conn).await {
                Ok(v) => {
                    Ok(serde_json::to_string::<Vec<Note>>(&v).unwrap())
                }
                Err(e) => {
                    Err(Status::InternalServerError)
                }
            }
        }
        Some(v) => {
            match Notes::query_content(v, &conn).await {
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

#[get("/api/note/search?<q>")]
pub async fn search_notes(q: String, conn: NotesConnection) -> Result<String, Status> {
    match Notes::search(q, &conn).await {
        Ok(v) => {
            Ok(serde_json::to_string::<Vec<Note>>(&v).unwrap())
        }
        Err(e) => {
            Err(Status::InternalServerError)
        }
    }
}

#[get("/api/note/like?<q>")]
pub async fn like_notes(q: String, conn: NotesConnection) -> Result<String, Status> {
    match Notes::like(&conn).await {
        Ok(v) => {
            let regex = Regex::new(&q).unwrap();
            Ok(serde_json::to_string::<Vec<Note>>(&(v.iter()
                .filter(|c| {
                    match regex.find(&c.content) {
                        Some(v) => true,
                        None => false
                    }
                }).map(|c| {
                Note {
                    _id: c._id,
                    title: c.title.clone(),
                    update_at:c.update_at,
                }
            }).collect())).unwrap())
        }
        Err(e) => {
            Err(Status::InternalServerError)
        }
    }
}

#[post("/api/note/insert?<id..>", data = "<note_form>")]
pub async fn insert_note(id: Option<i32>, note_form: String, conn: NotesConnection) -> Result<String, Status> {
    match id {
        None => {
            match serde_json::from_str::<Notes>(&note_form) {
                Ok(v) => {
                    if let Err(e) = Notes::insert(v, &conn).await {
                        println!("{}", e);
                        return Err(Status::InternalServerError);
                    }
                }
                Err(e) => {
                    println!("{}", e);
                    return Err(Status::InternalServerError);
                }
            }
        }
        Some(_id) => {
            match serde_json::from_str::<Notes>(&note_form) {
                Ok(mut v) => {
                    v._id = Some(_id);
                    Notes::update(v, &conn).await;
                }
                Err(e) => {
                    println!("{}", e);
                    return Err(Status::InternalServerError);
                }
            }
        }
    }
    Ok("Success".to_string())
}

#[post("/api/note/append?<id>", data = "<v>")]
pub async fn append_note(id: i32, v: String, conn: NotesConnection) -> Result<String, Status> {
    if let Err(e) = Notes::append_content(id, v, &conn).await {
        return Err(Status::InternalServerError);
    }
    Ok("Success".to_string())
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

#[post("/api/snippet/insert?<id..>", data = "<snippet_form>")]
pub async fn insert_snippet(id: Option<i32>, snippet_form: String, conn: NotesConnection) -> Result<String, Status> {
    match id {
        None => {
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
        }
        Some(_) => {}
    }
    Ok("Success".to_string())
}

#[get("/api/snippet/delete?<prefix>")]
pub async fn delete_snippet(prefix: String, conn: NotesConnection) -> Result<String, Status> {
    if let Err(e) = Snippet::delete_with_prefix(prefix, &conn).await {
        println!("{}", e);
        return Err(Status::InternalServerError);
    }
    Ok("Success".to_string())
}

#[get("/api/statistics?<id..>")]
pub async fn get_statistics(id: Option<i32>, conn: NotesConnection) -> Result<String, Status> {
    match id {
        None => {
            match Statistics::all(&conn).await {
                Ok(v) => {
                    Ok(serde_json::to_string(&v).unwrap())
                }
                Err(e) => {
                    Err(Status::InternalServerError)
                }
            }
        }
        Some(v) => {
            match Statistics::update(v, &conn).await {
                Ok(v) => {
                    Ok(serde_json::to_string(&v).unwrap())
                }
                Err(e) => {
                    Err(Status::InternalServerError)
                }
            }
        }
    }
}

#[get("/api/statistics/insert?<id>")]
pub async fn insert_statistics(id: i32, conn: NotesConnection) -> Result<String, Status> {
    match Statistics::insert(id, &conn).await {
        Ok(v) => {
            Ok(serde_json::to_string(&v).unwrap())
        }
        Err(e) => {
            Err(Status::InternalServerError)
        }
    }
}
use rocket::fs::NamedFile;

#[get("/")]
pub async fn index() -> Option<NamedFile> {
    NamedFile::open("/index/index.html").await.ok()
}
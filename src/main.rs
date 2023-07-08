use rocket::http::uri::Origin;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::json::{json, Value};

#[macro_use]
extern crate rocket;

const TAURI_RELEASES_PREFIX: Origin<'static> = uri!("/tauri-releases");

#[get("/")]
fn index() -> Redirect {
    let msg: Option<&str> = None;
    Redirect::to(uri!(
        TAURI_RELEASES_PREFIX,
        google_keep_desktop_api("windows-x86_64", "v1.0.14", msg)
    ))
}

#[get("/google-keep-desktop/<_platform>/<_current_version>?<_msg>")] //http://127.0.0.1:8000/tauri-relases/google_keep_desktop/win_64/1.18.0
fn google_keep_desktop_api(
    _platform: &str,
    _current_version: &str,
    _msg: Option<&str>,
) -> Result<Value, Status> {
    //Status::NoContent
    if let Some(_msg) = _msg {
        //println!("{msg}");
        return Err(Status::NoContent);
    }

    Ok(json!({
        "notes": "IT WORKS"
    }))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount(TAURI_RELEASES_PREFIX, routes![google_keep_desktop_api])
}

use actix_web::{
    AsyncResponder, FromRequest, FutureResponse, HttpMessage, HttpRequest, HttpResponse, Json,
    Path, Result, State,
};
use diesel::prelude::*;
use futures::future::Future;

use models::{establish_connection, AuthReply, AuthRequest, Store, User};
use schema::users;

pub fn signup(_state: State<Store>, json: Json<AuthRequest>) -> Result<Json<AuthRequest>> {
    Ok(json)
}

pub fn get_users(req: &HttpRequest<Store>) -> Result<Json<Vec<User>>> {
    let conn = &req.state().db.get().expect("数据库连接失败");
    let u: Vec<User> = users::table.load(conn).expect("Error loading users");
    Ok(Json(u))
}

pub fn user_info(_state: State<Store>, path: Path<AuthRequest>) -> Result<Json<AuthRequest>> {
    // let a = Json::<AuthRequest>::extract(path).expect("解析错误");
    // Ok(Json(a))
    Ok(Json(AuthRequest {
        username: path.username.clone(),
        password: path.password.clone(),
    }))
}

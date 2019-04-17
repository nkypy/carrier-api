use actix_web::{
    AsyncResponder, FromRequest, FutureResponse, HttpMessage, HttpRequest, HttpResponse, Json,
    Path, State,
};



use error::{Error};
use models::{AppReply, AuthReply, AuthRequest, InfoReply, Store};


pub fn signin(_state: State<Store>, json: Json<AuthRequest>) -> Result<Json<AuthReply>, Error> {
    if let Json(_h) = json {
        return Ok(Json(AuthReply {
            error_code: None,
            error_message: None,
            token: None,
            data: None,
        }));
    }
    return Err(Error::TokenIsNotValid);
}

pub fn signup(_state: State<Store>, json: Json<AuthRequest>) -> Result<Json<AuthRequest>, Error> {
    Ok(json)
}

// pub fn get_users(req: &HttpRequest<Store>) -> Result<Json<Vec<User>>, Error> {
//     let conn = &req.state().db.get().expect("数据库连接失败");
//     let u: Vec<User> = users::table.load(conn).expect("Error loading users");
//     Ok(Json(u))
// }

pub fn get_users(_req: &HttpRequest<Store>) -> Result<Json<AppReply>, Error> {
    // let conn = &req.state().db.get().expect("数据库连接失败");
    // let u: Vec<User> = users::table.load(conn).expect("Error loading users");
    let reply = AppReply {
        data: vec![
            Ok(InfoReply {
                error_code: Some(1),
                error_message: None,
            }),
            Err(Error::TokenIsNotValid),
        ],
    };
    Ok(Json(reply))
}

pub fn user_info(
    _state: State<Store>,
    path: Path<AuthRequest>,
) -> Result<Json<AuthRequest>, Error> {
    // let a = Json::<AuthRequest>::extract(path).expect("解析错误");
    // // Ok(Json(a))
    // if let Path<h> = path {
    //     return Ok(Json(h));
    // }
    // return Err(Error::TokenIsNotValid);
    Ok(Json(AuthRequest {
        username: path.username.clone(),
        password: path.password.clone(),
    }))
}

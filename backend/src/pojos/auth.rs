use actix_web::actix::Message;
use diesel::sql_types::{Array, Integer, Record, VarChar};

use commons::Result;

use std::collections::HashMap;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub remember: bool,
    pub nickname: String,
    pub password: String,
}

impl Message for LoginRequest {
    type Result = Result<AuthInfo>;
}

#[derive(Deserialize, Serialize)]
pub struct Privileges {
    pub roles: Vec<String>,
    pub permissions: HashMap<String, i32>,
}

#[derive(Deserialize, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub privileges: Privileges,
}

#[derive(Queryable, QueryableByName, Deserialize, Serialize, Debug)]
pub struct AuthInfo {
    #[sql_type = "Integer"]
    pub id: i32,
    #[sql_type = "Array<VarChar>"]
    pub roles: Vec<String>,
    #[sql_type = "Array<Record<(VarChar,Integer)>>"]
    pub permissions: Vec<(String, i32)>,
}

#[derive(Deserialize, Insertable)]
#[table_name = "immortal_users"]
pub struct RegisterRequest {
    pub nickname: String,
    pub password: String,
    pub email: String,
    pub sex: i32,
}

impl Message for RegisterRequest {
    type Result = Result<()>;
}
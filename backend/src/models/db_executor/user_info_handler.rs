use actix_web::{actix::Handler, error, Result};
use diesel::prelude::*;

use crate::models::{DBExecutor, GetUser, ImmortalUser, schema};

impl Handler<GetUser> for DBExecutor {
    type Result = Result<Vec<ImmortalUser>>;
    fn handle(&mut self, get_user: GetUser, _: &mut Self::Context) -> Self::Result {
        use schema::immortal_user::dsl::*;
        let query_phone = get_user.phone.unwrap_or_default();
        let pattern = format!("%{}%", query_phone);
        let connection: &PgConnection = &self.0.get().unwrap();
        immortal_user
            .filter(phone.like(pattern))
            .limit(5)
            .load::<ImmortalUser>(connection)
            .map_err(error::ErrorInternalServerError)
    }
}
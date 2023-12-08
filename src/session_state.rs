use actix_session::{Session, SessionExt};
use actix_web::FromRequest;
use anyhow::Context;
use std::future::{ready, Ready};
use uuid::Uuid;

pub struct TypedSession(Session);

impl TypedSession {
    const USER_ID_KEY: &'static str = "user_id";

    pub fn renew(&self) {
        self.0.renew();
    }

    pub fn insert_user_id(&self, user_id: Uuid) -> Result<(), anyhow::Error> {
        self.0
            .insert(Self::USER_ID_KEY, user_id)
            .context("session insert error")
    }

    pub fn get_user(&self) -> Result<Option<Uuid>, anyhow::Error> {
        self.0
            .get(Self::USER_ID_KEY)
            .context("session get user error")
    }

    pub fn logout(&self) {
        self.0.purge()
    }
}

impl FromRequest for TypedSession {
    type Error = <Session as FromRequest>::Error;

    type Future = Ready<Result<TypedSession, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_http::Payload) -> Self::Future {
        ready(Ok(TypedSession(req.get_session())))
    }
}

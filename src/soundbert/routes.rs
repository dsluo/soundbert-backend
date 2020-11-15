use actix_web::web;

mod guilds {
    use actix_web::{delete, get, patch, post, web, Responder};

    use crate::soundbert::Snowflake;

    #[get("/{gid}")]
    pub(crate) async fn find(gid: web::Path<Snowflake>) -> impl Responder {
        unimplemented!();
    }

    #[post("/{gid}")]
    pub(crate) async fn create(gid: web::Path<Snowflake>) -> impl Responder {
        unimplemented!();
    }

    #[patch("/{gid}")]
    pub(crate) async fn update(gid: web::Path<Snowflake>) -> impl Responder {
        unimplemented!();
    }

    #[delete("/{gid}")]
    pub(crate) async fn delete(gid: web::Path<Snowflake>) -> impl Responder {
        unimplemented!();
    }
}

mod sounds {
    use actix_web::{delete, get, patch, post, web, Responder};

    use crate::soundbert::Snowflake;

    #[get("/{gid}/sounds/{sid}")]
    pub(crate) async fn find(gid: web::Path<Snowflake>, sid: web::Path<String>) -> impl Responder {
        unimplemented!();
    }

    #[get("/{gid}/sounds")]
    pub(crate) async fn find_all(gid: web::Path<Snowflake>) -> impl Responder {
        unimplemented!();
    }

    #[post("/{gid}/sounds")]
    pub(crate) async fn create(gid: web::Path<Snowflake>) -> impl Responder {
        unimplemented!();
    }

    #[patch("/{gid}/sounds/{sid}")]
    pub(crate) async fn update(
        gid: web::Path<Snowflake>,
        sid: web::Path<String>,
    ) -> impl Responder {
        unimplemented!();
    }

    #[delete("/{gid}/sounds/{sid}")]
    pub(crate) async fn delete(
        gid: web::Path<Snowflake>,
        sid: web::Path<String>,
    ) -> impl Responder {
        unimplemented!();
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(guilds::find);
    cfg.service(guilds::create);
    cfg.service(guilds::update);
    cfg.service(guilds::delete);

    cfg.service(sounds::find);
    cfg.service(sounds::create);
    cfg.service(sounds::update);
    cfg.service(sounds::delete);
}

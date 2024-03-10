use actix_web::body::MessageBody;
use actix_web::{get, middleware::Logger, post, web, App, Error, HttpResponse, HttpServer};
use actix_web_lab::middleware::{from_fn, Next};

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::result::Error::NotFound;
use dotenv::dotenv;

mod error;
mod models;
mod schema;

use error::{CustomError, DbError};

fn connect_db() -> Pool<ConnectionManager<SqliteConnection>> {
    let url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    let manager = ConnectionManager::<SqliteConnection>::new(url);

    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("error")
}

fn create_ban(
    connection: &mut SqliteConnection,
    json: web::Json<models::NewBanJSON>,
) -> Result<(), DbError> {
    use schema::bans::dsl::*;

    diesel::insert_into(bans)
        .values(models::Ban {
            roblox_id: json.roblox_id.to_owned(),
            added: chrono::Local::now().naive_local(),
            updated: chrono::Local::now().naive_local(),
            unbanned_at: json.unbanned_at.to_owned(),
            username: json.username.to_owned(),
            reason: json.reason.to_owned(),
        })
        .execute(connection)?;

    Ok(())
}

#[post("/Punish")]
async fn set_punishment(
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
    data: web::Json<models::NewBanJSON>,
) -> Result<HttpResponse, CustomError> {
    log::info!("{:?}", &data);

    let result = web::block(move || -> Result<(), DbError> {
        let mut connection = pool.get()?;

        create_ban(&mut connection, data)
    })
    .await;

    match result {
        Ok(Ok(_)) => Ok(HttpResponse::Ok().json(models::SuccessResponse { success: true })),
        Ok(Err(err)) => {
            log::error!("DB Error! {:?}", err);
            Err(CustomError::DbError)
        }
        Err(err) => {
            log::error!("DB Error! {:?}", err);
            Err(CustomError::DbError)
        }
    }
}

#[post("/Appeal/{userId}")]
async fn appeal_punishment(
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
    user_id: web::Path<String>,
) -> Result<HttpResponse, CustomError> {
    let sent_id: i64 = match user_id.into_inner().parse::<i64>() {
        Ok(value) => value,
        Err(_) => return Err(CustomError::Validation),
    };

    let result = web::block(move || -> Result<(), DbError> {
        let mut connection = pool.get()?;

        use crate::schema::bans::dsl::*;

        diesel::delete(bans.filter(roblox_id.eq(sent_id))).execute(&mut connection)?;

        Ok(())
    })
    .await;

    match result {
        Ok(Ok(_)) => Ok(HttpResponse::Ok().json(models::SuccessResponse { success: true })),
        Err(err) => {
            log::error!("DB Error! {:?}", err); // for debugging
            Err(CustomError::DbError)
        }
        Ok(Err(err)) => {
            log::error!("DB Error! {:?}", err);
            Err(CustomError::DbError)
        }
    }
}

#[get("/Punishments")]
async fn get_punishments(
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
) -> Result<HttpResponse, CustomError> {
    let result = web::block(move || -> Result<Vec<models::Ban>, DbError> {
        let mut connection = pool.get()?;

        use schema::bans::dsl::*;

        let ban_list = bans.load::<models::Ban>(&mut connection)?;

        Ok(ban_list)
    })
    .await;

    match result {
        Ok(Ok(json)) => Ok(HttpResponse::Ok().json(json)),
        _ => {
            log::error!("DB Error! {:?}", result);
            Err(CustomError::DbError)
        }
    }
}

// curl -d '{"roblox_id": 3, "reason":"smdsfsdells", "username":"unknodsdfsdwn", "unbanned_at"}' -H "Content-type: application/json" -X POST http://localhost:8080/Punish

#[get("/Punishment/{userId}")]
async fn get_punishment(
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
    user_id: web::Path<String>,
) -> Result<HttpResponse, CustomError> {
    let sent_id: i64 = match user_id.into_inner().parse::<i64>() {
        Ok(value) => value,
        Err(_) => return Err(CustomError::Validation),
    };

    let ban = web::block(move || -> Result<models::Ban, DbError> {
        let mut connection = pool.get()?;

        use crate::schema::bans::dsl::*;

        let ban = bans
            .filter(roblox_id.eq(sent_id))
            .first::<models::Ban>(&mut connection)?;

        Ok(ban)
    })
    .await;

    match ban {
        Ok(Ok(json)) => Ok(HttpResponse::Ok().json(json)),
        Err(err) => {
            log::error!("DB Error! {:?}", err);
            Err(CustomError::DbError)
        }
        Ok(Err(err)) => match err.downcast_ref::<diesel::result::Error>() {
            Some(NotFound) => Err(CustomError::NotFound),
            _ => {
                log::error!("DB Error! {:?}", err);
                Err(CustomError::DbError)
            }
        },
    }
}

async fn auth(
    req: actix_web::dev::ServiceRequest,
    next: Next<impl MessageBody + 'static>,
) -> Result<actix_web::dev::ServiceResponse<impl MessageBody + 'static>, Error> {
    let Some(token) = req.request().headers().get("Authorization") else {
        return Err(actix_web::error::ErrorUnauthorized("Invalid token"));
    };

    let Ok(result) = token.to_str() else {
        return Err(actix_web::error::ErrorUnauthorized("Invalid token"));
    };

    let api_key = std::env::var("API_KEY").expect("No API_Key environment variable!");

    match api_key.as_str() == result {
        true => next.call(req).await,
        _ => Err(actix_web::error::ErrorUnauthorized("Invalid token")),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let port = std::env::var("PORT")
        .expect("PORT should be set")
        .parse::<u16>()
        .expect("Could not parse PORT as a u16?");

    let db = connect_db();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .wrap(Logger::default())
            .wrap(from_fn(auth))
            .service(get_punishments)
            .service(get_punishment)
            .service(appeal_punishment)
            .service(set_punishment)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}

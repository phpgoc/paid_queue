use actix::prelude::*;
use actix_web::{middleware, web, App, Error as AWError, HttpResponse, HttpServer};
use r2d2_redis::{r2d2, RedisConnectionManager};
use r2d2_redis::redis::Commands;
use r2d2_redis::r2d2::Pool;

async fn incr(  pool: web::Data<Pool<RedisConnectionManager>>)-> Result<HttpResponse, AWError> {
    // let res = redis.send(Command(resp_array!["INCR","k"])).await.unwrap().ok().unwrap();
    let mut conn = pool.get().unwrap();
    let n: i64 = conn.incr("k", 1).unwrap();

    Ok(HttpResponse::Ok().body(format!("{}",n)))
   
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=trace,actix_redis=trace");
    env_logger::init();
    let manager = RedisConnectionManager::new("redis://redis:6379").unwrap();
    let pool = r2d2::Pool::builder()
        .build(manager)
        .unwrap();
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/")
                    .route(web::get().to(incr)),
            )
    })
    .bind("0.0.0.0:5000")?
    .run()
    .await
}

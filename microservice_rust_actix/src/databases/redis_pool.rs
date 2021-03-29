pub use r2d2_redis::r2d2::Pool;
pub use r2d2_redis::redis::{self, Commands};
pub use r2d2_redis::{r2d2, RedisConnectionManager};

pub fn get_pool() -> Pool<RedisConnectionManager> {
    let manager = RedisConnectionManager::new("redis://redis:6379").unwrap();
    r2d2::Pool::builder().build(manager).unwrap()
}

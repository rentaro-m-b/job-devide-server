use diesel::pg::PgConnection;
use dotenv::dotenv;
use diesel::r2d2::{ConnectionManager, Pool};
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_connection_pool() -> DbPool {
    dotenv().ok();

    let db_url = &env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);

    let db_pool_max_size_string = &env::var("DB_POOL_MAX_SIZE").unwrap_or("5".to_string());
    let db_pool_max_size: u32 = db_pool_max_size_string.parse().unwrap();
    
    Pool::builder()
        .test_on_check_out(true)
        .max_size(db_pool_max_size)
        .build(manager)
        .expect("Could not build connection pool")
}

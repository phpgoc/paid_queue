use microservice_rust_actix::databases;
fn main(){
    let pool = databases::redis_pool::get_pool();
    println!("paid_queue");
    
}
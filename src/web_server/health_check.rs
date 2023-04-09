use actix_web::get;
use actix_web::HttpRequest;

#[get("/health")]
pub async fn is_alive(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    "I am ok!\r\n"
}

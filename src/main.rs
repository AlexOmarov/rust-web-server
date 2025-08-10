use actix_web::{get, web, App, HttpServer, Responder};

// Простая функция для тестирования
pub fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(greet))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_numbers() {
        assert_eq!(add_numbers(2, 3), 5);
        assert_eq!(add_numbers(-1, 1), 0);
        assert_eq!(add_numbers(0, 0), 0);
    }

    #[test]
    fn test_add_numbers_edge_cases() {
        assert_eq!(add_numbers(i32::MAX, 0), i32::MAX);
        assert_eq!(add_numbers(i32::MIN, 0), i32::MIN);
    }
}

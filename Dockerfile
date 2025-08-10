# Этап сборки
FROM rust:1.75-alpine AS builder

WORKDIR /app

# Копируем файлы зависимостей
COPY Cargo.toml Cargo.lock ./

# Создаем пустой main.rs для кэширования зависимостей
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Собираем зависимости
RUN cargo build --release

# Копируем исходный код
COPY src/ ./src/

# Собираем приложение
RUN cargo build --release

# Этап запуска
FROM alpine:latest

# Устанавливаем необходимые пакеты
RUN apk add --no-cache libc6-compat

WORKDIR /app

# Копируем собранное приложение
COPY --from=builder /app/target/release/rust-web-server .

# Создаем пользователя для безопасности
RUN addgroup -g 1001 -S rust && \
    adduser -S rust -u 1001

USER rust

# Открываем порт
EXPOSE 8080

# Запускаем приложение
CMD ["./rust-web-server"]

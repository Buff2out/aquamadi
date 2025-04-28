# Агрегатор API:

## Функционал:

Возможность выбора API (например хотим посмотреть температуру за такую то дату в таком то городе) -> выбираем в ползунках дату, время, город (в логике обработка какой запрос нужно сформировать к API) -> выводит результат.

в базу данных сохраняется 
домен, 
компания (возможность связи один ко многим - когда у одной компании много API), 
сама ссылка на API
?как реализовать формат по которому делается конкретный запрос.

```
aquamadi/
├── .github/
│   └── workflows/      # GitHub Actions (CI/CD)
├── migrations/          # SQLX миграции
├── src/
│   ├── main.rs         # Точка входа
│   ├── api/            # Роуты, обработчики
│   ├── config/         # Конфигурация (env, settings)
│   ├── database/       # Подключение к Postgres, репозитории
│   ├── models/         # DTO, сущности БД
│   ├── services/       # Бизнес-логика, агрегация данных
│   ├── auth/           # JWT, middleware
│   └── error.rs        # Кастомные ошибки
├── docker-compose.yml  # Docker-конфигурация
├── .env                # Переменные окружения (не в гите!)
├── .gitignore          # Игнорируем .env, target/, etc.
└── Cargo.toml          # Зависимости Rust
```

```
[dependencies]
axum = "0.7"
tokio = { version = "1.0", features = ["full"] }
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio"] }
tracing = "0.1"
serde = { version = "1.0", features = ["derive"] }
tracing-subscriber = "0.3"
dotenvy = "0.15"  # Для загрузки .env
```

```--no-default-features --features postgres``` 

- эти флаги нужны чтобы не установить лишних зависимостей.

## Как выполнять проект?

feature/название-своей-фичи

коммит, пуш, согласование на мёрж

## Postgres, Docker

Установка докера (если его нет).

### linux

https://docs.docker.com/engine/install/

### windows

https://docs.docker.com/desktop/setup/install/windows-install/

## Запуск контейнера с Postgresql

### .env

    Дисклеймер!
    Можно в целом спалить .env в README, ведь проект в первую очередь учебный. Но в проде - низзя! :)


>А теперь внимание! не копипастить! Смотреть внимательно, это .env файл. Его ни в коем случае нельзя коммитить, он должен находиться в gitignore

Смотрим мем для закрепления.

https://t.me/bugnotfeature/16322

В корне проекта создаём файл `.env` с содержимым:

```
POSTGRES_USER=aquamadi
POSTGRES_PASSWORD=secret
POSTGRES_DB=aquamadi_db
```

Проверить, запущен ли сервис docker, если да, то:
Строго под Linux (либо wsl2)!
```
sudo docker-compose up -d
```

### Устанавливаем sqlx 

- нужная штука

SQLx — это асинхронный драйвер для работы с базами данных в Rust, который сочетает безопасность типов, производительность и гибкость. Вот ключевые аспекты, которые нужно знать:
1. Основные особенности SQLx

    Безопасность типов: Проверяет SQL-запросы на этапе компиляции.
    Например, если вы опечатались в названии столбца или передали неверный тип данных — компилятор Rust выдаст ошибку.

    Асинхронность: Работает с async/await через Tokio или async-std (в вашем проекте используется Tokio).

    Поддержка СУБД: PostgreSQL, MySQL, SQLite. В вашем проекте — только Postgres.

    Отсутствие ORM: SQLx не навязывает объектно-реляционное отображение. Вы пишете "сырые" SQL-запросы, но с проверкой на корректность.

    Макросы: Например, query! проверяет синтаксис SQL и типы данных прямо в коде.

```
cargo install sqlx-cli --no-default-features --features postgres
```

## Тест запуска

В корне проекта
```
cargo run
```

должно быть

```
> cargo run
   Compiling ...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.33s
     Running `target/debug/aquamadi`
2025-04-28T14:55:45.348497Z  INFO aquamadi: ✅ Successfully connected to database
2025-04-28T14:55:45.348690Z  INFO aquamadi: 🚀 Server started on port 3000

```




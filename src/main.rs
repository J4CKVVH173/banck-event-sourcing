mod domain;
mod application;
mod infrastructure;
#[cfg(feature = "cli")]
use self::infrastructure::CommandReader;

#[cfg(not(feature = "cli"))]
use self::infrastructure::Server;

use tokio;

use self::infrastructure::storages::FileStorage;
use self::application::use_cases::UseCases;

#[cfg(feature = "cli")]
fn main() {
    let storage = FileStorage::new("./store.txt");
    let use_cases = UseCases::new(storage);

    CommandReader::run(use_cases);
}

#[cfg(not(feature = "cli"))]
#[tokio::main]
async fn main() {
    // Инициализация хранилища и бизнес-логики (предполагается, что реализации есть)
    let storage = FileStorage::new("./store.txt");
    let use_cases = UseCases::new(storage);

    // Инициализация и запуск сервера
    let server = Server::new(use_cases);
    server.run("127.0.0.1:3000").await;
}
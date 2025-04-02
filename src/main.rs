mod domain;
mod application;
mod infrastructure;

use self::infrastructure::CommandReader;
use self::infrastructure::storages::FileStorage;
use self::application::use_cases::UseCases;

fn main() {
    let storage = FileStorage::new("./store.txt");
    let use_cases = UseCases::new(storage);

    CommandReader::run(use_cases);
}

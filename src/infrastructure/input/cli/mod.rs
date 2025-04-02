use clap::{arg, Command as ClapCommand};
use std::io::{self, Write};

use crate::application::{use_cases::UseCases, Storage};

/// Структура, отвечающая за чтение и обработку команд из консоли.
pub struct CommandReader;

impl CommandReader {
    /// Метод для чтения строки с приглашением.
    fn read_input(prompt: &str) -> io::Result<String> {
        print!("{}", prompt);
        io::stdout().flush()?; // Выводим приглашение на экран сразу
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string())
    }

    /// Метод, который запускает бесконечный цикл, внутри которого:
    /// 1. Запрашивается ввод команды.
    /// 2. Используется `clap` для парсинга введённой строки.
    /// 3. Выполняется действие, соответствующее команде.
    pub fn run<T: Storage>(use_cases: UseCases<T>) {
        // Определяем "приложение" clap с подкомандами add, remove, total.
        let app = ClapCommand::new("interactive")
            .subcommand(
                ClapCommand::new("add").arg(
                    arg!(<number> "Число для добавления")
                        .value_parser(clap::value_parser!(usize)),
                ),
            )
            .subcommand(
                ClapCommand::new("remove").arg(
                    arg!(<number> "Число для удаления")
                        .value_parser(clap::value_parser!(usize)),
                ),
            )
            .subcommand(ClapCommand::new("total"))
            .subcommand(ClapCommand::new("end"))            ;

        loop {
            // Считываем ввод пользователя
            let input = match Self::read_input("Введите команду (add, remove, total, end): ") {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Ошибка ввода: {}", e);
                    continue;
                }
            };

            // Разбиваем введённую строку на "аргументы" (по пробелам).
            // Добавляем фиктивное имя программы для корректной работы clap.
            let args = std::iter::once("interactive")
                .chain(input.split_whitespace())
                .collect::<Vec<_>>();

            // Парсим аргументы с помощью clap.
            let matches = match app.clone().try_get_matches_from(args) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("Ошибка разбора команды: {}", e);
                    continue;
                }
            };

            // Обработка подкоманд
            match matches.subcommand() {
                Some(("add", sub_m)) => {
                    let number: usize = *sub_m.get_one::<usize>("number").unwrap();
                    match use_cases.add(number) {
                        Ok(_) => println!("Добавлено: {}", number),
                        Err(e) => eprintln!("Ошибка при добавлении: {}", e),
                    }
                }
                Some(("remove", sub_m)) => {
                    let number: usize = *sub_m.get_one::<usize>("number").unwrap();
                    match use_cases.remove(number) {
                        Ok(_) => println!("Удалено: {}", number),
                        Err(e) => eprintln!("Ошибка при добавлении: {}", e),
                    }
                }
                Some(("total", _)) => {
                    match use_cases.get_amount() {
                        Ok(amount) => println!("На счету: {}", amount),
                        Err(e) => eprintln!("Ошибка при добавлении: {}", e),
                    }
                }
                Some(("end", _)) => break,
                _ => {
                    eprintln!("Неизвестная команда");
                }
            }
        }
    }
}

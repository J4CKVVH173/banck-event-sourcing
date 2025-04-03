use clap::ArgMatches;
use signals::Signal;

mod setup;
mod signals;

use self::setup::setup_cli;
use std::{io::{self, Write}, rc::Rc};

use crate::application::{Storage, use_cases::UseCases};

/// Структура, отвечающая за чтение и обработку команд из консоли.
pub struct CommandReader;

impl CommandReader {
    fn read_input(prompt: &str) -> io::Result<String> {
        print!("{}", prompt);
        io::stdout().flush()?; // Выводим приглашение на экран сразу
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string())
    }

    fn split_input_args(input: &str) -> io::Result<Vec<&str>> {
        Ok(std::iter::once("interactive")
            .chain(input.split_whitespace())
            .collect::<Vec<_>>())
    }

    fn handle_command<T: Storage>(use_cases: Rc<UseCases<T>>, matches: ArgMatches) -> Signal {
        match matches.subcommand() {
            Some(("add", sub_m)) => {
                let number: usize = *sub_m.get_one::<usize>("number").unwrap();
                match use_cases.add(number) {
                    Ok(_) => println!("Добавлено: {}", number),
                    Err(e) => eprintln!("Ошибка при добавлении: {}", e),
                };
            }
            Some(("remove", sub_m)) => {
                let number: usize = *sub_m.get_one::<usize>("number").unwrap();
                match use_cases.remove(number) {
                    Ok(_) => println!("Удалено: {}", number),
                    Err(e) => eprintln!("Ошибка при добавлении: {}", e),
                };
            }
            Some(("total", _)) => match use_cases.get_amount() {
                Ok(amount) => println!("На счету: {}", amount),
                Err(e) => eprintln!("Ошибка при добавлении: {}", e),
            },
            Some(("end", _)) => return Signal::Stop,
            _ => {
                eprintln!("Неизвестная команда");
            }
        }
        Signal::Resume
    }

    /// Метод, который запускает бесконечный цикл, внутри которого:
    /// 1. Запрашивается ввод команды.
    /// 2. Используется `clap` для парсинга введённой строки.
    /// 3. Выполняется действие, соответствующее команде.
    pub fn run<T: Storage>(use_cases: UseCases<T>) {
        let clonable_use_case = Rc::new(use_cases);
        let cli = setup_cli();

        loop {
            // Считываем ввод пользователя
            let input = match Self::read_input("Введите команду (add, remove, total, end): ")
            {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Ошибка ввода: {}", e);
                    continue;
                }
            };

            let args = match Self::split_input_args(&input) {
                Ok(args) => args,
                Err(e) => {
                    eprintln!("Ошибка разбора аргументов: {}", e);
                    continue;
                }
            };

            let matches = match cli.clone().try_get_matches_from(args) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("Ошибка разбора команды: {}", e);
                    continue;
                }
            };

            let signal = Self::handle_command(clonable_use_case.clone(), matches);

            if signal == Signal::Stop {
                break;
            }
        }
    }
}

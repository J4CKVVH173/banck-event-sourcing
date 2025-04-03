use clap::{Command, arg};

pub fn setup_cli() -> Command {
    Command::new("interactive")
        .subcommand(
            Command::new("add").arg(
                arg!(<number> "Число для добавления").value_parser(clap::value_parser!(usize)),
            ),
        )
        .subcommand(
            Command::new("remove")
                .arg(arg!(<number> "Число для удаления").value_parser(clap::value_parser!(usize))),
        )
        .subcommand(Command::new("total"))
        .subcommand(Command::new("end"))
}

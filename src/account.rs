use crate::cli::{AddArgs, Command, GetArgs, RemoveArgs};
use arboard::Clipboard;
use data_encoding::BASE32;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::process::exit;
use std::time::{SystemTime, UNIX_EPOCH};
use tabled::settings::object::{Columns, Rows};
use tabled::settings::{Color, Remove, Style};
use tabled::{Table, Tabled};
use totp_lite::{Sha1, totp};

#[derive(Clone, Tabled, Serialize, Deserialize)]
struct Account {
    name: String,
    secret: String,
}

impl Account {
    fn new(name: &str, secret: &str) -> Self {
        Self {
            name: name.to_owned(),
            secret: secret.to_owned(),
        }
    }

    fn get_totp(&self, seconds: u64) -> String {
        let result = totp::<Sha1>(&decode(&self.secret), seconds);
        result[2..8].to_owned()
    }

    fn print_totp(&self, copy: bool) {
        let sec = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let code = self.get_totp(sec);

        let exp = 30 - (sec % 30);
        println!(
            "{}: {}\nExpires in {} second{}.",
            self.name.green(),
            code.bright_cyan(),
            exp.green(),
            if exp > 1 { "s" } else { "" }
        );

        if copy {
            let mut clipboard = Clipboard::new().unwrap_or_else(|_| {
                eprintln!("Unable to copy to clipboard.");
                exit(1);
            });
            clipboard.set_text(code).unwrap_or_else(|_| {
                eprintln!("Unable to copy to clipboard.");
                exit(1);
            });
        }
    }
}

pub(crate) struct AccountArray {
    accounts: Vec<Account>,
    json_path: PathBuf,
}

impl AccountArray {
    pub(crate) fn new(json_path: &Path) -> Self {
        Self {
            accounts: Vec::new(),
            json_path: json_path.to_path_buf(),
        }
    }

    pub(crate) fn load(&mut self) {
        let mut file = OpenOptions::new().read(true).open(&self.json_path).unwrap();

        file.seek(SeekFrom::Start(0)).unwrap();

        let reader = BufReader::new(file);
        self.accounts = serde_json::from_reader(reader).unwrap_or_else(|_| {
            eprintln!(
                "{}",
                "JSON file is corrupted. Backup your secrets immediately!".bright_red()
            );
            exit(1);
        });
        self.accounts.sort_by(|a, b| a.name.cmp(&b.name));
    }

    pub(crate) fn match_cmd(&mut self, command: Command) {
        match command {
            Command::List(arg) => self.list(arg.secret),
            Command::Path(_) => self.path(),
            Command::Add(arg) => self.add(arg),
            Command::Remove(arg) => self.try_remove(arg),
            Command::Clean(_) => self.try_clean(),
            Command::Get(arg) => self.get(arg),
        };
    }

    fn save(&self) {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.json_path)
            .unwrap();

        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &self.accounts).unwrap();
        writer.flush().unwrap();
    }

    fn list(&self, secret: bool) {
        let white = Color::default();
        let blue = Color::rgb_fg(87, 170, 247);
        let green = Color::rgb_fg(13, 188, 121);

        let mut table = Table::builder(&self.accounts).index().name(None).build();

        table.with(Style::rounded());
        table.modify(Columns::first(), &blue);
        table.modify(Columns::one(1), &green);

        if !secret {
            table.with(Remove::column(Columns::last()));
        } else {
            table.modify(Columns::one(2), &white);
        }

        table.modify(Rows::first(), white);

        println!("{}", table);
    }

    fn path(&self) {
        println!("{}", self.json_path.display().bright_cyan());
    }

    fn add(&mut self, arg: AddArgs) {
        decode(&arg.secret);

        let name = &arg.name;
        let acc = Account::new(name, &arg.secret);

        match self.accounts.binary_search_by(|a| a.name.cmp(name)) {
            Ok(_) => {
                println!(
                    "Account {} already existed, please remove it first.",
                    name.green()
                );
            }
            Err(index) => {
                self.accounts.insert(index, acc);
                self.save();
                println!("Account added: {}", name.green());
            }
        }
    }

    fn try_remove(&mut self, arg: RemoveArgs) {
        let name = &arg.name;
        match self.find(name) {
            Ok(index) => self.remove_idx(index),
            Err(_) => {
                eprintln!("Cannot find account with name: {}", name.green());
                exit(1);
            }
        };
    }

    fn try_clean(&mut self) {
        self.list(true);
        print!(
            "\nDo you wish to delete all of these accounts? This action is irreversible. (yes/[no]): "
        );
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() != "yes" {
            println!("{}", "Aborted.".yellow());
            exit(0);
        }

        self.accounts.clear();
        self.save();
        println!("{}", "Removed all accounts.".bright_red());
    }

    fn get(&mut self, arg: GetArgs) {
        let index;
        let name = &arg.name;

        match name {
            None => {
                self.list(false);
                print!("\nSelect account from ID: ");

                let mut buf = String::new();
                index = match flush_ask(&mut buf).parse() {
                    Ok(index) => index,
                    Err(_) => {
                        eprintln!("{}", "Invalid ID!".bright_red());
                        exit(1);
                    }
                }
            }
            Some(name) => {
                index = match self.find(name) {
                    Ok(index) => index,
                    Err(_) => {
                        eprintln!("Cannot find account with name: {}", name.green());
                        exit(1);
                    }
                }
            }
        }
        match self.accounts.get(index) {
            None => {
                eprintln!("{}", "Index out of range!".bright_red());
                exit(1);
            }
            Some(acc) => acc.print_totp(!arg.no_copy),
        }
    }

    fn find(&self, name: &str) -> Result<usize, usize> {
        let name = &name.to_owned();
        self.accounts.binary_search_by(|a| a.name.cmp(name))
    }

    fn remove_idx(&mut self, index: usize) {
        let acc = self.accounts[index].clone();

        print!(
            "Do you wish to delete this account: {} (yes/[no]): ",
            acc.name.green()
        );
        let mut buf = String::new();
        if flush_ask(&mut buf) != "yes" {
            println!("{}", "Operation aborted.".yellow());
            exit(0);
        }

        let secret = &acc.secret;
        self.accounts.remove(index);
        self.save();
        println!(
            "Removed account: {}\nSecret: {}",
            acc.name.green(),
            secret.green()
        );
    }
}

fn flush_ask(buf: &mut String) -> &str {
    std::io::stdout().flush().unwrap();

    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
    buf.trim()
}

fn decode(secret: &str) -> Vec<u8> {
    BASE32.decode(secret.as_bytes()).unwrap_or_else(|_| {
        eprintln!("Invalid secret: {}", secret.green());
        exit(1);
    })
}

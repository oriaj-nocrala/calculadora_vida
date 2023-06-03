mod error;
mod my_naive_date;

use std::io::{self, Write};

use chrono::prelude::*;
use error::Error;

use my_naive_date::MyNaiveDate;

use colored::*;

fn main() {
    match run() {
        Ok(_) => {},
        Err(e) => {
            println!("{}", "Ocurrió un error".red());
            eprintln!("{}", e);
            println!("{}", "Presione enter para continuar...".green());
            let _ = io::stdin().read_line(&mut String::new());
        },
    }
}

fn run() -> Result<(), Error> {
    println!("Ingrese su fecha de nacimiento: ");
    let mut fecha_nacimiento = String::new();

    io::stdin().read_line(&mut fecha_nacimiento)?;
    let fecha_nacimiento = fecha_nacimiento.trim();

    let date_nacimiento: NaiveDate = fecha_nacimiento.try_into().map(|my_date: MyNaiveDate| my_date.into())?;

    let transcurrido = date_nacimiento.signed_duration_since(Utc::now().naive_utc().date()).num_days();

    println!("dias transcurridos desde la fecha de nacimiento: {}", transcurrido.abs());

    println!("{}", "Presione enter para continuar...".green());
    io::stdout().flush()?;
    io::stdin().read_line(&mut String::new())?;

    Ok(())
}

mod models;
const FILE_PATH: &str = "expenses.json";
use chrono::Datelike;
use clap::Parser;
use models::{
    cli::{Cli, Commands},
    expenses::Item,
};
use std::{fs, fs::OpenOptions, io::Write};

fn create_file() -> std::io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(FILE_PATH);
    let content = "[]";

    match file {
        Ok(mut file) => file.write_all(content.as_bytes())?,
        Err(e) => {
            if e.kind() != std::io::ErrorKind::AlreadyExists {
                return Err(e);
            }
        }
    }
    Ok(())
}

fn get_items() -> Vec<Item> {
    let file_content = fs::read_to_string(FILE_PATH).expect("Error al leer el archivo");
    let items: Vec<Item> =
        serde_json::from_str(&file_content).expect("Error al deserializar el archivo");

    return items;
}

fn save_items(items: &Vec<Item>) {
    let file_content = serde_json::to_string_pretty(items).expect("Error al serializar el archivo");
    fs::write(FILE_PATH, file_content).expect("Error al escribir el archivo");
    println!("Entradas guardadas con éxito.");
}

fn add_entry(description: &str, amount: &f32, category: Option<&str>) {
    let mut item_list = get_items();
    let id_item = match item_list.last() {
        Some(item) => item.id + 1,
        None => 1,
    };
    let item = Item {
        id: id_item,
        description: description.to_string(),
        amount: *amount,
        date: chrono::Local::now().format("%Y-%m-%d").to_string(),
        category: match category {
            Some(value) => value.to_string(),
            None => "unknown".to_string(),
        },
    };
    item_list.push(item);
    save_items(&item_list);
}

//TODO Agregar un filtro por categoria al listado de entradas
fn list_entries() {
    let item_list = get_items();
    println!("#ID DATE DESCRIPTION AMOUNT");
    for item in item_list {
        println!(
            "#{} {} {} {}",
            item.id, item.date, item.description, item.amount
        );
    }
}

//TODO Opcional. Agregar un filtro por categoria al resumen por mes/año
fn summary_entries(month: &Option<u8>) {
    let item_list = get_items();
    match month {
        Some(month) => {
            if month < &1 || month > &12 {
                panic!("Invalid month");
            }
            let current_year = chrono::Local::now().year();
            let formated_month = format!("{}-{:02}", current_year, month);
            let filtered_items: Vec<Item> = item_list
                .iter()
                .filter(|item| item.date.contains(&formated_month))
                .cloned()
                .collect();
            let total = filtered_items
                .iter()
                .fold(0.0, |acc, item| acc + item.amount);
            let rounded_total = format!("{:.2}", total);
            println!("Gastos del mes: ${}", rounded_total);
        }
        None => {
            let total = item_list.iter().fold(0.0, |acc, item| acc + item.amount);
            let rounded_total = format!("{:.2}", total);
            println!("Gastos en total: ${}", rounded_total);
        }
    }
}

fn delete_entry(id: &u32) {
    let mut item_list = get_items();
    let filtered_items: Vec<Item> = item_list
        .iter()
        .filter(|item| item.id != *id)
        .cloned()
        .collect();
    item_list = filtered_items;
    save_items(&item_list);
    println!("Entrada borrada con éxito.");
}

fn update_entry(
    id: &u32,
    description: &Option<String>,
    amount: &Option<f32>,
) -> Result<(), String> {
    let mut item_list = get_items();

    let selected_item = item_list
        .iter_mut()
        .find(|item| item.id == *id)
        .ok_or("Item not found")?;

    if let Some(description) = description {
        selected_item.description = description.to_string();
    }

    if let Some(amount) = amount {
        selected_item.amount = *amount;
    }

    save_items(&item_list);

    Ok(())
}

fn main() {
    create_file().expect("Error al crear el archivo");
    let cli = Cli::parse(); // Parsear los argumentos de la CLI

    match &cli.command {
        Commands::Add {
            description,
            amount,
            category,
        } => add_entry(description, amount, category.as_deref()),
        Commands::List => list_entries(),
        Commands::Summary { month } => summary_entries(month),
        Commands::Delete { id } => delete_entry(id),
        Commands::Update {
            id,
            description,
            amount,
        } => {
            update_entry(id, description, amount).unwrap();
            println!("Entrada {} actualizada", id);
        }
        //TODO Implementar la función para exportar como CSV
        Commands::Export => {
            println!("Exporting")
        }
    }
}

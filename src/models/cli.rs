use clap::{Parser, Subcommand};
/// Una CLI simple para gestionar comandos.
#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Meta información
pub struct Cli {
    /// Subcomando a ejecutar
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Agregar una entrada con descripción y cantidad
    Add {
        /// Descripción de la entrada
        #[arg(short, long)]
        description: String,

        /// Cantidad asociada a la entrada
        #[arg(short, long)]
        amount: f32,
        ///Categoria del producto
        #[arg(short, long)]
        category: Option<String>,
    },
    /// Listar todas las entradas
    List {
        #[arg(short, long)]
        ///Categoria a filtrar
        category: Option<String>,
    },
    /// Resumen de todas las entradas
    Summary {
        #[arg(short, long)]
        month: Option<u8>,
    },
    /// Borrar todas las entradas
    Delete {
        /// ID de la entrada a borrar
        #[arg(short, long)]
        id: u32,
    },
    /// Actualizar una entrada
    Update {
        /// ID de la entrada a actualizar
        #[arg(short, long)]
        id: u32,
        /// Descripción de la entrada
        #[arg(short, long)]
        description: Option<String>,

        /// Cantidad asociada a la entrada
        #[arg(short, long)]
        amount: Option<f32>,
    },
    ///Exporta la lista de gastos como un archivo CSV
    Export,
}

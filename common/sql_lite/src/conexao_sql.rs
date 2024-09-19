use rusqlite::{Connection};
use std::error::Error;
use log::{info, warn};

pub(crate) fn conexao_loader(data_name: &str, table_name: &str, colum_nome_arquivo: &str, colum_tamanho_arquivo: &str, colum_data_copia: &str, colum_binario_xml: &str) -> Result<(), Box<dyn Error>> {
    warn!("conexao_loader: conexao_loader---START");
    // Crie o banco de dados e a tabela se não existirem
    let conn  = Connection::open(data_name)?;
    conn.execute(&format!(
        "CREATE TABLE IF NOT EXISTS {} (
                {} TEXT PRIMARY KEY,
                {} INTEGER,
                {} TEXT,
                {} BLOB
            )",
        table_name, colum_nome_arquivo, colum_tamanho_arquivo, colum_data_copia, colum_binario_xml
    ), ())?;

    info!(target: "yak_events", "Commencing yak shaving for");
    // Retorna Ok(()) em caso de sucesso
    Ok(())
}
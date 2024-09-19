mod conexao_sql;

pub fn conexao_sql_acess_server(data_name: &str, table_name: &str, colum_nome_arquivo: &str, colum_tamanho_arquivo: &str, colum_data_copia: &str, colum_binario_xml: &str){
    let _ = conexao_sql::conexao_loader(data_name, table_name, colum_nome_arquivo, colum_tamanho_arquivo, colum_data_copia, colum_binario_xml);
    if let Err(e) = conexao_sql::conexao_loader(data_name, table_name, colum_nome_arquivo, colum_tamanho_arquivo, colum_data_copia, colum_binario_xml) {
        eprintln!("Erro ao carregar a conexão: {}", e);
    }
}

use sql_lite::conexao_sql_acess_server;
const DATABASE_NAME: &str = "control_xml.db";
const TABLE_NAME: &str = "arquivos_xml";
const COLUMN_NOME_ARQUIVO: &str = "nome_arquivo";
const COLUMN_TAMANHO_ARQUIVO: &str = "tamanho_arquivo";
const COLUMN_DATA_COPIA: &str = "data_copia";
const COLUMN_BINARIO_XML_ARQUIVO: &str = "binario_xml_arquivo";

pub fn loader_painel_view() {
    conexao_sql_acess_server(DATABASE_NAME,TABLE_NAME,COLUMN_NOME_ARQUIVO,COLUMN_TAMANHO_ARQUIVO,COLUMN_DATA_COPIA,COLUMN_BINARIO_XML_ARQUIVO)
}
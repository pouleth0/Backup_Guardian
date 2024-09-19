use std::fs::File;

fn main() {
    let caminho = "view/painel_load";
    match File::open(caminho) {
        Ok(_) => println!("Arquivo aberto com sucesso!"),
        Err(e) => println!("Erro ao abrir o arquivo: {}", e),
    }

}
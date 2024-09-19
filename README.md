# 📦 Backup_Guardian

`Backup_Guardian` é um sistema de controle de backup que permite enviar pastas específicas ou múltiplas pastas configuradas para serviços de armazenamento como FTP, Google Drive, AWS ou NAS. Este sistema modular é organizado em várias camadas, facilitando a integração com diferentes serviços de backup e melhorando a eficiência do processo.

## 🛠️ Funcionalidades Principais

- **Backup Automatizado**: Configuração automática de backup para pastas designadas.
- **Suporte a Múltiplos Serviços**: Compatível com FTP, Google Drive, AWS e NAS.
- **Monitoramento de Arquivos**: Detecta mudanças em arquivos e realiza backup de arquivos modificados.
- **Modularidade**: O projeto é organizado em módulos que facilitam a expansão e a reutilização.

## 🚀 Começando

Siga as instruções abaixo para rodar o `Backup_Guardian` em sua máquina.

### Pré-requisitos

Antes de começar, certifique-se de ter o seguinte software instalado em sua máquina:

- [Rust](https://www.rust-lang.org/tools/install) - Para compilar e rodar o projeto.
- Serviços de armazenamento compatíveis (FTP, Google Drive, AWS ou NAS).

### Instalação

1. Clone este repositório:

    ```bash
    git clone https://github.com/seu-usuario/Backup_Guardian.git
    ```

2. Navegue até o diretório do projeto:

    ```bash
    cd Backup_Guardian
    ```

3. Compile o projeto em modo release:

    ```bash
    cargo build --release
    ```

4. Execute o binário gerado:

    ```bash
    ./target/release/painel_load
    ```

## 📂 Estrutura do Projeto

```plaintext
Backup_Guardian/
│
├── common/
│   ├── sql_lite/          # Módulo para integração com SQLite
│   ├── dirs/              # Gerenciamento de diretórios
│   ├── notify/            # Monitoramento de mudanças em arquivos
│   ├── fs2/               # Sistema de arquivos e controle de lock
│   ├── winapi/            # Interações com a API do Windows
│   ├── winit/             # Gerenciamento de janelas no sistema
│   └── tray_item/         # Controle do ícone na bandeja do sistema
│
├── controller/
│   └── painel_controller/  # Controlador da lógica do painel de controle
│
├── view/
│   └── painel_load/        # Interface gráfica do painel de controle
│
├── Cargo.toml              # Configurações do projeto e dependências
└── README.md               # Documentação do projeto
```

## 📦 Dependências
Aqui estão as principais dependências usadas no projeto, conforme especificado no arquivo Cargo.toml:

```toml
[dependencies]
notify = "6.1.1"
rusqlite = { version = "0.32.1", default-features = false, features = ["bundled"] }
dirs = "5.0.1"
fs2 = "0.4.3"
winapi = "0.3.9"
winit = "0.30.5"
tray-item = "0.10.0"
log = "0.4.22"
```
- **notify:** Utilizado para monitoramento de alterações em arquivos e diretórios.
- **rusqlite:** Integração com o banco de dados SQLite.
- **dirs:** Manipulação de diretórios.
- **fs2:** Extensões para manipulação de sistemas de arquivos.
- **winapi:** Interações com a API do Windows.
- **winit:** Criação e manipulação de janelas no sistema.
- **tray-item:** Gerenciamento de ícones na bandeja do sistema.
## 🔧 Configuração do Projeto
Este projeto inclui as seguintes otimizações no arquivo Cargo.toml:
```
[profile.release]
lto = "fat"
opt-level = "z"
```

- **LTO (Link Time Optimization):** Otimiza o tempo de execução, criando binários mais eficientes.
- **opt-level = "z":** Otimiza o tamanho do binário.

## 🚧 Configurando o Backup
Para configurar o envio de arquivos para os serviços de backup, acesse a interface de controle e preencha as credenciais e configurações específicas de cada serviço.

Modos de Backup Suportados
1. FTP: Insira o endereço do servidor FTP e suas credenciais.
2. Google Drive: Autentique sua conta Google e escolha a pasta de destino.
3. AWS: Configure as chaves de acesso e o bucket do S3.
4. NAS: Defina o caminho de rede e autenticações necessárias.

## 🧪 Testes
Para rodar os testes unitários do projeto, execute o seguinte comando:
```
cargo test
```
Isso irá rodar todos os testes definidos no projeto e mostrar o status de cada um deles.

## 🛠️ Futuras Implementações
- **Mais Serviços de Backup:** Suporte para novos serviços de armazenamento na nuvem.
- **Relatórios de Backup:** Relatórios detalhados sobre os backups realizados.
- **Agendamento Automático:** Funcionalidade para agendamento de backups periódicos.
  
## 📄 Licença
Este projeto está licenciado sob os termos da licença MIT. Veja o arquivo LICENSE para mais detalhes.

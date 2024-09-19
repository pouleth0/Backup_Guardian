# 📦 Backup_Guardian

`Backup_Guardian` is a backup management system that allows users to send specific folders or multiple configured folders to storage services such as FTP, Google Drive, AWS, or NAS. This modular system is organized into multiple layers, facilitating integration with various backup services and enhancing process efficiency.

## 🛠️ Key Features

- **Automated Backup**: Automatically configures backups for designated folders.
- **Multi-Service Support**: Compatible with FTP, Google Drive, AWS, and NAS.
- **File Monitoring**: Detects file changes and performs backups of modified files.
- **Modular Design**: The project is organized into modules, allowing for easy expansion and reuse.

## 🚀 Getting Started

Follow the instructions below to run `Backup_Guardian` on your machine.

### Prerequisites

Before starting, ensure that the following software is installed on your machine:

- [Rust](https://www.rust-lang.org/tools/install) - Required to compile and run the project.
- Compatible storage services (FTP, Google Drive, AWS, or NAS).

### Installation

1. Clone this repository:

    ```bash
    git clone https://github.com/your-username/Backup_Guardian.git
    ```

2. Navigate to the project directory:

    ```bash
    cd Backup_Guardian
    ```

3. Compile the project in release mode:

    ```bash
    cargo build --release
    ```

4. Run the generated binary:

    ```bash
    ./target/release/painel_load
    ```

## 📂 Project Structure

```plaintext
Backup_Guardian/
│
├── common/
│   ├── sql_lite/          # SQLite integration module
│   ├── dirs/              # Directory management
│   ├── notify/            # File change monitoring
│   ├── fs2/               # Filesystem management and lock control
│   ├── winapi/            # Windows API interactions
│   ├── winit/             # Window management in the system
│   └── tray_item/         # System tray icon control
│
├── controller/
│   └── painel_controller/  # Controller for the control panel logic
│
├── view/
│   └── painel_load/        # Graphical interface for the control panel
│
├── Cargo.toml              # Project configuration and dependencies
└── README.md               # Project documentation
```

## 📦 Dependencies
Here are the main dependencies used in the project, as specified in the **Cargo.toml** file:

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
- **notify:** Used for monitoring changes in files and directories.
- **rusqlite:** SQLite database integration.
- **dirs:** Directory manipulation.
- **fs2:** Extensions for file system manipulation.
- **winapi:** Windows API interactions.
- **winit:** Window creation and management in the system.
- **tray-item:** System tray icon management.

## 🔧 Project Configuration
This project includes the following optimizations in the **Cargo.toml** file:
```
[profile.release]
lto = "fat"
opt-level = "z"
```

- **LTO (Link Time Optimization): Optimizes runtime by producing more efficient binaries.
- **opt-level = "z":** Optimizes binary size.

## 🚧 Configuring Backup
To configure file transfers to backup services, access the control panel interface and input the credentials and specific settings for each service.

Supported Backup Modes
1. FTP: Enter the FTP server address and credentials.
2. Google Drive:  Authenticate your Google account and choose the destination folder.
3. AWS: Configure access keys and the S3 bucket.
4. NAS: Define the network path and necessary authentications.

## 🧪 Running Tests
To run unit tests for the project, execute the following command:
```
cargo test
```
This will execute all defined tests and display their status.

## 🛠️ Future Enhancements
- **Mais Serviços de Backup:** Support for new cloud storage services.
- **Relatórios de Backup:** Detailed reports on performed backups.
- **Agendamento Automático:** Functionality for scheduling periodic backups.
  
## 📄  License
This project is licensed under the terms of the MIT license. See the LICENSE file for more details.

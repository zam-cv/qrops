# Qrops: A game for Verqor

Qrops is a 2D video game that combines entertainment with education, allowing players to experience the challenges of farming interactively. Through this game, we seek to empower users with knowledge and perspective on agricultural operations.

## Getting Started

### Prerequisites

- [Nix](https://nixos.org/download.html)
- [Docker](https://www.docker.com/get-started)
- [Docker Compose](https://docs.docker.com/compose/install/)
- [Git](https://git-scm.com/downloads)

### Installation

```bash
git clone https://github.com/zam-cv/B4
cd B4
```

#### Windows Installation (WSL)
_PowerShell_
```powershell
wsl --install
wsl --list --online  # Allows enabling available Linux distributions
wsl --install -d Ubuntu-20.04 # How to install an available distribution
ubuntu
sudo apt update
sudo apt upgrade
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh # rust install
sudo apt install libssl-dev pkg-config
nix-env -iA nixpkgs.diesel-cli
```
Note: To access the repository within the Windows ecosystem, we can clone the repository to the folder of your choice within the path /mnt/c/Users/TuNombreDeUsuario/

##### SQL Install

```bash
sudo apt install mysql-server
sudo apt-get install libmysqlclient-dev # Optional if there are SQL issues when running the backend
```

### Running the game

For Development:

```bash
docker run -e MYSQL_ROOT_PASSWORD=root -p 3307:3306 -d mysql:8.0
```

```bash
nix-shell
cd backend
cargo run
```

For Production:

```bash
docker-compose --profile prod up
```

### Using the platform

For Development:

```bash
nix-shell
cd backend
cargo run
```

```bash
nix-shell
cd platform
npm install
npm run dev
```

For Production:

```bash
docker-compose --profile prod up
```

```bash
cd platform
npm install
npm run tauri build
```

### Environment Variables

Example of variables in the backend:

```bash
# backend/.env

MODE=dev
RUST_LOG=debug
HOST=0.0.0.0
PORT=8080
USER_SECRET_KEY=user_secret_key
ADMIN_SECRET_KEY=admin_secret_key
DATABASE_URL=mysql://root:root@127.0.0.1:3307/game
IPINFO_TOKEN=d343be3bf5b846
ADMIN_DEFAULT_EMAIL=test@test.com
ADMIN_DEFAULT_PASSWORD=test
SMTP_HOST=smtp-mail.outlook.com
SMTP_USERNAME=test@outlook.com
SMTP_PASSWORD=test
```

Example of variables in the platform:

```bash
# platform/.env

VITE_APP_SERVER_PROTOCOL = "http"
VITE_APP_SERVER_HOST = "localhost"
VITE_APP_SERVER_PORT = 8080
VITE_APP_API_ROUTE = "api/admin"
```

### Deployment

This is an example of how to deploy the application in a production environment, in this case using Terraform to deploy the infrastructure on Oracle Cloud.

```bash
terraform init
terraform plan
terraform apply
```

## Usage

Once the appropriate environment is set up and running, access the game through the provided web interface. The specific address will depend on your setup but is typically http://localhost:8080 for local development environments or a predefined domain for production setups.

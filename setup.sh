#!/bin/bash

info() {
    echo -e "\033[34m[INFO]\033[0m $1"
}

sucesso() {
    echo -e "\033[32m[SUCESSO]\033[0m $1"
}

erro() {
    echo -e "\033[31m[ERRO]\033[0m $1" >&2
    exit 1
}

set -e

SUDO_USER_REAL=${SUDO_USER:-$(whoami)}

PKG_MANAGER=""
if command -v apt-get &> /dev/null; then
    PKG_MANAGER="apt"
    info "Sistema Debian/Ubuntu detectado. Usando 'apt'."
elif command -v pacman &> /dev/null; then
    PKG_MANAGER="pacman"
    info "Sistema Arch Linux detectado. Usando 'pacman'."
else
    erro "Sistema operacional não suportado. Este script funciona apenas em sistemas com 'apt' ou 'pacman'."
fi

info "Iniciando a instalação do Rust..."
if command -v rustc &> /dev/null; then
    sucesso "Rust já está instalado."
else
    if [ "$PKG_MANAGER" = "apt" ]; then
        apt-get update
        apt-get install -y curl build-essential
    else
        pacman -Syu --noconfirm --needed curl base-devel
    fi

    info "Baixando e instalando o Rust via rustup para o usuário '$SUDO_USER_REAL'..."
    sudo -u "$SUDO_USER_REAL" bash -c "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"
    
    export PATH="/home/$SUDO_USER_REAL/.cargo/bin:$PATH"
    
    sucesso "Rust foi instalado com sucesso."
fi

info "Iniciando a instalação do PostgreSQL..."
if command -v psql &> /dev/null; then
    sucesso "PostgreSQL já está instalado."
else
    case $PKG_MANAGER in
        "apt")
            info "Instalando PostgreSQL via apt..."
            apt-get install -y postgresql postgresql-contrib
            
            info "Iniciando e habilitando o serviço do PostgreSQL..."
            systemctl start postgresql
            systemctl enable postgresql
            ;;
        "pacman")
            info "Instalando PostgreSQL via pacman..."
            pacman -S --noconfirm postgresql
            
            info "Inicializando o cluster de banco de dados do PostgreSQL..."

            sudo -u postgres initdb -D /var/lib/postgres/data
            
            info "Iniciando e habilitando o serviço do PostgreSQL..."
            systemctl start postgresql
            systemctl enable postgresql
            ;;
    esac
    sucesso "PostgreSQL foi instalado e configurado."
fi

echo
sucesso "Instalação concluída!"
info "Para que as mudanças do Rust (rustc, cargo) tenham efeito, feche e abra seu terminal ou execute:"
info "source \$HOME/.cargo/env"
info "Para usar o PostgreSQL, você geralmente precisa mudar para o usuário 'postgres', por exemplo:"
info "sudo -u postgres psql"

cargo build --release

./target/release/alexandria

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Cargo {
    id_cargo: u32,
    titulo: String,
    data_criacao: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Motivo {
    id_motivo: u32,
    nome: String,
    data_criacao: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Diretorio {
    id_diretorio: u32,
    caminho: String,
    modificavel: bool,
    data_criacao: DateTime<Utc>,
    data_modificacao: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Endereco {
    id_endereco: u32,
    cep: String,
    logradouro: String,
    numero: String,
    complemento: Option<String>,
    bairro: String,
    cidade: String,
    estado: String,
    pais: String,
    data_criacao: DateTime<Utc>,
    data_modificacao: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Funcionario {
    id_funcionario: u32,
    nome: String,
    id_cargo: u32,
    email: Option<String>,
    num_telefone: Option<String>,
    username: String,
    pwd_hash: String,
    salt: String,
    data_criacao: DateTime<Utc>,
    data_modificacao: DateTime<Utc>,
    data_desligamento: Option<DateTime<Utc>>,
}

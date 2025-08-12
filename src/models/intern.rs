use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Cargo(String);

impl Cargo {
    pub fn new(cargo: &str) -> Self {
        Self(cargo.to_string())
    }
}

impl PartialEq for Cargo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Motivo(String);

#[derive(Serialize, Deserialize, Debug)]
pub struct Diretorio {
    caminho: String,
    modificavel: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Endereco {
    cep: String,
    logradouro: String,
    numero: u16,
    complemento: Option<String>,
    bairro: String,
    cidade: String,
    estado: String,
    pais: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Funcionario {
    nome: String,
    cargo: Cargo,
    email: Option<String>,
    num_telefone: Option<String>,
    username: String,
}

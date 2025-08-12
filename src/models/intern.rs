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

impl Motivo {
    pub fn new(motivo: &str) -> Self {
        Self(motivo.to_string())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Diretorio {
    caminho: String,
    modificavel: bool,
}

impl Diretorio {
    pub fn new(caminho: &str, modificavel: bool) -> Self {
        Self {
            caminho: caminho.to_string(),
            modificavel,
        }
    }
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

impl Endereco {
    pub fn new(
        cep: &str,
        logradouro: &str,
        numero: u16,
        complemento: Option<String>,
        bairro: &str,
        cidade: &str,
        estado: &str,
        pais: &str,
    ) -> Self {
        Self {
            cep: cep.to_string(),
            logradouro: logradouro.to_string(),
            numero,
            complemento,
            bairro: bairro.to_string(),
            cidade: cidade.to_string(),
            estado: estado.to_string(),
            pais: pais.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Funcionario {
    nome: String,
    cargo: Cargo,
    email: Option<String>,
    num_telefone: Option<String>,
    username: String,
}

use std::sync::Arc;

pub struct Cargo(String);
pub struct Motivo(String);

pub struct Diretorio {
    caminho: String,
    modificavel: bool,
}

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

pub struct Funcionario {
    nome: String,
    cargo: Arc<Cargo>,
    email: Option<String>,
    num_telefone: Option<String>,
    username: String,
}

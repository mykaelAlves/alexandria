use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum MeioAudiencia {
    Remoto,
    Hibrido,
    Presencial,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum StatusReclamacao {
    EmTramitacao,
    Arquivado,
    Desarquivado,
}

#[rustfmt::skip]
#[derive(Serialize, Deserialize, Debug)]
pub enum Uf {
    AC, AL, AP, AM, BA, CE, DF, ES, GO, MA, MT, MS,
    MG, PA, PB, PR, PE, PI, RJ, RN, RS, RO, RR, SC,
    SP, SE, TO,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cargo {
    titulo: String,
}

impl Cargo {
    pub fn new(cargo: &str) -> Self {
        Self {
            titulo: cargo.to_string(),
        }
    }

    pub fn titulo(&self) -> &str {
        &self.titulo
    }
}

impl PartialEq for Cargo {
    fn eq(&self, other: &Self) -> bool {
        self.titulo == other.titulo
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Motivo {
    nome: String,
}

impl Motivo {
    pub fn new(motivo: &str) -> Self {
        Self {
            nome: motivo.to_string(),
        }
    }

    pub fn nome(&self) -> &str {
        &self.nome
    }
}

impl PartialEq for Motivo {
    fn eq(&self, other: &Self) -> bool {
        self.nome == other.nome
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
    numero: String,
    complemento: Option<String>,
    bairro: String,
    cidade: String,
    estado: Uf,
    pais: String,
}

impl Endereco {
    pub fn new(
        cep: &str,
        logradouro: &str,
        numero: &str,
        complemento: Option<String>,
        bairro: &str,
        cidade: &str,
        estado: Uf,
        pais: &str,
    ) -> Self {
        Self {
            cep: cep.to_string(),
            logradouro: logradouro.to_string(),
            numero: numero.to_string(),
            complemento,
            bairro: bairro.to_string(),
            cidade: cidade.to_string(),
            estado,
            pais: pais.to_string(),
        }
    }

    pub fn cep(&self) -> &str {
        &self.cep
    }

    pub fn logradouro(&self) -> &str {
        &self.logradouro
    }

    pub fn numero(&self) -> &str {
        &self.numero
    }

    pub fn bairro(&self) -> &str {
        &self.bairro
    }

    pub fn cidade(&self) -> &str {
        &self.cidade
    }

    pub fn estado(&self) -> &Uf {
        &self.estado
    }

    pub fn pais(&self) -> &str {
        &self.pais
    }

    pub fn complemento(&self) -> Option<&str> {
        self.complemento.as_deref()
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

impl Funcionario {
    pub fn new(
        nome: &str,
        cargo: Cargo,
        email: Option<String>,
        num_telefone: Option<String>,
        username: &str,
    ) -> Self {
        Self {
            nome: nome.to_string(),
            cargo,
            email,
            num_telefone,
            username: username.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Procurador {
    nome: String,
    cpf: String,
    oab: String,
    email: Option<String>,
    num_telefone: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reclamante {
    tipo_pessoa: String,
    nome: String,
    cpf: Option<String>,
    cnpj: Option<String>,
    rg: Option<String>,
    endereco: Endereco,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reclamado {
    tipo_pessoa: String,
    nome: Option<String>,
    razao_social: Option<String>,
    nome_fantasia: Option<String>,
    cpf: Option<String>,
    cnpj: Option<String>,
    email: Option<String>,
    num_telefone: Option<String>,
    endereco: Endereco,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reclamacao {
    numero: i32,
    ano: i32,
    protocolo: String,
    reclamante: Reclamante,
    reclamado: Reclamado,
}

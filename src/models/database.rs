use chrono::{DateTime, Utc};

use crate::models::intern;

#[derive(sqlx::FromRow, Debug)]
pub struct Cargo {
    id_cargo: i64,
    titulo: String,
    data_criacao: DateTime<Utc>,
}

impl Cargo {
    pub fn to_intern(&self) -> intern::Cargo {
        intern::Cargo::new(&self.titulo)
    }
}

impl PartialEq for Cargo {
    fn eq(&self, other: &Self) -> bool {
        self.id_cargo == other.id_cargo
    }
}

#[derive(sqlx::FromRow, Debug)]
pub struct Motivo {
    id_motivo: i64,
    nome: String,
    data_criacao: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct Diretorio {
    id_diretorio: i64,
    caminho: String,
    modificavel: bool,
    data_criacao: DateTime<Utc>,
    data_modificacao: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct Endereco {
    id_endereco: i64,
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

#[derive(sqlx::FromRow, Debug)]
pub struct Funcionario {
    id_funcionario: i64,
    nome: String,
    id_cargo: i64,
    email: Option<String>,
    num_telefone: Option<String>,
    username: String,
    pwd_hash: String,
    salt: String,
    data_criacao: DateTime<Utc>,
    data_modificacao: DateTime<Utc>,
    data_desligamento: Option<DateTime<Utc>>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct Procurador {
    id_procurador: i64,
    nome: String,
    cpf: String,
    oab: String,
    email: Option<String>,
    num_telefone: Option<String>,
    data_criacao: DateTime<Utc>,
    data_modificacao: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct Reclamado {
    id_reclamado: i64,
    tipo_pessoa: char,
    nome: Option<String>,
    razao_social: Option<String>,
    nome_fantasia: Option<String>,
    cpf: Option<String>,
    cnpj: Option<String>,
    email: Option<String>,
    num_telefone: Option<String>,
    id_endereco: i64,
    data_criacao: DateTime<Utc>,
    data_modificacao: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct Reclamacao {
    id_reclamacao: i64,
    id_procurador: i64,
    id_reclamado: i64,
    id_motivo: i64,
    id_diretorio: i64,
    id_cargo: i64,
    status: StatusReclamacao,
    assunto: String,
    descricao: Option<String>,
    data_criacao: DateTime<Utc>,
    data_modificacao: DateTime<Utc>,
}

#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "meio_audiencia_enum")]
pub enum MeioAudiencia {
    Remoto,
    Hibrido,
    Presencial,
}

#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "status_reclamacao_enum")]
pub enum StatusReclamacao {
    EmTramitacao,
    Arquivado,
    Desarquivado,
}

#[derive(sqlx::FromRow, Debug)]
pub struct Audiencia {
    id_audiencia: i64,
    id_conciliador: i64,
    data: DateTime<Utc>,
    meio: MeioAudiencia,
    data_criacao: DateTime<Utc>,
    data_modificacao: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct HistoricoStatusReclamacao {
    id_reclamacao: i64,
    status_old: StatusReclamacao,
    status_new: StatusReclamacao,
    data_mudanca: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct RelacaoReclamacaoAudiencia {
    id_reclamacao: i64,
    id_audiencia: i64,
}

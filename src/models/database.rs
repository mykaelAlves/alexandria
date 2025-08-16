use chrono::{DateTime, Utc};

use crate::models::intern;

#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "meio_audiencia_enum", rename_all = "PascalCase")]
pub enum MeioAudiencia {
    Remoto,
    Hibrido,
    Presencial,
}

#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "status_reclamacao_enum", rename_all = "PascalCase")]
pub enum StatusReclamacao {
    EmTramitacao,
    Arquivado,
    Desarquivado,
}

#[rustfmt::skip]
#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "uf_enum", rename_all = "UPPERCASE")]
pub enum Uf {
    AC, AL, AP, AM, BA, CE, DF, ES, GO, MA, MT, MS,
    MG, PA, PB, PR, PE, PI, RJ, RN, RS, RO, RR, SC,
    SP, SE, TO,
}

#[derive(sqlx::FromRow, Debug)]
pub struct Cargo {
    id_cargo: i32,
    titulo: String,
    data_criacao: DateTime<Utc>,
}

impl Cargo {
    pub fn to_intern(&self) -> intern::Cargo {
        intern::Cargo::new(&self.titulo)
    }

    pub fn id_cargo(&self) -> i32 {
        self.id_cargo
    }

    pub fn titulo(&self) -> &str {
        &self.titulo
    }

    pub fn data_criacao(&self) -> DateTime<Utc> {
        self.data_criacao
    }
}

impl PartialEq for Cargo {
    fn eq(&self, other: &Self) -> bool {
        self.id_cargo == other.id_cargo
    }
}

#[derive(sqlx::FromRow, Debug)]
pub struct Motivo {
    id_motivo: i32,
    nome: String,
    data_criacao: DateTime<Utc>,
}

impl Motivo {
    pub fn to_intern(&self) -> intern::Motivo {
        intern::Motivo::new(&self.nome)
    }

    pub fn id_motivo(&self) -> i32 {
        self.id_motivo
    }

    pub fn nome(&self) -> &str {
        &self.nome
    }

    pub fn data_criacao(&self) -> DateTime<Utc> {
        self.data_criacao
    }
}

impl PartialEq for Motivo {
    fn eq(&self, other: &Self) -> bool {
        self.id_motivo == other.id_motivo
    }
}

#[derive(sqlx::FromRow, Debug)]
pub struct Diretorio {
    id_diretorio: i32,
    caminho: String,
    modificavel: bool,
    data_criacao: DateTime<Utc>,
    data_modificacao: DateTime<Utc>,
}

impl Diretorio {
    pub fn id_diretorio(&self) -> i32 {
        self.id_diretorio
    }
    pub fn caminho(&self) -> &str {
        &self.caminho
    }
    pub fn modificavel(&self) -> bool {
        self.modificavel
    }
    pub fn data_criacao(&self) -> DateTime<Utc> {
        self.data_criacao
    }
    pub fn data_modificacao(&self) -> DateTime<Utc> {
        self.data_modificacao
    }
}

#[derive(sqlx::FromRow, Debug)]
pub struct Endereco {
    id_endereco: i32,
    cep: String,
    logradouro: String,
    numero: String,
    complemento: Option<String>,
    bairro: String,
    cidade: String,
    estado: Uf,
    pais: String,
    data_criacao: DateTime<Utc>,
    data_modificacao: DateTime<Utc>,
}

impl Endereco {
    pub fn id_endereco(&self) -> i32 {
        self.id_endereco
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
    pub fn complemento(&self) -> Option<&str> {
        self.complemento.as_deref()
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
    pub fn data_criacao(&self) -> DateTime<Utc> {
        self.data_criacao
    }
    pub fn data_modificacao(&self) -> DateTime<Utc> {
        self.data_modificacao
    }
}

#[derive(sqlx::FromRow, Debug)]
pub struct Funcionario {
    id_funcionario: i32,
    nome: String,
    id_cargo: i32,
    email: Option<String>,
    num_telefone: Option<String>,
    username: String,
    pwd_hash: String,
    salt: String,
    data_criacao: DateTime<Utc>,
    data_modificacao: DateTime<Utc>,
    data_desligamento: Option<DateTime<Utc>>,
}

impl Funcionario {
    pub fn id_funcionario(&self) -> i32 {
        self.id_funcionario
    }
    pub fn nome(&self) -> &str {
        &self.nome
    }
    pub fn id_cargo(&self) -> i32 {
        self.id_cargo
    }
    pub fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }
    pub fn num_telefone(&self) -> Option<&str> {
        self.num_telefone.as_deref()
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn pwd_hash(&self) -> &str {
        &self.pwd_hash
    }
    pub fn salt(&self) -> &str {
        &self.salt
    }
    pub fn data_criacao(&self) -> DateTime<Utc> {
        self.data_criacao
    }
    pub fn data_modificacao(&self) -> DateTime<Utc> {
        self.data_modificacao
    }
    pub fn data_desligamento(&self) -> Option<DateTime<Utc>> {
        self.data_desligamento
    }
}

#[derive(sqlx::FromRow, Debug)]
pub struct Procurador {
    id_procurador: i32,
    nome: String,
    cpf: String,
    oab: String,
    email: Option<String>,
    num_telefone: Option<String>,
    data_criacao: DateTime<Utc>,
    data_modificacao: DateTime<Utc>,
}

impl Procurador {
    pub fn id_procurador(&self) -> i32 {
        self.id_procurador
    }
    pub fn nome(&self) -> &str {
        &self.nome
    }
    pub fn cpf(&self) -> &str {
        &self.cpf
    }
    pub fn oab(&self) -> &str {
        &self.oab
    }
    pub fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }
    pub fn num_telefone(&self) -> Option<&str> {
        self.num_telefone.as_deref()
    }
    pub fn data_criacao(&self) -> DateTime<Utc> {
        self.data_criacao
    }
    pub fn data_modificacao(&self) -> DateTime<Utc> {
        self.data_modificacao
    }
}

#[derive(sqlx::FromRow, Debug)]
pub struct Reclamante {
    id_reclamante: i32,
    tipo_pessoa: String,
    nome: String,
    cpf: Option<String>,
    cnpj: Option<String>,
    rg: Option<String>,
    id_endereco: i32,
    data_criacao: DateTime<Utc>,
    data_modificacao: DateTime<Utc>,
}

impl Reclamante {
    pub fn id_reclamante(&self) -> i32 {
        self.id_reclamante
    }
    pub fn tipo_pessoa(&self) -> &str {
        &self.tipo_pessoa
    }
    pub fn nome(&self) -> &str {
        &self.nome
    }
    pub fn cpf(&self) -> Option<&str> {
        self.cpf.as_deref()
    }
    pub fn cnpj(&self) -> Option<&str> {
        self.cnpj.as_deref()
    }
    pub fn rg(&self) -> Option<&str> {
        self.rg.as_deref()
    }
    pub fn id_endereco(&self) -> i32 {
        self.id_endereco
    }
    pub fn data_criacao(&self) -> DateTime<Utc> {
        self.data_criacao
    }
    pub fn data_modificacao(&self) -> DateTime<Utc> {
        self.data_modificacao
    }
}

#[derive(sqlx::FromRow, Debug)]
pub struct Reclamado {
    id_reclamado: i32,
    tipo_pessoa: char,
    nome: Option<String>,
    razao_social: Option<String>,
    nome_fantasia: Option<String>,
    cpf: Option<String>,
    cnpj: Option<String>,
    email: Option<String>,
    num_telefone: Option<String>,
    id_endereco: i32,
    data_criacao: DateTime<Utc>,
    data_modificacao: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct Reclamacao {
    id_reclamacao: i32,
    numero: i32,
    ano: i32,
    protocolo: String,
    id_reclamante: i32,
    id_motivo: i32,
    id_procurador: Option<i32>,
    observacao: Option<String>,
    atendido: Option<bool>,
    id_criador: i32,
    status: StatusReclamacao,
    id_diretorio: i32,
    data_criacao: DateTime<Utc>,
    data_modificacao: DateTime<Utc>,
}

impl Reclamacao {
    pub fn id_reclamacao(&self) -> i32 {
        self.id_reclamacao
    }
    pub fn numero(&self) -> i32 {
        self.numero
    }
    pub fn ano(&self) -> i32 {
        self.ano
    }
    pub fn protocolo(&self) -> &str {
        &self.protocolo
    }
    pub fn id_reclamante(&self) -> i32 {
        self.id_reclamante
    }
    pub fn id_motivo(&self) -> i32 {
        self.id_motivo
    }
    pub fn id_procurador(&self) -> Option<i32> {
        self.id_procurador
    }
    pub fn observacao(&self) -> Option<&str> {
        self.observacao.as_deref()
    }
    pub fn atendido(&self) -> Option<bool> {
        self.atendido
    }
    pub fn id_criador(&self) -> i32 {
        self.id_criador
    }
    pub fn status(&self) -> &StatusReclamacao {
        &self.status
    }
    pub fn id_diretorio(&self) -> i32 {
        self.id_diretorio
    }
    pub fn data_criacao(&self) -> DateTime<Utc> {
        self.data_criacao
    }
    pub fn data_modificacao(&self) -> DateTime<Utc> {
        self.data_modificacao
    }
}

#[derive(sqlx::FromRow, Debug)]
pub struct Audiencia {
    id_audiencia: i32,
    id_conciliador: i32,
    data: DateTime<Utc>,
    meio: Option<MeioAudiencia>,
    data_criacao: DateTime<Utc>,
    data_modificacao: DateTime<Utc>,
}

impl Audiencia {
    pub fn id_audiencia(&self) -> i32 {
        self.id_audiencia
    }
    pub fn id_conciliador(&self) -> i32 {
        self.id_conciliador
    }
    pub fn data(&self) -> DateTime<Utc> {
        self.data
    }
    pub fn meio(&self) -> Option<&MeioAudiencia> {
        self.meio.as_ref()
    }
    pub fn data_criacao(&self) -> DateTime<Utc> {
        self.data_criacao
    }
    pub fn data_modificacao(&self) -> DateTime<Utc> {
        self.data_modificacao
    }
}

#[derive(sqlx::FromRow, Debug)]
pub struct HistoricoStatusReclamacao {
    id_historico: i64,
    id_reclamacao: i32,
    status_anterior: StatusReclamacao,
    status_novo: StatusReclamacao,
    data_mudanca: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct RelacaoReclamacaoAudiencia {
    id_reclamacao: i32,
    id_audiencia: i32,
}

impl RelacaoReclamacaoAudiencia {
    pub fn id_reclamacao(&self) -> i32 {
        self.id_reclamacao
    }

    pub fn id_audiencia(&self) -> i32 {
        self.id_audiencia
    }

    pub fn has_relation_with(
        &self,
        relacao: &RelacaoReclamacaoAudiencia,
    ) -> bool {
        self.id_reclamacao == relacao.id_reclamacao
            && self.id_audiencia == relacao.id_audiencia
    }
}

impl PartialEq for RelacaoReclamacaoAudiencia {
    fn eq(&self, other: &Self) -> bool {
        self.id_reclamacao == other.id_reclamacao
            && self.id_audiencia == other.id_audiencia
    }
}

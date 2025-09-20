//! Modelos utilizados para empacotar dados entre a
//! aplicação e o banco de dados, e transmissão de dados
//! pela rede.

use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::database;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum MeioAudiencia {
	Remoto,
	Hibrido,
	Presencial,
}

impl From<&database::MeioAudiencia> for MeioAudiencia {
	fn from(meio: &database::MeioAudiencia) -> Self {
		match meio {
			database::MeioAudiencia::Remoto => MeioAudiencia::Remoto,
			database::MeioAudiencia::Hibrido => MeioAudiencia::Hibrido,
			database::MeioAudiencia::Presencial => MeioAudiencia::Presencial,
		}
	}
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum StatusReclamacao {
	EmTramitacao,
	Arquivado,
	Desarquivado,
}

impl From<&database::StatusReclamacao> for StatusReclamacao {
	fn from(status: &database::StatusReclamacao) -> Self {
		match status {
			database::StatusReclamacao::EmTramitacao => {
				StatusReclamacao::EmTramitacao
			}
			database::StatusReclamacao::Arquivado => {
				StatusReclamacao::Arquivado
			}
			database::StatusReclamacao::Desarquivado => {
				StatusReclamacao::Desarquivado
			}
		}
	}
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum TipoPessoa {
	Fisica,
	Juridica,
}

impl From<&database::TipoPessoa> for TipoPessoa {
	fn from(tipo: &database::TipoPessoa) -> Self {
		match tipo {
			database::TipoPessoa::Fisica => TipoPessoa::Fisica,
			database::TipoPessoa::Juridica => TipoPessoa::Juridica,
		}
	}
}

#[rustfmt::skip]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Uf {
    AC, AL, AP, AM, BA, CE, DF, ES, GO, MA, MT, MS,
    MG, PA, PB, PR, PE, PI, RJ, RN, RS, RO, RR, SC,
    SP, SE, TO,
}

#[rustfmt::skip]
impl From<&database::Uf> for Uf {
    fn from(uf: &database::Uf) -> Self {
        match uf {
            database::Uf::AC => Uf::AC, database::Uf::AL => Uf::AL,
            database::Uf::AP => Uf::AP, database::Uf::AM => Uf::AM,
            database::Uf::BA => Uf::BA, database::Uf::CE => Uf::CE,
            database::Uf::DF => Uf::DF, database::Uf::ES => Uf::ES,
            database::Uf::GO => Uf::GO, database::Uf::MA => Uf::MA,
            database::Uf::MT => Uf::MT, database::Uf::MS => Uf::MS,
            database::Uf::MG => Uf::MG, database::Uf::PA => Uf::PA,
            database::Uf::PB => Uf::PB, database::Uf::PR => Uf::PR,
            database::Uf::PE => Uf::PE, database::Uf::PI => Uf::PI,
            database::Uf::RJ => Uf::RJ, database::Uf::RN => Uf::RN,
            database::Uf::RS => Uf::RS, database::Uf::RO => Uf::RO,
            database::Uf::RR => Uf::RR, database::Uf::SC => Uf::SC,
            database::Uf::SP => Uf::SP, database::Uf::SE => Uf::SE,
            database::Uf::TO => Uf::TO,
        }
    }
}

#[rustfmt::skip]
impl Display for Uf {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let uf_str = match self {
			Uf::AC => "AC", Uf::AL => "AL", Uf::AP => "AP",
			Uf::AM => "AM", Uf::BA => "BA", Uf::CE => "CE",
			Uf::DF => "DF", Uf::ES => "ES", Uf::GO => "GO",
			Uf::MA => "MA", Uf::MT => "MT", Uf::MS => "MS",
			Uf::MG => "MG", Uf::PA => "PA", Uf::PB => "PB",
			Uf::PR => "PR", Uf::PE => "PE", Uf::PI => "PI",
			Uf::RJ => "RJ", Uf::RN => "RN", Uf::RS => "RS",
			Uf::RO => "RO", Uf::RR => "RR", Uf::SC => "SC",
			Uf::SP => "SP", Uf::SE => "SE", Uf::TO => "TO",
		};
		write!(f, "{uf_str}")
	}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cargo {
	pub titulo: String,
}

impl Cargo {
	pub fn new(titulo: &str) -> Self {
		Self {
			titulo: titulo.to_string(),
		}
	}
}

impl From<&database::Cargo> for Cargo {
	fn from(cargo: &database::Cargo) -> Self {
		Self {
			titulo: cargo.titulo.clone(),
		}
	}
}

impl From<&str> for Cargo {
	fn from(titulo: &str) -> Self {
		Self {
			titulo: titulo.to_string(),
		}
	}
}

impl From<String> for Cargo {
	fn from(titulo: String) -> Self {
		Self { titulo }
	}
}

impl From<&Cargo> for String {
	fn from(cargo: &Cargo) -> Self {
		cargo.titulo.clone()
	}
}

impl PartialEq for Cargo {
	fn eq(&self, other: &Self) -> bool {
		self.titulo == other.titulo
	}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Motivo {
	pub nome: String,
	pub artigo: i16,
	pub paragrafo_unico: bool,
	pub inciso: Option<i16>,
}

impl PartialEq for Motivo {
	fn eq(&self, other: &Self) -> bool {
		self.nome == other.nome
	}
}

impl From<&database::Motivo> for Motivo {
	fn from(motivo: &database::Motivo) -> Self {
		Self {
			nome: motivo.nome.clone(),
			artigo: motivo.artigo,
			paragrafo_unico: motivo.paragrafo_unico,
			inciso: motivo.inciso,
		}
	}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Diretorio {
	pub caminho: String,
	pub modificavel: bool,
}

impl PartialEq for Diretorio {
	fn eq(&self, other: &Self) -> bool {
		self.caminho == other.caminho
	}
}

impl From<&database::Diretorio> for Diretorio {
	fn from(diretorio: &database::Diretorio) -> Self {
		Self {
			caminho: diretorio.caminho.clone(),
			modificavel: diretorio.modificavel,
		}
	}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Endereco {
	pub cep: String,
	pub logradouro: String,
	pub numero: String,
	pub complemento: Option<String>,
	pub bairro: String,
	pub cidade: String,
	pub estado: Uf,
	pub pais: String,
}

impl PartialEq for Endereco {
	fn eq(&self, other: &Self) -> bool {
		self.cep == other.cep
			&& self.logradouro == other.logradouro
			&& self.numero == other.numero
			&& self.complemento == other.complemento
			&& self.bairro == other.bairro
			&& self.cidade == other.cidade
			&& self.estado == other.estado
			&& self.pais == other.pais
	}
}

impl From<&database::Endereco> for Endereco {
	fn from(endereco: &database::Endereco) -> Self {
		Self {
			cep: endereco.cep.clone(),
			logradouro: endereco.logradouro.clone(),
			numero: endereco.numero.clone(),
			complemento: endereco.complemento.clone(),
			bairro: endereco.bairro.clone(),
			cidade: endereco.cidade.clone(),
			estado: Uf::from(&endereco.estado),
			pais: endereco.pais.clone(),
		}
	}
}

impl From<&Endereco> for String {
	fn from(endereco: &Endereco) -> Self {
		format!(
			"{}, {}, {}, {}. {} - {}. {}. CEP: {}",
			endereco.logradouro,
			endereco.numero,
			endereco.complemento.as_deref().unwrap_or(""),
			endereco.bairro,
			endereco.cidade,
			endereco.estado,
			endereco.pais,
			endereco.cep
		)
	}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Funcionario {
	pub nome: String,
	pub cargo: Cargo,
	pub email: Option<String>,
	pub num_telefone: Option<String>,
	pub username: String,
	pub ativo: bool,
}

impl TryFrom<(&database::Funcionario, &database::Cargo)> for Funcionario {
	type Error = &'static str;

	fn try_from(
		(funcionario, cargo): (&database::Funcionario, &database::Cargo),
	) -> Result<Self, Self::Error> {
		if funcionario.id_cargo.0 != cargo.id_cargo.0 {
			return Err("Cargo não corresponde ao funcionário");
		}

		Ok(Self {
			nome: funcionario.nome.clone(),
			cargo: Cargo::from(cargo),
			email: funcionario.email.clone(),
			num_telefone: funcionario.num_telefone.clone(),
			username: funcionario.username.clone(),
			ativo: funcionario.data_desligamento.is_none(),
		})
	}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Procurador {
	pub nome: String,
	pub cpf: String,
	pub endereco: Endereco,
	pub email: Option<String>,
	pub num_telefone: Option<String>,
}

impl TryFrom<(&database::Procurador, &database::Endereco)> for Procurador {
	type Error = &'static str;

	fn try_from(
		(procurador, endereco): (&database::Procurador, &database::Endereco),
	) -> Result<Self, Self::Error> {
		if procurador.id_endereco.0 != endereco.id_endereco.0 {
			return Err("Endereço não corresponde ao procurador");
		}
		
		Ok(Self {
			nome: procurador.nome.clone(),
			cpf: procurador.cpf.clone(),
			endereco: Endereco::from(endereco),
			email: procurador.email.clone(),
			num_telefone: procurador.num_telefone.clone(),
		})
	}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reclamante {
	pub tipo_pessoa: TipoPessoa,
	pub nome: String,
	pub cpf: Option<String>,
	pub cnpj: Option<String>,
	pub endereco: Endereco,
}

impl TryFrom<(&database::Reclamante, &database::Endereco)> for Reclamante {
	type Error = &'static str;

	fn try_from(
		(reclamante, endereco): (&database::Reclamante, &database::Endereco),
	) -> Result<Self, Self::Error> {
		if reclamante.tipo_pessoa == database::TipoPessoa::Fisica
			&& reclamante.cpf.is_none()
		{
			return Err("Reclamante pessoa física deve ter CPF");
		}

		if reclamante.tipo_pessoa == database::TipoPessoa::Juridica
			&& reclamante.cnpj.is_none()
		{
			return Err("Reclamante pessoa jurídica deve ter CNPJ");
		}

		if reclamante.id_endereco.0 != endereco.id_endereco.0 {
			return Err("Endereço não corresponde ao reclamante");
		}

		Ok(Self {
			tipo_pessoa: TipoPessoa::from(&reclamante.tipo_pessoa),
			nome: reclamante.nome.clone(),
			cpf: reclamante.cpf.clone(),
			cnpj: reclamante.cnpj.clone(),
			endereco: endereco.into(),
		})
	}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reclamado {
	pub tipo_pessoa: TipoPessoa,
	pub nome: String,
	pub razao_social: Option<String>,
	pub nome_fantasia: Option<String>,
	pub cpf: Option<String>,
	pub cnpj: Option<String>,
	pub email: Option<String>,
	pub num_telefone: Option<String>,
	pub endereco: Endereco,
}

impl TryFrom<(&database::Reclamado, &database::Endereco)> for Reclamado {
	type Error = &'static str;

	fn try_from(
		(reclamado, endereco): (&database::Reclamado, &database::Endereco),
	) -> Result<Self, Self::Error> {
		if reclamado.tipo_pessoa == database::TipoPessoa::Fisica
			&& reclamado.cpf.is_none()
		{
			return Err("Reclamado pessoa física deve ter CPF");
		}

		if reclamado.tipo_pessoa == database::TipoPessoa::Juridica
			&& reclamado.cnpj.is_none()
		{
			return Err("Reclamado pessoa jurídica deve ter CNPJ");
		}

		if reclamado.id_endereco.0 != endereco.id_endereco.0 {
			return Err("Endereço não corresponde ao reclamado");
		}

		Ok(Self {
			tipo_pessoa: TipoPessoa::from(&reclamado.tipo_pessoa),
			nome: reclamado.nome.clone().unwrap_or_default(),
			razao_social: reclamado.razao_social.clone(),
			nome_fantasia: reclamado.nome_fantasia.clone(),
			cpf: reclamado.cpf.clone(),
			cnpj: reclamado.cnpj.clone(),
			email: reclamado.email.clone(),
			num_telefone: reclamado.num_telefone.clone(),
			endereco: Endereco::from(endereco),
		})
	}
}

impl PartialEq for Reclamado {
	fn eq(&self, other: &Self) -> bool {
		match (&self.cpf, &self.cnpj, &other.cpf, &other.cnpj) {
			(Some(cpf1), _, Some(cpf2), _) => {
				cpf1 == cpf2 && self.nome_fantasia == other.nome_fantasia
			}
			(_, Some(cnpj1), _, Some(cnpj2)) => cnpj1 == cnpj2,
			_ => false,
		}
	}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Audiencia {
	pub conciliador: Funcionario,
	pub data_hora: DateTime<Utc>,
	pub meio: MeioAudiencia,
}

impl From<(&database::Audiencia, Funcionario)> for Audiencia {
	fn from(
		(audiencia, funcionario): (&database::Audiencia, Funcionario),
	) -> Self {
		Self {
			conciliador: funcionario,
			data_hora: audiencia.data_hora,
			meio: MeioAudiencia::from(&audiencia.meio),
		}
	}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Protocolo {
	pub numero: i32,
	pub ano: i32,
}

impl PartialEq for Protocolo {
	fn eq(&self, other: &Self) -> bool {
		self.numero == other.numero && self.ano == other.ano
	}
}

impl TryFrom<&str> for Protocolo {
	type Error = &'static str;

	fn try_from(s: &str) -> Result<Self, &'static str> {
		let parts: Vec<&str> = s.split('-').collect();
		if parts.len() != 2 {
			return Err("Formato inválido");
		}

		let numero = parts[0].parse::<i32>().map_err(|_| "Número inválido")?;
		let ano = parts[1].parse::<i32>().map_err(|_| "Ano inválido")?;

		Ok(Self { numero, ano })
	}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reclamacao {
	pub protocolo: Protocolo,
	pub reclamante: Reclamante,
	pub reclamados: Vec<Reclamado>,
	pub audiencias: Vec<Audiencia>,
	pub motivo: Motivo,
	pub procurador: Option<Procurador>,
	pub observacao: Option<String>,
	pub atendido: Option<bool>,
	pub criador: Funcionario,
	pub status: StatusReclamacao,
	pub diretorio: Diretorio,
}

impl Reclamacao {
	pub fn audiencia_marcada_at(&self, data_hora: DateTime<Utc>) -> bool {
		self.audiencias.iter().any(|a| a.data_hora == data_hora)
	}

	pub fn has_reclamado(&self, reclamado: &Reclamado) -> bool {
		self.reclamados.iter().any(|r| r == reclamado)
	}

	pub fn protocolo_string(&self) -> String {
		format!("{:04}-{}", self.protocolo.numero, self.protocolo.ano)
	}
}

impl PartialEq for Reclamacao {
	fn eq(&self, other: &Self) -> bool {
		self.protocolo == other.protocolo
	}
}

//
// Test
//

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_cargo_from_str() {
		let cargo: Cargo = "Assessor".into();
		assert_eq!(cargo.titulo, "Assessor");
	}

	#[test]
	fn test_cargo_from_string() {
		let cargo: Cargo = String::from("Conciliador").into();
		assert_eq!(cargo.titulo, "Conciliador");
	}

	#[test]
	fn test_cargo_into_string() {
		let cargo = Cargo::new("Auxiliar");
		let titulo: String = String::from(&cargo);
		assert_eq!(titulo, "Auxiliar");
	}

	#[test]
	fn test_cargo_partial_eq() {
		let a = Cargo::new("Escrivão");
		let b = Cargo::new("Escrivão");
		let c = Cargo::new("Auxiliar");
		assert_eq!(a, b);
		assert_ne!(a, c);
	}

	#[test]
	fn test_motivo_partial_eq_by_nome() {
		let a = Motivo {
			nome: "Juros Exorbitantes".into(),
			artigo: 1,
			paragrafo_unico: false,
			inciso: Some(2),
		};
		let b = Motivo {
			nome: "Juros Exorbitantes".into(),
			artigo: 99,
			paragrafo_unico: true,
			inciso: None,
		};
		assert_eq!(a, b);
	}

	#[test]
	fn test_motivo_partial_ne_by_nome() {
		let a = Motivo {
			nome: "Motivo A".into(),
			artigo: 1,
			paragrafo_unico: false,
			inciso: None,
		};
		let b = Motivo {
			nome: "Motivo B".into(),
			artigo: 1,
			paragrafo_unico: false,
			inciso: None,
		};
		assert_ne!(a, b);
	}

	#[test]
	fn test_diretorio_partial_eq_by_caminho() {
		let a = Diretorio {
			caminho: "/var/data".into(),
			modificavel: true,
		};
		let b = Diretorio {
			caminho: "/var/data".into(),
			modificavel: false,
		};
		assert_eq!(a, b);
	}

	#[test]
	fn test_diretorio_partial_ne_by_caminho() {
		let a = Diretorio {
			caminho: "/var/data/a".into(),
			modificavel: true,
		};
		let b = Diretorio {
			caminho: "/var/data/b".into(),
			modificavel: true,
		};
		assert_ne!(a, b);
	}

	#[test]
	fn test_uf_display() {
		assert_eq!(format!("{}", Uf::SP), "SP");
		assert_eq!(format!("{}", Uf::RJ), "RJ");
		assert_eq!(format!("{}", Uf::RS), "RS");
		assert_eq!(format!("{}", Uf::BA), "BA");
	}

	fn mk_endereco(complemento: Option<&str>, estado: Uf) -> Endereco {
		Endereco {
			cep: "12345-678".into(),
			logradouro: "Rua A".into(),
			numero: "100".into(),
			complemento: complemento.map(|s| s.to_string()),
			bairro: "Centro".into(),
			cidade: "São Paulo".into(),
			estado,
			pais: "Brasil".into(),
		}
	}

	#[test]
	fn test_endereco_partial_eq() {
		let a = mk_endereco(Some("Apto 1"), Uf::SP);
		let b = mk_endereco(Some("Apto 1"), Uf::SP);
		assert_eq!(a, b);
	}

	#[test]
	fn test_endereco_partial_ne_when_field_differs() {
		let mut a = mk_endereco(Some("Apto 1"), Uf::SP);
		let mut b = mk_endereco(Some("Apto 1"), Uf::SP);
		b.numero = "101".into();
		assert_ne!(a, b);

		a = mk_endereco(None, Uf::SP);
		b = mk_endereco(Some("Apto 1"), Uf::SP);
		assert_ne!(a, b);
	}

	#[test]
	fn test_endereco_to_string_with_complemento() {
		let e = mk_endereco(Some("Apto 1"), Uf::SP);
		let s: String = String::from(&e);
		assert_eq!(
			s,
			"Rua A, 100, Apto 1, Centro. São Paulo - SP. Brasil. CEP: \
			 12345-678"
		);
	}

	#[test]
	fn test_endereco_to_string_without_complemento() {
		let e = mk_endereco(None, Uf::SP);
		let s: String = String::from(&e);

		assert_eq!(
			s,
			"Rua A, 100, Centro. São Paulo - SP. Brasil. CEP: 12345-678"
		);
	}

	#[test]
	fn test_enums_basic_equality() {
		assert_eq!(MeioAudiencia::Remoto, MeioAudiencia::Remoto);
		assert_ne!(MeioAudiencia::Remoto, MeioAudiencia::Presencial);

		assert_eq!(StatusReclamacao::Arquivado, StatusReclamacao::Arquivado);
		assert_ne!(StatusReclamacao::Arquivado, StatusReclamacao::EmTramitacao);

		assert_eq!(TipoPessoa::Fisica, TipoPessoa::Fisica);
		assert_ne!(TipoPessoa::Fisica, TipoPessoa::Juridica);
	}
}

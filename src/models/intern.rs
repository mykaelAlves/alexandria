//! Modelos utilizados para empacotar dados entre a
//! aplicação e o banco de dados, e transmissão de dados
//! pela rede.

use std::fmt::Display;

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

pub struct Funcionario {
	pub nome: String,
	pub cargo: Cargo,
	pub email: Option<String>,
	pub num_telefone: Option<String>,
	pub username: String,
	pub ativo: bool,
}

impl From<(&database::Funcionario, &database::Cargo)> for Funcionario {
	fn from(
		(funcionario, cargo): (
			&database::Funcionario,
			&database::Cargo,
		),
	) -> Self {
		Self {
			nome: funcionario.nome.clone(),
			cargo: Cargo::from(cargo),
			email: funcionario.email.clone(),
			num_telefone: funcionario.num_telefone.clone(),
			username: funcionario.username.clone(),
			ativo: funcionario.data_desligamento.is_none(),
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_cargo_from_str() {
		let cargo: Cargo = "Desenvolvedor".into();
		assert_eq!(cargo.titulo, "Desenvolvedor");
	}
}

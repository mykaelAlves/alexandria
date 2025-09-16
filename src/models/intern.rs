//! Modelos utilizados para empacotar dados entre a
//! aplicação e o banco de dados, e transmissão de dados
//! pela rede.

use serde::{Deserialize, Serialize};

use crate::models::database;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MeioAudiencia {
	Remoto,
	Hibrido,
	Presencial,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StatusReclamacao {
	EmTramitacao,
	Arquivado,
	Desarquivado,
}

#[rustfmt::skip]
#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_cargo_from_str() {
		let cargo: Cargo = "Desenvolvedor".into();
		assert_eq!(cargo.titulo, "Desenvolvedor");
	}
}

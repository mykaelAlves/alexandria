//! Modelos que representam diretamente as estruturas do
//! banco de dados.

use chrono::{DateTime, Utc};

#[derive(sqlx::Type, PartialEq, Debug, Clone, Copy)]
#[sqlx(type_name = "meio_audiencia_enum", rename_all = "PascalCase")]
pub enum MeioAudiencia {
	Remoto,
	Hibrido,
	Presencial,
}

#[derive(sqlx::Type, PartialEq, Debug, Clone, Copy)]
#[sqlx(type_name = "status_reclamacao_enum", rename_all = "PascalCase")]
pub enum StatusReclamacao {
	EmTramitacao,
	Arquivado,
	Desarquivado,
}

#[derive(sqlx::Type, PartialEq, Debug, Clone, Copy)]
#[sqlx(type_name = "tipo_pessoa_enum", rename_all = "PascalCase")]
pub enum TipoPessoa {
	Fisica,
	Juridica,
}

#[rustfmt::skip]
#[derive(sqlx::Type, PartialEq, Debug, Clone, Copy)]
#[sqlx(type_name = "uf_enum", rename_all = "UPPERCASE")]
pub enum Uf {
    AC, AL, AP, AM, BA, CE, DF, ES, GO, MA, MT, MS,
    MG, PA, PB, PR, PE, PI, RJ, RN, RS, RO, RR, SC,
    SP, SE, TO,
}

#[derive(sqlx::Type, Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[sqlx(transparent)]
pub struct CargoId(pub i32);

#[readonly::make]
#[derive(sqlx::FromRow, Debug)]
pub struct Cargo {
	pub id_cargo: CargoId,
	pub titulo: String,
	pub data_criacao: DateTime<Utc>,
}

impl PartialEq for Cargo {
	fn eq(&self, other: &Self) -> bool {
		self.id_cargo == other.id_cargo
	}
}

#[derive(sqlx::Type, Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[sqlx(transparent)]
pub struct MotivoId(pub i32);

#[readonly::make]
#[derive(sqlx::FromRow, Debug)]
pub struct Motivo {
	pub id_motivo: MotivoId,
	pub nome: String,
	pub artigo: i16,
	pub paragrafo_unico: bool,
	pub inciso: Option<i16>,
	pub data_criacao: DateTime<Utc>,
	pub data_modificacao: DateTime<Utc>,
}

impl PartialEq for Motivo {
	fn eq(&self, other: &Self) -> bool {
		self.id_motivo == other.id_motivo
	}
}

#[derive(sqlx::Type, Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[sqlx(transparent)]
pub struct DiretorioId(pub i32);

#[readonly::make]
#[derive(sqlx::FromRow, Debug)]
pub struct Diretorio {
	pub id_diretorio: DiretorioId,
	pub caminho: String,
	pub modificavel: bool,
	pub data_criacao: DateTime<Utc>,
	pub data_modificacao: DateTime<Utc>,
}

#[derive(sqlx::Type, Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[sqlx(transparent)]
pub struct EnderecoId(pub i32);

#[readonly::make]
#[derive(sqlx::FromRow, Debug)]
pub struct Endereco {
	pub id_endereco: EnderecoId,
	pub cep: String,
	pub logradouro: String,
	pub numero: String,
	pub complemento: Option<String>,
	pub bairro: String,
	pub cidade: String,
	pub estado: Uf,
	pub pais: String,
	pub data_criacao: DateTime<Utc>,
	pub data_modificacao: DateTime<Utc>,
}

#[derive(sqlx::Type, Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[sqlx(transparent)]
pub struct FuncionarioId(pub i32);

#[readonly::make]
#[derive(sqlx::FromRow, Debug)]
pub struct Funcionario {
	pub id_funcionario: FuncionarioId,
	pub nome: String,
	pub id_cargo: CargoId,
	pub email: Option<String>,
	pub num_telefone: Option<String>,
	pub username: String,
	pub pwd_hash: String,
	pub salt: String,
	pub data_criacao: DateTime<Utc>,
	pub data_modificacao: DateTime<Utc>,
	pub data_desligamento: Option<DateTime<Utc>>,
}

impl PartialEq for Funcionario {
	fn eq(&self, other: &Self) -> bool {
		self.id_funcionario == other.id_funcionario
	}
}

#[derive(sqlx::Type, Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[sqlx(transparent)]
pub struct ProcuradorId(pub i32);

#[readonly::make]
#[derive(sqlx::FromRow, Debug)]
pub struct Procurador {
	pub id_procurador: ProcuradorId,
	pub nome: String,
	pub cpf: String,
	pub id_endereco: EnderecoId,
	pub email: Option<String>,
	pub num_telefone: Option<String>,
	pub data_criacao: DateTime<Utc>,
	pub data_modificacao: DateTime<Utc>,
}

impl PartialEq for Procurador {
	fn eq(&self, other: &Self) -> bool {
		self.id_procurador == other.id_procurador
	}
}

#[derive(sqlx::Type, Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[sqlx(transparent)]
pub struct ReclamanteId(pub i32);

#[readonly::make]
#[derive(sqlx::FromRow, Debug)]
pub struct Reclamante {
	pub id_reclamante: ReclamanteId,
	pub tipo_pessoa: TipoPessoa,
	pub nome: String,
	pub cpf: Option<String>,
	pub cnpj: Option<String>,
	pub id_endereco: EnderecoId,
	pub data_criacao: DateTime<Utc>,
	pub data_modificacao: DateTime<Utc>,
}

impl PartialEq for Reclamante {
	fn eq(&self, other: &Self) -> bool {
		self.id_reclamante == other.id_reclamante
	}
}

#[derive(sqlx::Type, Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[sqlx(transparent)]
pub struct ReclamadoId(pub i32);

#[readonly::make]
#[derive(sqlx::FromRow, Debug)]
pub struct Reclamado {
	pub id_reclamado: ReclamadoId,
	pub tipo_pessoa: TipoPessoa,
	pub nome: Option<String>,
	pub razao_social: Option<String>,
	pub nome_fantasia: Option<String>,
	pub cpf: Option<String>,
	pub cnpj: Option<String>,
	pub email: Option<String>,
	pub num_telefone: Option<String>,
	pub id_endereco: EnderecoId,
	pub data_criacao: DateTime<Utc>,
	pub data_modificacao: DateTime<Utc>,
}

impl PartialEq for Reclamado {
	fn eq(&self, other: &Self) -> bool {
		self.id_reclamado == other.id_reclamado
	}
}

#[derive(sqlx::Type, Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[sqlx(transparent)]
pub struct AudienciaId(pub i32);

#[readonly::make]
#[derive(sqlx::FromRow, Debug)]
pub struct Audiencia {
	pub id_audiencia: AudienciaId,
	pub id_conciliador: FuncionarioId,
	pub data_hora: DateTime<Utc>,
	pub meio: MeioAudiencia,
	pub data_criacao: DateTime<Utc>,
	pub data_modificacao: DateTime<Utc>,
}

impl PartialEq for Audiencia {
	fn eq(&self, other: &Self) -> bool {
		self.id_audiencia == other.id_audiencia
	}
}

#[derive(sqlx::Type, Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[sqlx(transparent)]
pub struct ReclamacaoId(pub i32);

#[readonly::make]
#[derive(sqlx::FromRow, Debug)]
pub struct Reclamacao {
	pub id_reclamacao: ReclamacaoId,
	pub numero: i32,
	pub ano: i32,
	pub protocolo: String,
	pub id_reclamante: ReclamanteId,
	pub id_motivo: MotivoId,
	pub id_procurador: Option<ProcuradorId>,
	pub observacao: Option<String>,
	pub atendido: Option<bool>,
	pub id_criador: FuncionarioId,
	pub status: StatusReclamacao,
	pub id_diretorio: DiretorioId,
	pub data_criacao: DateTime<Utc>,
	pub data_modificacao: DateTime<Utc>,
}

impl PartialEq for Reclamacao {
	fn eq(&self, other: &Self) -> bool {
		self.id_reclamacao == other.id_reclamacao
	}
}

#[derive(sqlx::Type, Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[sqlx(transparent)]
pub struct HistoricoStatusReclamacaoId(pub i64);

#[readonly::make]
#[derive(sqlx::FromRow, Debug)]
pub struct HistoricoStatusReclamacao {
	pub id_historico: HistoricoStatusReclamacaoId,
	pub id_reclamacao: ReclamacaoId,
	pub status_anterior: StatusReclamacao,
	pub status_novo: StatusReclamacao,
	pub data_mudanca: DateTime<Utc>,
}

impl PartialEq for HistoricoStatusReclamacao {
	fn eq(&self, other: &Self) -> bool {
		self.id_historico == other.id_historico
	}
}

#[readonly::make]
#[derive(sqlx::FromRow, PartialEq, Debug)]
pub struct RelacaoReclamacaoReclamado {
	pub id_reclamacao: ReclamacaoId,
	pub id_reclamado: ReclamadoId,
}

#[readonly::make]
#[derive(sqlx::FromRow, PartialEq, Debug)]
pub struct RelacaoReclamacaoAudiencia {
	pub id_reclamacao: ReclamacaoId,
	pub id_audiencia: AudienciaId,
}

//
// Test
//

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_partial_eq() {
		let cargo1 = Cargo {
			id_cargo: CargoId(1),
			titulo: "Desenvolvedor".into(),
			data_criacao: Utc::now(),
		};
		let cargo2 = Cargo {
			id_cargo: CargoId(1),
			titulo: "Gerente".into(),
			data_criacao: Utc::now(),
		};
		let cargo3 = Cargo {
			id_cargo: CargoId(2),
			titulo: "Desenvolvedor".into(),
			data_criacao: Utc::now(),
		};

		assert_eq!(cargo1, cargo2);
		assert_ne!(cargo1, cargo3);
	}

	#[test]
	fn test_newtype() {
		let id_cargo = CargoId(1);
		assert_eq!(id_cargo.0, 1);

		let id_motivo = MotivoId(1);
		assert_eq!(id_motivo.0, 1);

		let id_diretorio = DiretorioId(2);
		assert_eq!(id_diretorio.0, 2);
	}
}

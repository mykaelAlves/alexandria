#![allow(dead_code)]

pub mod motivo {
	pub const COUNT_ALL: &str = "SELECT COUNT(*) FROM motivos";

	pub const GET_ALL: &str = "SELECT id_motivo, nome, artigo, \
	                           paragrafo_unico, inciso, data_criacao, \
	                           data_modificacao FROM motivos ORDER BY nome";

	pub const GET_N: &str = "SELECT id_motivo, nome, artigo, paragrafo_unico, \
	                         inciso, data_criacao, data_modificacao FROM \
	                         motivos ORDER BY nome LIMIT $1";

	pub const LIST: &str = "SELECT id_motivo, nome, artigo, paragrafo_unico, \
	                        inciso, data_criacao, data_modificacao FROM \
	                        motivos ORDER BY nome LIMIT $1 OFFSET $2";

	pub const GET_BY_NOME: &str =
		"SELECT id_motivo, nome, artigo, paragrafo_unico, inciso, \
		 data_criacao, data_modificacao FROM motivos WHERE nome = $1";

	pub const GET_BY_ID: &str =
		"SELECT id_motivo, nome, artigo, paragrafo_unico, inciso, \
		 data_criacao, data_modificacao FROM motivos WHERE id_motivo = $1";

	pub const INSERT: &str =
		"INSERT INTO motivos (nome, artigo, paragrafo_unico, inciso) VALUES \
		 ($1, $2, $3, $4) RETURNING id_motivo, nome, artigo, paragrafo_unico, \
		 inciso, data_criacao, data_modificacao";

	pub const DELETE_BY_ID: &str =
		"DELETE FROM motivos WHERE id_motivo = $1 RETURNING nome";

	pub const DELETE_BY_NOME: &str = "DELETE FROM motivos WHERE nome = $1";

	pub const UPDATE_BY_ID: &str = "UPDATE motivos SET nome = $1, artigo = \
	                                $2, paragrafo_unico = $3, inciso = $4 \
	                                WHERE id_motivo = $5";

	pub const UPDATE_BY_NOME: &str = "UPDATE motivos SET nome = $1, artigo = \
	                                  $2, paragrafo_unico = $3, inciso = $4 \
	                                  WHERE nome = $5";
}

pub mod cargo {
	pub const COUNT_ALL: &str = "SELECT COUNT(*) FROM cargos";

	pub const GET_ALL: &str =
		"SELECT id_cargo, titulo, data_criacao FROM cargos ORDER BY titulo";

	pub const GET_N: &str = "SELECT id_cargo, titulo, data_criacao FROM \
	                         cargos ORDER BY titulo LIMIT $1";

	pub const LIST: &str = "SELECT id_cargo, titulo, data_criacao FROM cargos \
	                        ORDER BY titulo LIMIT $1 OFFSET $2";

	pub const GET_BY_TITULO: &str =
		"SELECT id_cargo, titulo, data_criacao FROM cargos WHERE titulo = $1";

	pub const GET_BY_ID: &str =
		"SELECT id_cargo, titulo, data_criacao FROM cargos WHERE id_cargo = $1";

	pub const INSERT: &str = "INSERT INTO cargos (titulo) VALUES ($1) \
	                          RETURNING id_cargo, titulo, data_criacao";

	pub const DELETE_BY_ID: &str = "DELETE FROM cargos WHERE id_cargo = $1";
}

pub mod diretorio {
	pub const COUNT_ALL: &str = "SELECT COUNT(*) FROM diretorios";

	pub const GET_ALL: &str = "SELECT id_diretorio, caminho, modificavel, \
	                           data_criacao, data_modificacao FROM diretorios \
	                           ORDER BY caminho";

	pub const GET_N: &str = "SELECT id_diretorio, caminho, modificavel, \
	                         data_criacao, data_modificacao FROM diretorios \
	                         ORDER BY caminho LIMIT $1";

	pub const LIST: &str = "SELECT id_diretorio, caminho, modificavel, \
	                        data_criacao, data_modificacao FROM diretorios \
	                        ORDER BY caminho LIMIT $1 OFFSET $2";

	pub const GET_BY_CAMINHO: &str =
		"SELECT id_diretorio, caminho, modificavel, data_criacao, \
		 data_modificacao FROM diretorios WHERE caminho = $1";

	pub const GET_BY_ID: &str = "SELECT id_diretorio, caminho, modificavel, \
	                             data_criacao, data_modificacao FROM \
	                             diretorios WHERE id_diretorio = $1";

	pub const GET_BY_PROTOCOLO: &str =
		"SELECT d.id_diretorio, d.caminho, d.modificavel, d.data_criacao, \
		 d.data_modificacao FROM diretorios d JOIN reclamacoes r ON \
		 r.id_diretorio = d.id_diretorio WHERE r.protocolo = $1";

	pub const INSERT: &str = "INSERT INTO diretorios (caminho, modificavel) \
	                          VALUES ($1, $2) RETURNING id_diretorio, \
	                          caminho, modificavel, data_criacao, \
	                          data_modificacao";

	pub const DELETE_BY_ID: &str =
		"DELETE FROM diretorios WHERE id_diretorio = $1";

	pub const UPDATE_BY_ID: &str = "UPDATE diretorios SET caminho = $1, \
	                                modificavel = $2 WHERE id_diretorio = $3";

	pub const UPDATE_BY_CAMINHO: &str = "UPDATE diretorios SET caminho = $1, \
	                                     modificavel = $2 WHERE caminho = $3";
}

pub mod endereco {}

pub mod funcionario {}

pub mod procurador {}

pub mod reclamante {}

pub mod reclamado {}

pub mod audiencia {}

pub mod reclamacao {}

pub mod historico {}

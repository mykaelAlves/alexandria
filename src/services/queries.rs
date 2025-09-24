#![allow(dead_code)]

pub mod motivo {
	pub const COUNT_ALL: &str = "SELECT COUNT(*) FROM motivos";

	pub const GET_ALL: &str = "SELECT id_motivo, nome, artigo, \
	                           paragrafo_unico, inciso, data_criacao FROM \
	                           motivos ORDER BY nome";

	pub const GET_N: &str = "SELECT id_motivo, nome, artigo, paragrafo_unico, \
	                         inciso, data_criacao FROM motivos ORDER BY nome \
	                         LIMIT $1";

	pub const LIST: &str = "SELECT id_motivo, nome, artigo, paragrafo_unico, \
	                        inciso, data_criacao FROM motivos ORDER BY nome \
	                        LIMIT $1 OFFSET $2";

	pub const GET_BY_NOME: &str = "SELECT id_motivo, nome, artigo, \
	                               paragrafo_unico, inciso, data_criacao FROM \
	                               motivos WHERE nome = $1";

	pub const GET_BY_ID: &str = "SELECT id_motivo, nome, artigo, \
	                             paragrafo_unico, inciso, data_criacao FROM \
	                             motivos WHERE id_motivo = $1";

	pub const INSERT: &str = "INSERT INTO motivos (nome, artigo, \
	                          paragrafo_unico, inciso) VALUES ($1, $2, $3, \
	                          $4) RETURNING id_motivo, nome, artigo, \
	                          paragrafo_unico, inciso, data_criacao";

	pub const DELETE_BY_ID: &str = "DELETE FROM motivos WHERE id_motivo = $1";

	pub const DELETE_BY_NOME: &str = "DELETE FROM motivos WHERE nome = $1";

	pub const UPDATE_BY_ID: &str =
		"UPDATE motivos SET nome = $1, artigo = $2, paragrafo_unico = $3, \
		 inciso = $4 WHERE id_motivo = $5 RETURNING id_motivo, nome, artigo, \
		 paragrafo_unico, inciso, data_criacao";

	pub const UPDATE_BY_NOME: &str =
		"UPDATE motivos SET nome = $1, artigo = $2, paragrafo_unico = $3, \
		 inciso = $4 WHERE nome = $5 RETURNING id_motivo, nome, artigo, \
		 paragrafo_unico, inciso, data_criacao";
}

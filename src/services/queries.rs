//! Contains all queries used in this application

#![allow(dead_code)]

// -----------------------------------------------------------------------------
// --- CARGOS ---
// -----------------------------------------------------------------------------

// --------------------------
// CREATE
// --------------------------
pub const CREATE_CARGO: &str =
	"INSERT INTO cargos (titulo) VALUES ($1) RETURNING id_cargo;";

// --------------------------
// READ (Ativos)
// --------------------------
pub const GET_CARGO_BY_ID: &str = "SELECT id_cargo, titulo, created_at FROM \
                                   cargos WHERE id_cargo = $1 AND deleted_at \
                                   IS NULL;";

pub const GET_CARGO_BY_TITULO: &str = "SELECT id_cargo, titulo, created_at \
                                       FROM cargos WHERE titulo = $1 AND \
                                       deleted_at IS NULL;";

pub const COUNT_ALL_CARGOS: &str =
	"SELECT COUNT(*) FROM cargos WHERE deleted_at IS NULL;";

pub const LIST_CARGOS: &str = "SELECT id_cargo, titulo, created_at FROM \
                               cargos WHERE deleted_at IS NULL ORDER BY \
                               id_cargo LIMIT $1 OFFSET $2;";

// --------------------------
// UPDATE
// --------------------------

// --------------------------
// DELETE (Soft Delete)
// --------------------------
pub const SOFT_DELETE_CARGO_BY_ID: &str = "UPDATE cargos SET deleted_at = \
                                           NOW() WHERE id_cargo = $1 AND \
                                           deleted_at IS NULL;";

// --------------------------
// READ (ALL)
// --------------------------
pub const __UNDELETE_CARGO_BY_ID: &str = "UPDATE cargos SET deleted_at = NULL \
                                          WHERE id_cargo = $1 AND deleted_at \
                                          IS NOT NULL;";

pub const __GET_CARGO_BY_ID: &str = "SELECT id_cargo, titulo, created_at, \
                                     deleted_at FROM cargos WHERE id_cargo = \
                                     $1;";

pub const __GET_CARGO_BY_TITULO: &str = "SELECT id_cargo, titulo, created_at, \
                                         deleted_at FROM cargos WHERE titulo \
                                         = $1;";

pub const __COUNT_ALL_CARGOS: &str = "SELECT COUNT(*) FROM cargos;";

pub const __LIST_CARGOS: &str = "SELECT id_cargo, titulo, created_at, \
                                 deleted_at FROM cargos ORDER BY id_cargo \
                                 LIMIT $1 OFFSET $2;";

pub const __LIST_DELETED_CARGOS: &str =
	"SELECT id_cargo, titulo, created_at, deleted_at FROM cargos WHERE \
	 deleted_at IS NOT NULL ORDER BY deleted_at DESC LIMIT $1 OFFSET $2;";

// --------------------------
// CREATE
// --------------------------
pub const CREATE_MOTIVO: &str = "INSERT INTO motivos (nome, artigo, \
                                 paragrafo_unico, inciso) VALUES ($1, $2, $3, \
                                 $4) RETURNING id_motivo;";

// --------------------------
// READ (Ativos)
// --------------------------
pub const GET_MOTIVO_BY_ID: &str =
	"SELECT id_motivo, nome, artigo, paragrafo_unico, inciso, created_at, \
	 updated_at FROM motivos WHERE id_motivo = $1 AND deleted_at IS NULL;";

pub const GET_MOTIVO_BY_NOME: &str =
	"SELECT id_motivo, nome, artigo, paragrafo_unico, inciso, created_at, \
	 updated_at FROM motivos WHERE nome = $1 AND deleted_at IS NULL;";

pub const COUNT_ALL_MOTIVOS: &str =
	"SELECT COUNT(*) FROM motivos WHERE deleted_at IS NULL;";

pub const LIST_MOTIVOS: &str = "SELECT id_motivo, nome, artigo, \
                                paragrafo_unico, inciso, created_at, \
                                updated_at FROM motivos WHERE deleted_at IS \
                                NULL ORDER BY nome LIMIT $1 OFFSET $2;";

// --------------------------
// UPDATE (Ativos)
// --------------------------
pub const UPDATE_MOTIVO: &str = "UPDATE motivos SET nome = $1, artigo = $2, \
                                 paragrafo_unico = $3, inciso = $4 WHERE \
                                 id_motivo = $5 AND deleted_at IS NULL;";

// --------------------------
// DELETE (Soft Delete)
// --------------------------
pub const SOFT_DELETE_MOTIVO_BY_ID: &str = "UPDATE motivos SET deleted_at = \
                                            NOW() WHERE id_motivo = $1 AND \
                                            deleted_at IS NULL;";

// --------------------------
// READ (ALL)
// --------------------------
pub const __UNDELETE_MOTIVO_BY_ID: &str = "UPDATE motivos SET deleted_at = \
                                           NULL WHERE id_motivo = $1 AND \
                                           deleted_at IS NOT NULL;";

pub const __GET_MOTIVO_BY_ID: &str =
	"SELECT id_motivo, nome, artigo, paragrafo_unico, inciso, created_at, \
	 updated_at, deleted_at FROM motivos WHERE id_motivo = $1;";

pub const __GET_MOTIVO_BY_NOME: &str =
	"SELECT id_motivo, nome, artigo, paragrafo_unico, inciso, created_at, \
	 updated_at, deleted_at FROM motivos WHERE nome = $1;";

pub const __COUNT_ALL_MOTIVOS: &str = "SELECT COUNT(*) FROM motivos;";

pub const __LIST_MOTIVOS: &str =
	"SELECT id_motivo, nome, artigo, paragrafo_unico, inciso, created_at, \
	 updated_at, deleted_at FROM motivos ORDER BY nome LIMIT $1 OFFSET $2;";

pub const __LIST_DELETED_MOTIVOS: &str =
	"SELECT id_motivo, nome, artigo, paragrafo_unico, inciso, created_at, \
	 updated_at, deleted_at FROM motivos WHERE deleted_at IS NOT NULL ORDER \
	 BY deleted_at DESC LIMIT $1 OFFSET $2;";

// -----------------------------------------------------------------------------
// --- DIRETORIOS ---
// -----------------------------------------------------------------------------

// --------------------------
// CREATE
// --------------------------
pub const CREATE_DIRETORIO: &str = "INSERT INTO diretorios (caminho, \
                                    modificavel) VALUES ($1, $2) RETURNING \
                                    id_diretorio;";

// --------------------------
// READ (Ativos)
// --------------------------
pub const GET_DIRETORIO_BY_ID: &str =
	"SELECT id_diretorio, caminho, modificavel, created_at, updated_at FROM \
	 diretorios WHERE id_diretorio = $1 AND deleted_at IS NULL;";

pub const GET_DIRETORIO_BY_CAMINHO: &str =
	"SELECT id_diretorio, caminho, modificavel, created_at, updated_at FROM \
	 diretorios WHERE caminho = $1 AND deleted_at IS NULL;";

pub const COUNT_ALL_DIRETORIOS: &str =
	"SELECT COUNT(*) FROM diretorios WHERE deleted_at IS NULL;";

pub const LIST_DIRETORIOS: &str =
	"SELECT id_diretorio, caminho, modificavel, created_at, updated_at FROM \
	 diretorios WHERE deleted_at IS NULL ORDER BY caminho LIMIT $1 OFFSET $2;";

// --------------------------
// UPDATE (Ativos)
// --------------------------
pub const UPDATE_DIRETORIO: &str = "UPDATE diretorios SET caminho = $1, \
                                    modificavel = $2 WHERE id_diretorio = $3 \
                                    AND deleted_at IS NULL;";

// --------------------------
// DELETE (Soft Delete)
// --------------------------
pub const SOFT_DELETE_DIRETORIO_BY_ID: &str =
	"UPDATE diretorios SET deleted_at = NOW() WHERE id_diretorio = $1 AND \
	 deleted_at IS NULL;";

// --------------------------
// READ (ALL)
// --------------------------
pub const __UNDELETE_DIRETORIO_BY_ID: &str =
	"UPDATE diretorios SET deleted_at = NULL WHERE id_diretorio = $1 AND \
	 deleted_at IS NOT NULL;";

pub const __GET_DIRETORIO_BY_ID: &str =
	"SELECT id_diretorio, caminho, modificavel, created_at, updated_at, \
	 deleted_at FROM diretorios WHERE id_diretorio = $1;";

pub const __GET_DIRETORIO_BY_CAMINHO: &str =
	"SELECT id_diretorio, caminho, modificavel, created_at, updated_at, \
	 deleted_at FROM diretorios WHERE caminho = $1;";

pub const __COUNT_ALL_DIRETORIOS: &str = "SELECT COUNT(*) FROM diretorios;";

pub const __LIST_DIRETORIOS: &str =
	"SELECT id_diretorio, caminho, modificavel, created_at, updated_at, \
	 deleted_at FROM diretorios ORDER BY caminho LIMIT $1 OFFSET $2;";

pub const __LIST_DELETED_DIRETORIOS: &str =
	"SELECT id_diretorio, caminho, modificavel, created_at, updated_at, \
	 deleted_at FROM diretorios WHERE deleted_at IS NOT NULL ORDER BY \
	 deleted_at DESC LIMIT $1 OFFSET $2;";

// -----------------------------------------------------------------------------
// --- ENDERECOS ---
// (Mutável, com Soft Delete)
// -----------------------------------------------------------------------------

// --------------------------
// CREATE
// --------------------------
pub const CREATE_ENDERECO: &str = "INSERT INTO enderecos (cep, logradouro, \
                                   numero, complemento, bairro, cidade, \
                                   estado, pais) VALUES ($1, $2, $3, $4, $5, \
                                   $6, $7, $8) RETURNING id_endereco;";

// --------------------------
// READ (Ativos)
// --------------------------
pub const GET_ENDERECO_BY_ID: &str =
	"SELECT id_endereco, cep, logradouro, numero, complemento, bairro, \
	 cidade, estado, pais, created_at, updated_at FROM enderecos WHERE \
	 id_endereco = $1 AND deleted_at IS NULL;";

pub const COUNT_ALL_ENDERECOS: &str =
	"SELECT COUNT(*) FROM enderecos WHERE deleted_at IS NULL;";

pub const LIST_ENDERECOS: &str =
	"SELECT id_endereco, cep, logradouro, numero, complemento, bairro, \
	 cidade, estado, pais, created_at, updated_at FROM enderecos WHERE \
	 deleted_at IS NULL ORDER BY logradouro LIMIT $1 OFFSET $2;";

// --------------------------
// UPDATE (Ativos)
// --------------------------
pub const UPDATE_ENDERECO: &str =
	"UPDATE enderecos SET cep = $1, logradouro = $2, numero = $3, complemento \
	 = $4, bairro = $5, cidade = $6, estado = $7, pais = $8 WHERE id_endereco \
	 = $9 AND deleted_at IS NULL;";

// --------------------------
// DELETE (Soft Delete)
// --------------------------
pub const SOFT_DELETE_ENDERECO_BY_ID: &str = "UPDATE enderecos SET deleted_at \
                                              = NOW() WHERE id_endereco = $1 \
                                              AND deleted_at IS NULL;";

// --------------------------
// READ (ALL)
// --------------------------
pub const __UNDELETE_ENDERECO_BY_ID: &str = "UPDATE enderecos SET deleted_at \
                                             = NULL WHERE id_endereco = $1 \
                                             AND deleted_at IS NOT NULL;";

pub const __GET_ENDERECO_BY_ID: &str =
	"SELECT id_endereco, cep, logradouro, numero, complemento, bairro, \
	 cidade, estado, pais, created_at, updated_at, deleted_at FROM enderecos \
	 WHERE id_endereco = $1;";

pub const __COUNT_ALL_ENDERECOS: &str = "SELECT COUNT(*) FROM enderecos;";

pub const __LIST_ENDERECOS: &str =
	"SELECT id_endereco, cep, logradouro, numero, complemento, bairro, \
	 cidade, estado, pais, created_at, updated_at, deleted_at FROM enderecos \
	 ORDER BY logradouro LIMIT $1 OFFSET $2;";

pub const __LIST_DELETED_ENDERECOS: &str =
	"SELECT id_endereco, cep, logradouro, numero, complemento, bairro, \
	 cidade, estado, pais, created_at, updated_at, deleted_at FROM enderecos \
	 WHERE deleted_at IS NOT NULL ORDER BY deleted_at DESC LIMIT $1 OFFSET $2;";

// --------------------------
// CREATE
// --------------------------
pub const CREATE_FUNCIONARIO: &str =
	"INSERT INTO funcionarios (nome, id_cargo, email, num_telefone, username, \
	 pwd_hash) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id_funcionario;";

// --------------------------
// READ (Ativos)
// --------------------------
pub const GET_FUNCIONARIO_BY_ID: &str =
	"SELECT f.id_funcionario, f.nome, f.email, f.num_telefone, f.username, \
	 f.created_at, f.updated_at, c.id_cargo AS cargo_id, c.titulo AS \
	 cargo_titulo FROM funcionarios f LEFT JOIN cargos c ON f.id_cargo = \
	 c.id_cargo WHERE f.id_funcionario = $1 AND f.deleted_at IS NULL;";

pub const GET_FUNCIONARIO_BY_EMAIL: &str =
	"SELECT f.id_funcionario, f.nome, f.email, f.num_telefone, f.username, \
	 f.created_at, f.updated_at, c.id_cargo AS cargo_id, c.titulo AS \
	 cargo_titulo FROM funcionarios f LEFT JOIN cargos c ON f.id_cargo = \
	 c.id_cargo WHERE f.email = $1 AND f.deleted_at IS NULL;";

pub const GET_FUNCIONARIO_BY_USERNAME: &str =
	"SELECT f.id_funcionario, f.nome, f.email, f.num_telefone, f.username, \
	 f.created_at, f.updated_at, c.id_cargo AS cargo_id, c.titulo AS \
	 cargo_titulo FROM funcionarios f LEFT JOIN cargos c ON f.id_cargo = \
	 c.id_cargo WHERE f.username = $1 AND f.deleted_at IS NULL;";

// Query especial para autenticação (rápida, sem joins, com hash)
pub const GET_FUNCIONARIO_AUTH_BY_USERNAME: &str =
	"SELECT id_funcionario, username, pwd_hash, deleted_at FROM funcionarios \
	 WHERE username = $1;";

pub const COUNT_ALL_FUNCIONARIOS: &str =
	"SELECT COUNT(*) FROM funcionarios WHERE deleted_at IS NULL;";

pub const LIST_FUNCIONARIOS: &str =
	"SELECT f.id_funcionario, f.nome, f.email, f.num_telefone, f.username, \
	 f.created_at, f.updated_at, c.id_cargo AS cargo_id, c.titulo AS \
	 cargo_titulo FROM funcionarios f LEFT JOIN cargos c ON f.id_cargo = \
	 c.id_cargo WHERE f.deleted_at IS NULL ORDER BY f.nome LIMIT $1 OFFSET $2;";

// --------------------------
// UPDATE (Ativos)
// --------------------------
// Atualiza dados, mas não a senha
pub const UPDATE_FUNCIONARIO: &str =
	"UPDATE funcionarios SET nome = $1, id_cargo = $2, email = $3, \
	 num_telefone = $4, username = $5 WHERE id_funcionario = $6 AND \
	 deleted_at IS NULL;";

// Atualiza apenas a senha
pub const UPDATE_FUNCIONARIO_PASSWORD: &str =
	"UPDATE funcionarios SET pwd_hash = $1 WHERE id_funcionario = $2 AND \
	 deleted_at IS NULL;";

// --------------------------
// DELETE (Soft Delete)
// --------------------------
pub const SOFT_DELETE_FUNCIONARIO_BY_ID: &str =
	"UPDATE funcionarios SET deleted_at = NOW() WHERE id_funcionario = $1 AND \
	 deleted_at IS NULL;";

// --------------------------
// READ (ALL)
// --------------------------
pub const __UNDELETE_FUNCIONARIO_BY_ID: &str =
	"UPDATE funcionarios SET deleted_at = NULL WHERE id_funcionario = $1 AND \
	 deleted_at IS NOT NULL;";

// Admin GET (com joins, sem hash)
pub const __GET_FUNCIONARIO_BY_ID: &str =
	"SELECT f.id_funcionario, f.nome, f.email, f.num_telefone, f.username, \
	 f.created_at, f.updated_at, f.deleted_at, c.id_cargo AS cargo_id, \
	 c.titulo AS cargo_titulo FROM funcionarios f LEFT JOIN cargos c ON \
	 f.id_cargo = c.id_cargo WHERE f.id_funcionario = $1;";

pub const __COUNT_ALL_FUNCIONARIOS: &str = "SELECT COUNT(*) FROM funcionarios;";

pub const __LIST_FUNCIONARIOS: &str =
	"SELECT f.id_funcionario, f.nome, f.email, f.num_telefone, f.username, \
	 f.created_at, f.updated_at, f.deleted_at, c.id_cargo AS cargo_id, \
	 c.titulo AS cargo_titulo FROM funcionarios f LEFT JOIN cargos c ON \
	 f.id_cargo = c.id_cargo ORDER BY f.nome LIMIT $1 OFFSET $2;";

pub const __LIST_DELETED_FUNCIONARIOS: &str =
	"SELECT f.id_funcionario, f.nome, f.email, f.num_telefone, f.username, \
	 f.created_at, f.updated_at, f.deleted_at, c.id_cargo AS cargo_id, \
	 c.titulo AS cargo_titulo FROM funcionarios f LEFT JOIN cargos c ON \
	 f.id_cargo = c.id_cargo WHERE f.deleted_at IS NOT NULL ORDER BY \
	 f.deleted_at DESC LIMIT $1 OFFSET $2;";

// -----------------------------------------------------------------------------
// --- PROCURADORES ---
// (Mutável, com Soft Delete, com FK para enderecos)
// -----------------------------------------------------------------------------

// --------------------------
// CREATE
// --------------------------
pub const CREATE_PROCURADOR: &str =
	"INSERT INTO procuradores (nome, cpf, id_endereco, email, num_telefone) \
	 VALUES ($1, $2, $3, $4, $5) RETURNING id_procurador;";

// --------------------------
// READ (Ativos)
// --------------------------
pub const GET_PROCURADOR_BY_ID: &str =
	"SELECT p.id_procurador, p.nome, p.cpf, p.email, p.num_telefone, \
	 p.created_at, p.updated_at, e.id_endereco AS endereco_id, e.cep AS \
	 endereco_cep, e.logradouro AS endereco_logradouro, e.numero AS \
	 endereco_numero, e.complemento AS endereco_complemento, e.bairro AS \
	 endereco_bairro, e.cidade AS endereco_cidade, e.estado AS \
	 endereco_estado, e.pais AS endereco_pais FROM procuradores p LEFT JOIN \
	 enderecos e ON p.id_endereco = e.id_endereco WHERE p.id_procurador = $1 \
	 AND p.deleted_at IS NULL;";

pub const GET_PROCURADOR_BY_CPF: &str =
	"SELECT p.id_procurador, p.nome, p.cpf, p.email, p.num_telefone, \
	 p.created_at, p.updated_at, e.id_endereco AS endereco_id, e.cep AS \
	 endereco_cep, e.logradouro AS endereco_logradouro, e.numero AS \
	 endereco_numero, e.complemento AS endereco_complemento, e.bairro AS \
	 endereco_bairro, e.cidade AS endereco_cidade, e.estado AS \
	 endereco_estado, e.pais AS endereco_pais FROM procuradores p LEFT JOIN \
	 enderecos e ON p.id_endereco = e.id_endereco WHERE p.cpf = $1 AND \
	 p.deleted_at IS NULL;";

pub const GET_PROCURADOR_BY_EMAIL: &str =
	"SELECT p.id_procurador, p.nome, p.cpf, p.email, p.num_telefone, \
	 p.created_at, p.updated_at, e.id_endereco AS endereco_id, e.cep AS \
	 endereco_cep, e.logradouro AS endereco_logradouro, e.numero AS \
	 endereco_numero, e.complemento AS endereco_complemento, e.bairro AS \
	 endereco_bairro, e.cidade AS endereco_cidade, e.estado AS \
	 endereco_estado, e.pais AS endereco_pais FROM procuradores p LEFT JOIN \
	 enderecos e ON p.id_endereco = e.id_endereco WHERE p.email = $1 AND \
	 p.deleted_at IS NULL;";

pub const COUNT_ALL_PROCURADORES: &str =
	"SELECT COUNT(*) FROM procuradores WHERE deleted_at IS NULL;";

pub const LIST_PROCURADORES: &str =
	"SELECT p.id_procurador, p.nome, p.cpf, p.email, p.num_telefone, \
	 p.created_at, p.updated_at, e.id_endereco AS endereco_id, e.cep AS \
	 endereco_cep, e.logradouro AS endereco_logradouro, e.numero AS \
	 endereco_numero, e.complemento AS endereco_complemento, e.bairro AS \
	 endereco_bairro, e.cidade AS endereco_cidade, e.estado AS \
	 endereco_estado, e.pais AS endereco_pais FROM procuradores p LEFT JOIN \
	 enderecos e ON p.id_endereco = e.id_endereco WHERE p.deleted_at IS NULL \
	 ORDER BY p.nome LIMIT $1 OFFSET $2;";

// --------------------------
// UPDATE (Ativos)
// --------------------------
pub const UPDATE_PROCURADOR: &str =
	"UPDATE procuradores SET nome = $1, id_endereco = $2, email = $3, \
	 num_telefone = $4 WHERE id_procurador = $5 AND deleted_at IS NULL;";

// --------------------------
// DELETE (Soft Delete)
// --------------------------
pub const SOFT_DELETE_PROCURADOR_BY_ID: &str =
	"UPDATE procuradores SET deleted_at = NOW() WHERE id_procurador = $1 AND \
	 deleted_at IS NULL;";

// --------------------------
// READ (ALL)
// --------------------------
pub const __UNDELETE_PROCURADOR_BY_ID: &str =
	"UPDATE procuradores SET deleted_at = NULL WHERE id_procurador = $1 AND \
	 deleted_at IS NOT NULL;";

pub const __GET_PROCURADOR_BY_ID: &str =
	"SELECT p.id_procurador, p.nome, p.cpf, p.email, p.num_telefone, \
	 p.created_at, p.updated_at, p.deleted_at, e.id_endereco AS endereco_id, \
	 e.cep AS endereco_cep, e.logradouro AS endereco_logradouro, e.numero AS \
	 endereco_numero, e.complemento AS endereco_complemento, e.bairro AS \
	 endereco_bairro, e.cidade AS endereco_cidade, e.estado AS \
	 endereco_estado, e.pais AS endereco_pais FROM procuradores p LEFT JOIN \
	 enderecos e ON p.id_endereco = e.id_endereco WHERE p.id_procurador = $1;";

pub const __GET_PROCURADOR_BY_CPF: &str =
	"SELECT p.id_procurador, p.nome, p.cpf, p.email, p.num_telefone, \
	 p.created_at, p.updated_at, p.deleted_at, e.id_endereco AS endereco_id, \
	 e.cep AS endereco_cep, e.logradouro AS endereco_logradouro, e.numero AS \
	 endereco_numero, e.complemento AS endereco_complemento, e.bairro AS \
	 endereco_bairro, e.cidade AS endereco_cidade, e.estado AS \
	 endereco_estado, e.pais AS endereco_pais FROM procuradores p LEFT JOIN \
	 enderecos e ON p.id_endereco = e.id_endereco WHERE p.cpf = $1;";

pub const __COUNT_ALL_PROCURADORES: &str = "SELECT COUNT(*) FROM procuradores;";

pub const __LIST_PROCURADORES: &str =
	"SELECT p.id_procurador, p.nome, p.cpf, p.email, p.num_telefone, \
	 p.created_at, p.updated_at, p.deleted_at, e.id_endereco AS endereco_id, \
	 e.cep AS endereco_cep, e.logradouro AS endereco_logradouro, e.numero AS \
	 endereco_numero, e.complemento AS endereco_complemento, e.bairro AS \
	 endereco_bairro, e.cidade AS endereco_cidade, e.estado AS \
	 endereco_estado, e.pais AS endereco_pais FROM procuradores p LEFT JOIN \
	 enderecos e ON p.id_endereco = e.id_endereco ORDER BY p.nome LIMIT $1 \
	 OFFSET $2;";

pub const __LIST_DELETED_PROCURADORES: &str =
	"SELECT p.id_procurador, p.nome, p.cpf, p.email, p.num_telefone, \
	 p.created_at, p.updated_at, p.deleted_at, e.id_endereco AS endereco_id, \
	 e.cep AS endereco_cep, e.logradouro AS endereco_logradouro, e.numero AS \
	 endereco_numero, e.complemento AS endereco_complemento, e.bairro AS \
	 endereco_bairro, e.cidade AS endereco_cidade, e.estado AS \
	 endereco_estado, e.pais AS endereco_pais FROM procuradores p LEFT JOIN \
	 enderecos e ON p.id_endereco = e.id_endereco WHERE p.deleted_at IS NOT \
	 NULL ORDER BY p.deleted_at DESC LIMIT $1 OFFSET $2;";

// -----------------------------------------------------------------------------
// --- RECLAMANTES ---
// -----------------------------------------------------------------------------

// --------------------------
// CREATE
// --------------------------
pub const CREATE_RECLAMANTE_FISICA: &str =
	"INSERT INTO reclamantes (nome, tipo_pessoa, cpf, id_endereco) VALUES \
	 ($1, 'Fisica', $2, $3) RETURNING id_reclamante;";

pub const CREATE_RECLAMANTE_JURIDICA: &str =
	"INSERT INTO reclamantes (nome, tipo_pessoa, cnpj, id_endereco) VALUES \
	 ($1, 'Juridica', $2, $3) RETURNING id_reclamante;";

// --------------------------
// READ (Ativos)
// --------------------------
pub const GET_RECLAMANTE_BY_ID: &str =
	"SELECT r.id_reclamante, r.tipo_pessoa, r.nome, r.cpf, r.cnpj, \
	 r.created_at, r.updated_at, e.id_endereco AS endereco_id, e.cep AS \
	 endereco_cep, e.logradouro AS endereco_logradouro, e.numero AS \
	 endereco_numero, e.complemento AS endereco_complemento, e.bairro AS \
	 endereco_bairro, e.cidade AS endereco_cidade, e.estado AS \
	 endereco_estado, e.pais AS endereco_pais FROM reclamantes r LEFT JOIN \
	 enderecos e ON r.id_endereco = e.id_endereco WHERE r.id_reclamante = $1 \
	 AND r.deleted_at IS NULL;";

pub const GET_RECLAMANTE_BY_CPF: &str =
	"SELECT r.id_reclamante, r.tipo_pessoa, r.nome, r.cpf, r.cnpj, \
	 r.created_at, r.updated_at, e.id_endereco AS endereco_id, e.cep AS \
	 endereco_cep, e.logradouro AS endereco_logradouro, e.numero AS \
	 endereco_numero, e.complemento AS endereco_complemento, e.bairro AS \
	 endereco_bairro, e.cidade AS endereco_cidade, e.estado AS \
	 endereco_estado, e.pais AS endereco_pais FROM reclamantes r LEFT JOIN \
	 enderecos e ON r.id_endereco = e.id_endereco WHERE r.cpf = $1 AND \
	 r.deleted_at IS NULL;";

pub const GET_RECLAMANTE_BY_CNPJ: &str =
	"SELECT r.id_reclamante, r.tipo_pessoa, r.nome, r.cpf, r.cnpj, \
	 r.created_at, r.updated_at, e.id_endereco AS endereco_id, e.cep AS \
	 endereco_cep, e.logradouro AS endereco_logradouro, e.numero AS \
	 endereco_numero, e.complemento AS endereco_complemento, e.bairro AS \
	 endereco_bairro, e.cidade AS endereco_cidade, e.estado AS \
	 endereco_estado, e.pais AS endereco_pais FROM reclamantes r LEFT JOIN \
	 enderecos e ON r.id_endereco = e.id_endereco WHERE r.cnpj = $1 AND \
	 r.deleted_at IS NULL;";

pub const COUNT_ALL_RECLAMANTES: &str =
	"SELECT COUNT(*) FROM reclamantes WHERE deleted_at IS NULL;";

pub const LIST_RECLAMANTES: &str =
	"SELECT r.id_reclamante, r.tipo_pessoa, r.nome, r.cpf, r.cnpj, \
	 r.created_at, r.updated_at, e.id_endereco AS endereco_id, e.cep AS \
	 endereco_cep, e.logradouro AS endereco_logradouro, e.numero AS \
	 endereco_numero, e.complemento AS endereco_complemento, e.bairro AS \
	 endereco_bairro, e.cidade AS endereco_cidade, e.estado AS \
	 endereco_estado, e.pais AS endereco_pais FROM reclamantes r LEFT JOIN \
	 enderecos e ON r.id_endereco = e.id_endereco WHERE r.deleted_at IS NULL \
	 ORDER BY r.nome LIMIT $1 OFFSET $2;";

// --------------------------
// UPDATE (Ativos)
// --------------------------
// Atualiza dados, mas não o tipo/documento
pub const UPDATE_RECLAMANTE: &str = "UPDATE reclamantes SET nome = $1, \
                                     id_endereco = $2 WHERE id_reclamante = \
                                     $3 AND deleted_at IS NULL;";

// --------------------------
// DELETE (Soft Delete)
// --------------------------
pub const SOFT_DELETE_RECLAMANTE_BY_ID: &str =
	"UPDATE reclamantes SET deleted_at = NOW() WHERE id_reclamante = $1 AND \
	 deleted_at IS NULL;";

// --------------------------
// READ (ALL)
// --------------------------
pub const __UNDELETE_RECLAMANTE_BY_ID: &str =
	"UPDATE reclamantes SET deleted_at = NULL WHERE id_reclamante = $1 AND \
	 deleted_at IS NOT NULL;";

pub const __GET_RECLAMANTE_BY_ID: &str =
	"SELECT r.id_reclamante, r.tipo_pessoa, r.nome, r.cpf, r.cnpj, \
	 r.created_at, r.updated_at, r.deleted_at, e.id_endereco AS endereco_id, \
	 e.cep AS endereco_cep, e.logradouro AS endereco_logradouro, e.numero AS \
	 endereco_numero, e.complemento AS endereco_complemento, e.bairro AS \
	 endereco_bairro, e.cidade AS endereco_cidade, e.estado AS \
	 endereco_estado, e.pais AS endereco_pais FROM reclamantes r LEFT JOIN \
	 enderecos e ON r.id_endereco = e.id_endereco WHERE r.id_reclamante = $1;";

pub const __GET_RECLAMANTE_BY_CPF: &str =
	"SELECT r.id_reclamante, r.tipo_pessoa, r.nome, r.cpf, r.cnpj, \
	 r.created_at, r.updated_at, r.deleted_at, e.id_endereco AS endereco_id, \
	 e.cep AS endereco_cep, e.logradouro AS endereco_logradouro, e.numero AS \
	 endereco_numero, e.complemento AS endereco_complemento, e.bairro AS \
	 endereco_bairro, e.cidade AS endereco_cidade, e.estado AS \
	 endereco_estado, e.pais AS endereco_pais FROM reclamantes r LEFT JOIN \
	 enderecos e ON r.id_endereco = e.id_endereco WHERE r.cpf = $1;";

pub const __GET_RECLAMANTE_BY_CNPJ: &str =
	"SELECT r.id_reclamante, r.tipo_pessoa, r.nome, r.cpf, r.cnpj, \
	 r.created_at, r.updated_at, r.deleted_at, e.id_endereco AS endereco_id, \
	 e.cep AS endereco_cep, e.logradouro AS endereco_logradouro, e.numero AS \
	 endereco_numero, e.complemento AS endereco_complemento, e.bairro AS \
	 endereco_bairro, e.cidade AS endereco_cidade, e.estado AS \
	 endereco_estado, e.pais AS endereco_pais FROM reclamantes r LEFT JOIN \
	 enderecos e ON r.id_endereco = e.id_endereco WHERE r.cnpj = $1;";

pub const __COUNT_ALL_RECLAMANTES: &str = "SELECT COUNT(*) FROM reclamantes;";

pub const __LIST_RECLAMANTES: &str =
	"SELECT r.id_reclamante, r.tipo_pessoa, r.nome, r.cpf, r.cnpj, \
	 r.created_at, r.updated_at, r.deleted_at, e.id_endereco AS endereco_id, \
	 e.cep AS endereco_cep, e.logradouro AS endereco_logradouro, e.numero AS \
	 endereco_numero, e.complemento AS endereco_complemento, e.bairro AS \
	 endereco_bairro, e.cidade AS endereco_cidade, e.estado AS \
	 endereco_estado, e.pais AS endereco_pais FROM reclamantes r LEFT JOIN \
	 enderecos e ON r.id_endereco = e.id_endereco ORDER BY r.nome LIMIT $1 \
	 OFFSET $2;";

pub const __LIST_DELETED_RECLAMANTES: &str =
	"SELECT r.id_reclamante, r.tipo_pessoa, r.nome, r.cpf, r.cnpj, \
	 r.created_at, r.updated_at, r.deleted_at, e.id_endereco AS endereco_id, \
	 e.cep AS endereco_cep, e.logradouro AS endereco_logradouro, e.numero AS \
	 endereco_numero, e.complemento AS endereco_complemento, e.bairro AS \
	 endereco_bairro, e.cidade AS endereco_cidade, e.estado AS \
	 endereco_estado, e.pais AS endereco_pais FROM reclamantes r LEFT JOIN \
	 enderecos e ON r.id_endereco = e.id_endereco WHERE r.deleted_at IS NOT \
	 NULL ORDER BY r.deleted_at DESC LIMIT $1 OFFSET $2;";

// -----------------------------------------------------------------------------
// --- RECLAMADOS ---
// (Mutável, com Soft Delete, com FK para enderecos, Lógica de TipoPessoa)
// -----------------------------------------------------------------------------

// --------------------------
// CREATE
// --------------------------
pub const CREATE_RECLAMADO_FISICA: &str =
	"INSERT INTO reclamados (nome, cpf, email, num_telefone, id_endereco, \
	 tipo_pessoa) VALUES ($1, $2, $3, $4, $5, 'Fisica') RETURNING \
	 id_reclamado;";

pub const CREATE_RECLAMADO_JURIDICA: &str =
	"INSERT INTO reclamados (razao_social, nome_fantasia, cnpj, email, \
	 num_telefone, id_endereco, tipo_pessoa) VALUES ($1, $2, $3, $4, $5, $6, \
	 'Juridica') RETURNING id_reclamado;";

// --------------------------
// READ (Ativos)
// --------------------------
pub const GET_RECLAMADO_BY_ID: &str =
	"SELECT r.id_reclamado, r.tipo_pessoa, r.nome, r.razao_social, \
	 r.nome_fantasia, r.cpf, r.cnpj, r.email, r.num_telefone, r.created_at, \
	 r.updated_at, e.id_endereco AS endereco_id, e.cep AS endereco_cep, \
	 e.logradouro AS endereco_logradouro, e.numero AS endereco_numero, \
	 e.complemento AS endereco_complemento, e.bairro AS endereco_bairro, \
	 e.cidade AS endereco_cidade, e.estado AS endereco_estado, e.pais AS \
	 endereco_pais FROM reclamados r LEFT JOIN enderecos e ON r.id_endereco = \
	 e.id_endereco WHERE r.id_reclamado = $1 AND r.deleted_at IS NULL;";

pub const GET_RECLAMADO_BY_CPF: &str =
	"SELECT r.id_reclamado, r.tipo_pessoa, r.nome, r.razao_social, \
	 r.nome_fantasia, r.cpf, r.cnpj, r.email, r.num_telefone, r.created_at, \
	 r.updated_at, e.id_endereco AS endereco_id, e.cep AS endereco_cep, \
	 e.logradouro AS endereco_logradouro, e.numero AS endereco_numero, \
	 e.complemento AS endereco_complemento, e.bairro AS endereco_bairro, \
	 e.cidade AS endereco_cidade, e.estado AS endereco_estado, e.pais AS \
	 endereco_pais FROM reclamados r LEFT JOIN enderecos e ON r.id_endereco = \
	 e.id_endereco WHERE r.cpf = $1 AND r.deleted_at IS NULL;";

pub const GET_RECLAMADO_BY_CNPJ: &str =
	"SELECT r.id_reclamado, r.tipo_pessoa, r.nome, r.razao_social, \
	 r.nome_fantasia, r.cpf, r.cnpj, r.email, r.num_telefone, r.created_at, \
	 r.updated_at, e.id_endereco AS endereco_id, e.cep AS endereco_cep, \
	 e.logradouro AS endereco_logradouro, e.numero AS endereco_numero, \
	 e.complemento AS endereco_complemento, e.bairro AS endereco_bairro, \
	 e.cidade AS endereco_cidade, e.estado AS endereco_estado, e.pais AS \
	 endereco_pais FROM reclamados r LEFT JOIN enderecos e ON r.id_endereco = \
	 e.id_endereco WHERE r.cnpj = $1 AND r.deleted_at IS NULL;";

pub const COUNT_ALL_RECLAMADOS: &str =
	"SELECT COUNT(*) FROM reclamados WHERE deleted_at IS NULL;";

pub const LIST_RECLAMADOS: &str =
	"SELECT r.id_reclamado, r.tipo_pessoa, r.nome, r.razao_social, \
	 r.nome_fantasia, r.cpf, r.cnpj, r.email, r.num_telefone, r.created_at, \
	 r.updated_at, e.id_endereco AS endereco_id, e.cep AS endereco_cep, \
	 e.logradouro AS endereco_logradouro, e.numero AS endereco_numero, \
	 e.complemento AS endereco_complemento, e.bairro AS endereco_bairro, \
	 e.cidade AS endereco_cidade, e.estado AS endereco_estado, e.pais AS \
	 endereco_pais FROM reclamados r LEFT JOIN enderecos e ON r.id_endereco = \
	 e.id_endereco WHERE r.deleted_at IS NULL ORDER BY r.nome, r.razao_social \
	 LIMIT $1 OFFSET $2;";

// --------------------------
// UPDATE (Ativos)
// --------------------------
pub const UPDATE_RECLAMADO_FISICA: &str =
	"UPDATE reclamados SET nome = $1, email = $2, num_telefone = $3, \
	 id_endereco = $4 WHERE id_reclamado = $5 AND tipo_pessoa = 'Fisica' AND \
	 deleted_at IS NULL;";

pub const UPDATE_RECLAMADO_JURIDICA: &str =
	"UPDATE reclamados SET razao_social = $1, nome_fantasia = $2, email = $3, \
	 num_telefone = $4, id_endereco = $5 WHERE id_reclamado = $6 AND \
	 tipo_pessoa = 'Juridica' AND deleted_at IS NULL;";

// --------------------------
// DELETE (Soft Delete)
// --------------------------
pub const SOFT_DELETE_RECLAMADO_BY_ID: &str =
	"UPDATE reclamados SET deleted_at = NOW() WHERE id_reclamado = $1 AND \
	 deleted_at IS NULL;";

// --------------------------
// READ (ALL)
// --------------------------
pub const __UNDELETE_RECLAMADO_BY_ID: &str =
	"UPDATE reclamados SET deleted_at = NULL WHERE id_reclamado = $1 AND \
	 deleted_at IS NOT NULL;";

pub const __GET_RECLAMADO_BY_ID: &str =
	"SELECT r.id_reclamado, r.tipo_pessoa, r.nome, r.razao_social, \
	 r.nome_fantasia, r.cpf, r.cnpj, r.email, r.num_telefone, r.created_at, \
	 r.updated_at, r.deleted_at, e.id_endereco AS endereco_id, e.cep AS \
	 endereco_cep, e.logradouro AS endereco_logradouro, e.numero AS \
	 endereco_numero, e.complemento AS endereco_complemento, e.bairro AS \
	 endereco_bairro, e.cidade AS endereco_cidade, e.estado AS \
	 endereco_estado, e.pais AS endereco_pais FROM reclamados r LEFT JOIN \
	 enderecos e ON r.id_endereco = e.id_endereco WHERE r.id_reclamado = $1;";

pub const __GET_RECLAMADO_BY_CPF: &str =
	"SELECT r.id_reclamado, r.tipo_pessoa, r.nome, r.razao_social, \
	 r.nome_fantasia, r.cpf, r.cnpj, r.email, r.num_telefone, r.created_at, \
	 r.updated_at, r.deleted_at, e.id_endereco AS endereco_id, e.cep AS \
	 endereco_cep, e.logradouro AS endereco_logradouro, e.numero AS \
	 endereco_numero, e.complemento AS endereco_complemento, e.bairro AS \
	 endereco_bairro, e.cidade AS endereco_cidade, e.estado AS \
	 endereco_estado, e.pais AS endereco_pais FROM reclamados r LEFT JOIN \
	 enderecos e ON r.id_endereco = e.id_endereco WHERE r.cpf = $1;";

pub const __GET_RECLAMADO_BY_CNPJ: &str =
	"SELECT r.id_reclamado, r.tipo_pessoa, r.nome, r.razao_social, \
	 r.nome_fantasia, r.cpf, r.cnpj, r.email, r.num_telefone, r.created_at, \
	 r.updated_at, r.deleted_at, e.id_endereco AS endereco_id, e.cep AS \
	 endereco_cep, e.logradouro AS endereco_logradouro, e.numero AS \
	 endereco_numero, e.complemento AS endereco_complemento, e.bairro AS \
	 endereco_bairro, e.cidade AS endereco_cidade, e.estado AS \
	 endereco_estado, e.pais AS endereco_pais FROM reclamados r LEFT JOIN \
	 enderecos e ON r.id_endereco = e.id_endereco WHERE r.cnpj = $1;";

pub const __COUNT_ALL_RECLAMADOS: &str = "SELECT COUNT(*) FROM reclamados;";

pub const __LIST_RECLAMADOS: &str =
	"SELECT r.id_reclamado, r.tipo_pessoa, r.nome, r.razao_social, \
	 r.nome_fantasia, r.cpf, r.cnpj, r.email, r.num_telefone, r.created_at, \
	 r.updated_at, r.deleted_at, e.id_endereco AS endereco_id, e.cep AS \
	 endereco_cep, e.logradouro AS endereco_logradouro, e.numero AS \
	 endereco_numero, e.complemento AS endereco_complemento, e.bairro AS \
	 endereco_bairro, e.cidade AS endereco_cidade, e.estado AS \
	 endereco_estado, e.pais AS endereco_pais FROM reclamados r LEFT JOIN \
	 enderecos e ON r.id_endereco = e.id_endereco ORDER BY r.nome, \
	 r.razao_social LIMIT $1 OFFSET $2;";

pub const __LIST_DELETED_RECLAMADOS: &str =
	"SELECT r.id_reclamado, r.tipo_pessoa, r.nome, r.razao_social, \
	 r.nome_fantasia, r.cpf, r.cnpj, r.email, r.num_telefone, r.created_at, \
	 r.updated_at, r.deleted_at, e.id_endereco AS endereco_id, e.cep AS \
	 endereco_cep, e.logradouro AS endereco_logradouro, e.numero AS \
	 endereco_numero, e.complemento AS endereco_complemento, e.bairro AS \
	 endereco_bairro, e.cidade AS endereco_cidade, e.estado AS \
	 endereco_estado, e.pais AS endereco_pais FROM reclamados r LEFT JOIN \
	 enderecos e ON r.id_endereco = e.id_endereco WHERE r.deleted_at IS NOT \
	 NULL ORDER BY r.deleted_at DESC LIMIT $1 OFFSET $2;";

// -----------------------------------------------------------------------------
// --- AUDIENCIAS ---
// -----------------------------------------------------------------------------

// --------------------------
// CREATE
// --------------------------
pub const CREATE_AUDIENCIA: &str = "INSERT INTO audiencias (id_conciliador, \
                                    data_hora, meio) VALUES ($1, $2, $3) \
                                    RETURNING id_audiencia;";

// --------------------------
// READ (Ativos)
// --------------------------
pub const GET_AUDIENCIA_BY_ID: &str =
	"SELECT a.id_audiencia, a.data_hora, a.meio, a.created_at, a.updated_at, \
	 f.id_funcionario AS funcionario_id, f.nome AS funcionario_nome, f.email \
	 AS funcionario_email, f.username AS funcionario_username, f.num_telefone \
	 AS funcionario_num_telefone FROM audiencias a LEFT JOIN funcionarios f \
	 ON a.id_conciliador = f.id_funcionario WHERE a.id_audiencia = $1 AND \
	 a.deleted_at IS NULL;";

pub const COUNT_ALL_AUDIENCIAS: &str =
	"SELECT COUNT(*) FROM audiencias WHERE deleted_at IS NULL;";

pub const LIST_AUDIENCIAS: &str =
	"SELECT a.id_audiencia, a.data_hora, a.meio, a.created_at, a.updated_at, \
	 f.id_funcionario AS funcionario_id, f.nome AS funcionario_nome, f.email \
	 AS funcionario_email, f.username AS funcionario_username, f.num_telefone \
	 AS funcionario_num_telefone FROM audiencias a LEFT JOIN funcionarios f \
	 ON a.id_conciliador = f.id_funcionario WHERE a.deleted_at IS NULL ORDER \
	 BY a.data_hora DESC LIMIT $1 OFFSET $2;";

// --------------------------
// UPDATE (Ativos)
// --------------------------
pub const UPDATE_AUDIENCIA: &str = "UPDATE audiencias SET id_conciliador = \
                                    $1, data_hora = $2, meio = $3 WHERE \
                                    id_audiencia = $4 AND deleted_at IS NULL;";

// --------------------------
// DELETE (Soft Delete)
// --------------------------
pub const SOFT_DELETE_AUDIENCIA_BY_ID: &str =
	"UPDATE audiencias SET deleted_at = NOW() WHERE id_audiencia = $1 AND \
	 deleted_at IS NULL;";

// --------------------------
// READ (ALL)
// --------------------------
pub const __UNDELETE_AUDIENCIA_BY_ID: &str =
	"UPDATE audiencias SET deleted_at = NULL WHERE id_audiencia = $1 AND \
	 deleted_at IS NOT NULL;";

pub const __GET_AUDIENCIA_BY_ID: &str =
	"SELECT a.id_audiencia, a.data_hora, a.meio, a.created_at, a.updated_at, \
	 a.deleted_at, f.id_funcionario AS funcionario_id, f.nome AS \
	 funcionario_nome, f.email AS funcionario_email, f.username AS \
	 funcionario_username, f.num_telefone AS funcionario_num_telefone FROM \
	 audiencias a LEFT JOIN funcionarios f ON a.id_conciliador = \
	 f.id_funcionario WHERE a.id_audiencia = $1;";

pub const __COUNT_ALL_AUDIENCIAS: &str = "SELECT COUNT(*) FROM audiencias;";

pub const __LIST_AUDIENCIAS: &str =
	"SELECT a.id_audiencia, a.data_hora, a.meio, a.created_at, a.updated_at, \
	 a.deleted_at, f.id_funcionario AS funcionario_id, f.nome AS \
	 funcionario_nome, f.email AS funcionario_email, f.username AS \
	 funcionario_username, f.num_telefone AS funcionario_num_telefone FROM \
	 audiencias a LEFT JOIN funcionarios f ON a.id_conciliador = \
	 f.id_funcionario ORDER BY a.data_hora DESC LIMIT $1 OFFSET $2;";

pub const __LIST_DELETED_AUDIENCIAS: &str =
	"SELECT a.id_audiencia, a.data_hora, a.meio, a.created_at, a.updated_at, \
	 a.deleted_at, f.id_funcionario AS funcionario_id, f.nome AS \
	 funcionario_nome, f.email AS funcionario_email, f.username AS \
	 funcionario_username, f.num_telefone AS funcionario_num_telefone FROM \
	 audiencias a LEFT JOIN funcionarios f ON a.id_conciliador = \
	 f.id_funcionario WHERE a.deleted_at IS NOT NULL ORDER BY a.deleted_at \
	 DESC LIMIT $1 OFFSET $2;";

// -----------------------------------------------------------------------------
// --- RECLAMACOES ---
// (Mutável, com Soft Delete, com 5 FKs)
// -----------------------------------------------------------------------------

// --------------------------
// CREATE
// --------------------------
pub const CREATE_RECLAMACAO: &str =
	"INSERT INTO reclamacoes (numero, ano, id_reclamante, id_motivo, \
	 id_procurador, observacao, atendido, id_criador, status, id_diretorio) \
	 VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING \
	 id_reclamacao, protocolo;";

// --------------------------
// READ (Ativos)
// --------------------------
// Nota: Esta é uma query muito grande.
pub const GET_RECLAMACAO_BY_ID: &str =
	"SELECT r.id_reclamacao, r.numero, r.ano, r.protocolo, r.observacao, \
	 r.atendido, r.status, r.created_at, r.updated_at, rnt.id_reclamante AS \
	 reclamante_id, rnt.nome AS reclamante_nome, rnt.cpf AS reclamante_cpf, \
	 rnt.cnpj AS reclamante_cnpj, m.id_motivo AS motivo_id, m.nome AS \
	 motivo_nome, p.id_procurador AS procurador_id, p.nome AS \
	 procurador_nome, p.cpf AS procurador_cpf, f.id_funcionario AS \
	 funcionario_id_criador, f.nome AS funcionario_nome_criador, \
	 d.id_diretorio AS diretorio_id, d.caminho AS diretorio_caminho FROM \
	 reclamacoes r LEFT JOIN reclamantes rnt ON r.id_reclamante = \
	 rnt.id_reclamante LEFT JOIN motivos m ON r.id_motivo = m.id_motivo LEFT \
	 JOIN procuradores p ON r.id_procurador = p.id_procurador LEFT JOIN \
	 funcionarios f ON r.id_criador = f.id_funcionario LEFT JOIN diretorios d \
	 ON r.id_diretorio = d.id_diretorio WHERE r.id_reclamacao = $1 AND \
	 r.deleted_at IS NULL;";

pub const GET_RECLAMACAO_BY_PROTOCOLO: &str =
	"SELECT r.id_reclamacao, r.numero, r.ano, r.protocolo, r.observacao, \
	 r.atendido, r.status, r.created_at, r.updated_at, rnt.id_reclamante AS \
	 reclamante_id, rnt.nome AS reclamante_nome, rnt.cpf AS reclamante_cpf, \
	 rnt.cnpj AS reclamante_cnpj, m.id_motivo AS motivo_id, m.nome AS \
	 motivo_nome, p.id_procurador AS procurador_id, p.nome AS \
	 procurador_nome, p.cpf AS procurador_cpf, f.id_funcionario AS \
	 funcionario_id_criador, f.nome AS funcionario_nome_criador, \
	 d.id_diretorio AS diretorio_id, d.caminho AS diretorio_caminho FROM \
	 reclamacoes r LEFT JOIN reclamantes rnt ON r.id_reclamante = \
	 rnt.id_reclamante LEFT JOIN motivos m ON r.id_motivo = m.id_motivo LEFT \
	 JOIN procuradores p ON r.id_procurador = p.id_procurador LEFT JOIN \
	 funcionarios f ON r.id_criador = f.id_funcionario LEFT JOIN diretorios d \
	 ON r.id_diretorio = d.id_diretorio WHERE r.numero = $1 AND r.ano = $2 \
	 AND r.deleted_at IS NULL;";

pub const COUNT_ALL_RECLAMACOES: &str =
	"SELECT COUNT(*) FROM reclamacoes WHERE deleted_at IS NULL;";

pub const LIST_RECLAMACOES: &str =
	"SELECT r.id_reclamacao, r.numero, r.ano, r.protocolo, r.observacao, \
	 r.atendido, r.status, r.created_at, r.updated_at, rnt.id_reclamante AS \
	 reclamante_id, rnt.nome AS reclamante_nome, m.id_motivo AS motivo_id, \
	 m.nome AS motivo_nome, p.id_procurador AS procurador_id, p.nome AS \
	 procurador_nome, f.id_funcionario AS funcionario_id_criador, f.nome AS \
	 funcionario_nome_criador FROM reclamacoes r LEFT JOIN reclamantes rnt ON \
	 r.id_reclamante = rnt.id_reclamante LEFT JOIN motivos m ON r.id_motivo = \
	 m.id_motivo LEFT JOIN procuradores p ON r.id_procurador = \
	 p.id_procurador LEFT JOIN funcionarios f ON r.id_criador = \
	 f.id_funcionario WHERE r.deleted_at IS NULL ORDER BY r.ano DESC, \
	 r.numero DESC LIMIT $1 OFFSET $2;";

// --------------------------
// UPDATE (Ativos)
// --------------------------
pub const UPDATE_RECLAMACAO: &str =
	"UPDATE reclamacoes SET id_reclamante = $1, id_motivo = $2, id_procurador \
	 = $3, observacao = $4, atendido = $5, status = $6, id_diretorio = $7 \
	 WHERE id_reclamacao = $8 AND deleted_at IS NULL;";
// Query específica para mudar SÓ o status (dispara a trigger)
pub const UPDATE_RECLAMACAO_STATUS: &str = "UPDATE reclamacoes SET status = \
                                            $1 WHERE id_reclamacao = $2 AND \
                                            deleted_at IS NULL;";

// --------------------------
// DELETE (Soft Delete)
// --------------------------
pub const SOFT_DELETE_RECLAMACAO_BY_ID: &str =
	"UPDATE reclamacoes SET deleted_at = NOW() WHERE id_reclamacao = $1 AND \
	 deleted_at IS NULL;";

// --------------------------
// READ (Admin - Inclui deletados)
// --------------------------
pub const __UNDELETE_RECLAMACAO_BY_ID: &str =
	"UPDATE reclamacoes SET deleted_at = NULL WHERE id_reclamacao = $1 AND \
	 deleted_at IS NOT NULL;";

pub const __GET_RECLAMACAO_BY_ID: &str =
	"SELECT r.id_reclamacao, r.numero, r.ano, r.protocolo, r.observacao, \
	 r.atendido, r.status, r.created_at, r.updated_at, r.deleted_at, \
	 rnt.id_reclamante AS reclamante_id, rnt.nome AS reclamante_nome, rnt.cpf \
	 AS reclamante_cpf, rnt.cnpj AS reclamante_cnpj, m.id_motivo AS \
	 motivo_id, m.nome AS motivo_nome, p.id_procurador AS procurador_id, \
	 p.nome AS procurador_nome, p.cpf AS procurador_cpf, f.id_funcionario AS \
	 funcionario_id_criador, f.nome AS funcionario_nome_criador, \
	 d.id_diretorio AS diretorio_id, d.caminho AS diretorio_caminho FROM \
	 reclamacoes r LEFT JOIN reclamantes rnt ON r.id_reclamante = \
	 rnt.id_reclamante LEFT JOIN motivos m ON r.id_motivo = m.id_motivo LEFT \
	 JOIN procuradores p ON r.id_procurador = p.id_procurador LEFT JOIN \
	 funcionarios f ON r.id_criador = f.id_funcionario LEFT JOIN diretorios d \
	 ON r.id_diretorio = d.id_diretorio WHERE r.id_reclamacao = $1;";

pub const __COUNT_ALL_RECLAMACOES: &str = "SELECT COUNT(*) FROM reclamacoes;";

pub const __LIST_RECLAMACOES: &str =
	"SELECT r.id_reclamacao, r.numero, r.ano, r.protocolo, r.observacao, \
	 r.atendido, r.status, r.created_at, r.updated_at, r.deleted_at, \
	 rnt.id_reclamante AS reclamante_id, rnt.nome AS reclamante_nome, \
	 m.id_motivo AS motivo_id, m.nome AS motivo_nome, p.id_procurador AS \
	 procurador_id, p.nome AS procurador_nome, f.id_funcionario AS \
	 funcionario_id_criador, f.nome AS funcionario_nome_criador FROM \
	 reclamacoes r LEFT JOIN reclamantes rnt ON r.id_reclamante = \
	 rnt.id_reclamante LEFT JOIN motivos m ON r.id_motivo = m.id_motivo LEFT \
	 JOIN procuradores p ON r.id_procurador = p.id_procurador LEFT JOIN \
	 funcionarios f ON r.id_criador = f.id_funcionario ORDER BY r.ano DESC, \
	 r.numero DESC LIMIT $1 OFFSET $2;";

pub const __LIST_DELETED_RECLAMACOES: &str =
	"SELECT r.id_reclamacao, r.numero, r.ano, r.protocolo, r.observacao, \
	 r.atendido, r.status, r.created_at, r.updated_at, r.deleted_at, \
	 rnt.id_reclamante AS reclamante_id, rnt.nome AS reclamante_nome, \
	 m.id_motivo AS motivo_id, m.nome AS motivo_nome FROM reclamacoes r LEFT \
	 JOIN reclamantes rnt ON r.id_reclamante = rnt.id_reclamante LEFT JOIN \
	 motivos m ON r.id_motivo = m.id_motivo WHERE r.deleted_at IS NOT NULL \
	 ORDER BY r.deleted_at DESC LIMIT $1 OFFSET $2;";

// -----------------------------------------------------------------------------
// --- HISTORICO_STATUS_RECLAMACOES ---
// (Tabela de Log/Auditoria - Apenas Leitura)
// (Sem Create/Update/Delete, Sem Soft Delete, com FK para reclamacoes)
// -----------------------------------------------------------------------------

// --------------------------
// READ
// --------------------------
pub const LIST_HISTORICO_BY_RECLAMACAO_ID: &str =
	"SELECT h.id_historico, h.status_anterior, h.status_novo, h.data_mudanca, \
	 r.id_reclamacao AS reclamacao_id, r.protocolo AS reclamacao_protocolo \
	 FROM historico_status_reclamacoes h LEFT JOIN reclamacoes r ON \
	 h.id_reclamacao = r.id_reclamacao WHERE h.id_reclamacao = $1 ORDER BY \
	 h.data_mudanca DESC;";

// --------------------------
// READ (Admin)
// --------------------------
pub const __COUNT_ALL_HISTORICO: &str =
	"SELECT COUNT(*) FROM historico_status_reclamacoes;";

pub const __LIST_ALL_HISTORICO: &str =
	"SELECT h.id_historico, h.status_anterior, h.status_novo, h.data_mudanca, \
	 r.id_reclamacao AS reclamacao_id, r.protocolo AS reclamacao_protocolo \
	 FROM historico_status_reclamacoes h LEFT JOIN reclamacoes r ON \
	 h.id_reclamacao = r.id_reclamacao ORDER BY h.data_mudanca DESC LIMIT $1 \
	 OFFSET $2;";

     // -----------------------------------------------------------------------------
// --- RELACAO_RECLAMACAO_RECLAMADO ---
// (Tabela de Ligação N-N, com Soft Delete)
// -----------------------------------------------------------------------------

// --------------------------
// CREATE (LINK)
// --------------------------
// Cria a ligação entre uma reclamação e um reclamado.
// Se a ligação já existiu e foi "soft-deleted", esta query a restaura.
pub const LINK_RECLAMACAO_RECLAMADO: &str = "INSERT INTO \
                                              relacao_reclamacao_reclamado \
                                              (id_reclamacao, \
                                              id_reclamado) VALUES ($1, $2) \
                                              ON CONFLICT (id_reclamacao, \
                                              id_reclamado) DO UPDATE SET \
                                              deleted_at = NULL;";

// --------------------------
// READ (Listar Relações Ativas)
// --------------------------
// Lista todos os RECLAMADOS associados a uma RECLAMAÇÃO
pub const LIST_RECLAMADOS_BY_RECLAMACAO_ID: &str = "SELECT r.id_reclamado, \
                                                    r.tipo_pessoa, r.nome \
                                                    AS reclamado_nome, \
                                                    r.razao_social AS \
                                                    reclamado_razao_social, \
                                                    r.nome_fantasia AS \
                                                    reclamado_nome_fantasia, \
                                                    r.cpf AS \
                                                    reclamado_cpf, \
                                                    r.cnpj AS \
                                                    reclamado_cnpj, \
                                                    r.email AS \
                                                    reclamado_email, \
                                                    r.num_telefone AS \
                                                    reclamado_num_telefone \
                                                    FROM \
                                                    relacao_reclamacao_reclamado \
                                                    rel JOIN reclamados r \
                                                    ON rel.id_reclamado = \
                                                    r.id_reclamado WHERE \
                                                    rel.id_reclamacao = $1 \
                                                    AND rel.deleted_at IS \
                                                    NULL AND \
                                                    r.deleted_at IS \
                                                    NULL;";

// Lista todas as RECLAMAÇÕES associadas a um RECLAMADO
pub const LIST_RECLAMACOES_BY_RECLAMADO_ID: &str = "SELECT r.id_reclamacao, \
                                                    r.protocolo AS \
                                                    reclamacao_protocolo, \
                                                    r.status AS \
                                                    reclamacao_status, \
                                                    r.created_at AS \
                                                    reclamacao_created_at, \
                                                    rnt.nome AS \
                                                    reclamante_nome FROM \
                                                    relacao_reclamacao_reclamado \
                                                    rel JOIN reclamacoes r \
                                                    ON rel.id_reclamacao = \
                                                    r.id_reclamacao LEFT \
                                                    JOIN reclamantes rnt \
                                                    ON r.id_reclamante = \
                                                    rnt.id_reclamante \
                                                    WHERE \
                                                    rel.id_reclamado = $1 \
                                                    AND rel.deleted_at IS \
                                                    NULL AND \
                                                    r.deleted_at IS \
                                                    NULL;";

// --------------------------
// DELETE (UNLINK / Soft Delete)
// --------------------------
pub const UNLINK_RECLAMACAO_RECLAMADO: &str = "UPDATE \
                                               relacao_reclamacao_reclamado \
                                               SET deleted_at = NOW() \
                                               WHERE id_reclamacao = $1 \
                                               AND id_reclamado = $2 AND \
                                               deleted_at IS NULL;";

// --------------------------
// READ (Admin - Inclui deletados)
// --------------------------
// Restaura um link (admin)
pub const __UNDELETE_LINK_RECLAMACAO_RECLAMADO: &str = "UPDATE \
                                                        relacao_reclamacao_reclamado \
                                                        SET deleted_at = \
                                                        NULL WHERE \
                                                        id_reclamacao = \
                                                        $1 AND \
                                                        id_reclamado = \
                                                        $2 AND \
                                                        deleted_at IS NOT \
                                                        NULL;";

// Lista TODOS os RECLAMADOS associados a uma RECLAMAÇÃO (incluindo links
// deletados)
pub const __LIST_RECLAMADOS_BY_RECLAMACAO_ID: &str = "SELECT \
                                                      r.id_reclamado, \
                                                      r.tipo_pessoa, \
                                                      r.nome AS \
                                                      reclamado_nome, \
                                                      r.razao_social AS \
                                                      reclamado_razao_social, \
                                                      r.cpf AS \
                                                      reclamado_cpf, \
                                                      r.cnpj AS \
                                                      reclamado_cnpj, \
                                                      rel.deleted_at AS \
                                                      link_deleted_at FROM \
                                                      relacao_reclamacao_reclamado \
                                                      rel JOIN reclamados \
                                                      r ON \
                                                      rel.id_reclamado = \
                                                      r.id_reclamado \
                                                      WHERE \
                                                      rel.id_reclamacao = \
                                                      $1;";

// -----------------------------------------------------------------------------
// --- RELACAO_RECLAMACAO_AUDIENCIA ---
// (Tabela de Ligação N-N, com Soft Delete)
// -----------------------------------------------------------------------------

// --------------------------
// CREATE (LINK)
// --------------------------
// Cria a ligação entre uma reclamação e uma audiência.
// Se a ligação já existiu e foi "soft-deleted", esta query a restaura.
pub const LINK_RECLAMACAO_AUDIENCIA: &str = "INSERT INTO \
                                              relacao_reclamacao_audiencia \
                                              (id_reclamacao, \
                                              id_audiencia) VALUES ($1, $2) \
                                              ON CONFLICT (id_reclamacao, \
                                              id_audiencia) DO UPDATE SET \
                                              deleted_at = NULL;";

// --------------------------
// READ (Listar Relações Ativas)
// --------------------------
// Lista todas as AUDIÊNCIAS associadas a uma RECLAMAÇÃO
pub const LIST_AUDIENCIAS_BY_RECLAMACAO_ID: &str = "SELECT a.id_audiencia, \
                                                    a.data_hora AS \
                                                    audiencia_data_hora, \
                                                    a.meio AS \
                                                    audiencia_meio, \
                                                    a.created_at AS \
                                                    audiencia_created_at, \
                                                    f.id_funcionario AS \
                                                    funcionario_id, \
                                                    f.nome AS \
                                                    funcionario_nome FROM \
                                                    relacao_reclamacao_audiencia \
                                                    rel JOIN audiencias a \
                                                    ON rel.id_audiencia = \
                                                    a.id_audiencia LEFT \
                                                    JOIN funcionarios f ON \
                                                    a.id_conciliador = \
                                                    f.id_funcionario \
                                                    WHERE \
                                                    rel.id_reclamacao = $1 \
                                                    AND rel.deleted_at IS \
                                                    NULL AND \
                                                    a.deleted_at IS \
                                                    NULL;";

// Lista todas as RECLAMAÇÕES associadas a uma AUDIÊNCIA
pub const LIST_RECLAMACOES_BY_AUDIENCIA_ID: &str = "SELECT r.id_reclamacao, \
                                                    r.protocolo AS \
                                                    reclamacao_protocolo, \
                                                    r.status AS \
                                                    reclamacao_status, \
                                                    r.created_at AS \
                                                    reclamacao_created_at, \
                                                    rnt.nome AS \
                                                    reclamante_nome FROM \
                                                    relacao_reclamacao_audiencia \
                                                    rel JOIN reclamacoes r \
                                                    ON rel.id_reclamacao = \
                                                    r.id_reclamacao LEFT \
                                                    JOIN reclamantes rnt \
                                                    ON r.id_reclamante = \
                                                    rnt.id_reclamante \
                                                    WHERE \
                                                    rel.id_audiencia = $1 \
                                                    AND rel.deleted_at IS \
                                                    NULL AND \
                                                    r.deleted_at IS \
                                                    NULL;";

// --------------------------
// DELETE (UNLINK / Soft Delete)
// --------------------------
pub const UNLINK_RECLAMACAO_AUDIENCIA: &str = "UPDATE \
                                               relacao_reclamacao_audiencia \
                                               SET deleted_at = NOW() \
                                               WHERE id_reclamacao = $1 \
                                               AND id_audiencia = $2 AND \
                                               deleted_at IS NULL;";

// --------------------------
// READ (Admin - Inclui deletados)
// --------------------------
// Restaura um link (admin)
pub const __UNDELETE_LINK_RECLAMACAO_AUDIENCIA: &str = "UPDATE \
                                                        relacao_reclamacao_audiencia \
                                                        SET deleted_at = \
                                                        NULL WHERE \
                                                        id_reclamacao = \
                                                        $1 AND \
                                                        id_audiencia = $2 \
                                                        AND deleted_at IS \
                                                        NOT NULL;";

// Lista TODAS as AUDIÊNCIAS associadas a uma RECLAMAÇÃO (incluindo links
// deletados)
pub const __LIST_AUDIENCIAS_BY_RECLAMACAO_ID: &str = "SELECT a.id_audiencia, \
                                                      a.data_hora AS \
                                                      audiencia_data_hora, \
                                                      a.meio AS \
                                                      audiencia_meio, \
                                                      f.nome AS \
                                                      funcionario_nome, \
                                                      rel.deleted_at AS \
                                                      link_deleted_at \
                                                      FROM \
                                                      relacao_reclamacao_audiencia \
                                                      rel JOIN audiencias \
                                                      a ON \
                                                      rel.id_audiencia = \
                                                      a.id_audiencia \
                                                      LEFT JOIN \
                                                      funcionarios f ON \
                                                      a.id_conciliador = \
                                                      f.id_funcionario \
                                                      WHERE \
                                                      rel.id_reclamacao = \
                                                      $1;";
-- ====================================================================
-- SEED SCRIPT FOR POPULATING THE DATABASE
-- ====================================================================
-- This script is idempotent and can be run multiple times.
-- It first deletes existing data in the correct order to avoid FK constraints,
-- then inserts a new set of consistent test data.
-- ====================================================================

BEGIN;

-- --------------------------------------------------------------------
-- DELETION PHASE - Clean up tables in reverse order of dependencies
-- --------------------------------------------------------------------
-- Disabling triggers to avoid issues during mass delete/insert
SET session_replication_role = 'replica';

DELETE FROM relacao_reclamacao_audiencia;
DELETE FROM relacao_reclamacao_reclamado;
DELETE FROM historico_status_reclamacoes;
DELETE FROM reclamacoes;
DELETE FROM audiencias;
DELETE FROM reclamados;
DELETE FROM reclamantes;
DELETE FROM procuradores;
DELETE FROM funcionarios;
DELETE FROM enderecos;
DELETE FROM diretorios;
DELETE FROM motivos;
DELETE FROM cargos;

-- Re-enabling triggers
SET session_replication_role = 'origin';

-- Resetting sequences for primary keys to start from 1 again
ALTER SEQUENCE cargos_id_cargo_seq RESTART WITH 1;
ALTER SEQUENCE motivos_id_motivo_seq RESTART WITH 1;
ALTER SEQUENCE diretorios_id_diretorio_seq RESTART WITH 1;
ALTER SEQUENCE enderecos_id_endereco_seq RESTART WITH 1;
ALTER SEQUENCE funcionarios_id_funcionario_seq RESTART WITH 1;
ALTER SEQUENCE procuradores_id_procurador_seq RESTART WITH 1;
ALTER SEQUENCE reclamantes_id_reclamante_seq RESTART WITH 1;
ALTER SEQUENCE reclamados_id_reclamado_seq RESTART WITH 1;
ALTER SEQUENCE audiencias_id_audiencia_seq RESTART WITH 1;
ALTER SEQUENCE reclamacoes_id_reclamacao_seq RESTART WITH 1;
ALTER SEQUENCE historico_status_reclamacoes_id_historico_seq RESTART WITH 1;


-- --------------------------------------------------------------------
-- INSERTION PHASE - Populate tables with test data
-- --------------------------------------------------------------------

-- Tabela: cargos
INSERT INTO cargos (titulo) VALUES
('Conciliador'),
('Atendente'),
('Escrivã'),
('Administrador do Sistema');

-- Tabela: motivos
INSERT INTO motivos (nome, artigo, paragrafo_unico, inciso) VALUES
('Produto com defeito de fabricação', 18, false, 1),
('Serviço não fornecido conforme o contrato', 20, true, NULL),
('Publicidade enganosa ou abusiva', 37, false, 1),
('Cobrança indevida de valores', 42, true, NULL);

-- Tabela: diretorios
INSERT INTO diretorios (caminho, modificavel) VALUES
('/srv/reclamacoes/docs/2025/', TRUE),
('/srv/reclamacoes/docs/2024/', FALSE),
('/srv/reclamacoes/docs/arquivados/', TRUE);

-- Tabela: enderecos
INSERT INTO enderecos (cep, logradouro, numero, complemento, bairro, cidade, estado) VALUES
('63900001', 'Rua da Matriz', '123', 'Apto 101', 'Centro', 'Quixadá', 'CE'),
('63908410', 'Avenida Plácido Castelo', '456', NULL, 'Planalto Universitário', 'Quixadá', 'CE'),
('60150160', 'Avenida Santos Dumont', '789', 'Sala 502', 'Aldeota', 'Fortaleza', 'CE'),
('01310100', 'Avenida Paulista', '1000', 'Andar 8', 'Bela Vista', 'São Paulo', 'SP');

-- Tabela: funcionarios
-- Senhas: 'senha123', 'senha456', 'senha789'
INSERT INTO funcionarios (nome, id_cargo, email, num_telefone, username, pwd_hash) VALUES
('Ana Costa', 1, 'ana.conciliadora@email.com', '88999991111', 'ana.costa', crypt('senha123', gen_salt('bf'))),
('Bruno Lima', 2, 'bruno.atendente@email.com', '88999992222', 'bruno.lima', crypt('senha456', gen_salt('bf'))),
('Carla Dias', 3, 'carla.gerente@email.com', '85988883333', 'carla.dias', crypt('senha789', gen_salt('bf')));

-- Tabela: procuradores
INSERT INTO procuradores (nome, cpf, id_endereco, email, num_telefone) VALUES
('Dr. Carlos Mendes', '45870558328', 3, 'carlos.mendes@adv.com', '85987654321');

-- Tabela: reclamantes
INSERT INTO reclamantes (tipo_pessoa, nome, cpf, cnpj, id_endereco) VALUES
('Fisica', 'João da Silva', '12936179386', NULL, 1),
('Fisica', 'Maria Oliveira', '17178766336', NULL, 2),
('Juridica', 'Tecnologia Inovadora Ltda', NULL, '55666604000163', 4);

-- Tabela: reclamados
INSERT INTO reclamados (tipo_pessoa, nome, razao_social, nome_fantasia, cpf, cnpj, email, num_telefone, id_endereco) VALUES
('Fisica', 'Pedro Souza', NULL, NULL, '78901838362', NULL, 'pedro.souza@email.com', '11988776655', 4),
('Juridica', NULL, 'Comércio Varejista S.A.', 'Loja de Tudo', NULL, '12469231000128', 'contato@lojadetudo.com', '1140028922', 4),
('Juridica', NULL, 'Serviços Online ME', 'Web Rápido', NULL, '13121403000130', 'suporte@webrapido.com', '21977665544', 3);

-- Tabela: audiencias
INSERT INTO audiencias (id_conciliador, data_hora, meio) VALUES
(1, '2025-11-20 10:00:00-03', 'Remoto'),
(1, '2025-09-15 14:30:00-03', 'Presencial');

-- Tabela: reclamacoes
INSERT INTO reclamacoes (numero, ano, id_reclamante, id_motivo, id_procurador, observacao, atendido, id_criador, status, id_diretorio) VALUES
(101, 2025, 1, 1, 1, 'O smartphone comprado parou de funcionar após uma semana de uso.', NULL, 2, 'EmTramitacao', 1),
(102, 2025, 2, 4, NULL, 'Recebi uma fatura de cartão de crédito com cobrança de anuidade, serviço que foi prometido como isento.', TRUE, 2, 'Arquivado', 3),
(103, 2025, 3, 2, 1, 'Contratamos um serviço de desenvolvimento de software que não foi entregue no prazo e está incompleto.', FALSE, 2, 'EmTramitacao', 1);

-- Tabela: relacao_reclamacao_reclamado (ligação N-N)
INSERT INTO relacao_reclamacao_reclamado (id_reclamacao, id_reclamado) VALUES
(1, 2), -- João da Silva vs Loja de Tudo
(2, 3), -- Maria Oliveira vs Web Rápido
(3, 3); -- Tecnologia Inovadora vs Web Rápido

-- Tabela: relacao_reclamacao_audiencia (ligação N-N)
INSERT INTO relacao_reclamacao_audiencia (id_reclamacao, id_audiencia) VALUES
(1, 1); -- Reclamação 101/2025 tem uma audiência agendada

-- --------------------------------------------------------------------
-- UPDATE PHASE - Testar triggers
-- --------------------------------------------------------------------
-- Esta atualização irá acionar o trigger 'trigger_registrar_mudanca_status'
-- e inserir um registro na tabela 'historico_status_reclamacoes'.
-- A reclamação 3 passará de 'EmTramitacao' para 'Arquivado'.
UPDATE reclamacoes
SET status = 'Arquivado', atendido = TRUE, data_modificacao = NOW()
WHERE id_reclamacao = 3;


COMMIT;

-- ====================================================================
-- FIM DO SCRIPT
-- ====================================================================

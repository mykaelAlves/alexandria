-- =============================================================================
-- SEÇÃO 1: EXCLUSÃO DAS TABELAS
-- =============================================================================
-- As tabelas são removidas na ordem inversa da sua criação para maior segurança.

DROP TABLE IF EXISTS relacao_reclamacao_audiencia CASCADE;
DROP TABLE IF EXISTS relacao_reclamacao_reclamado CASCADE;
DROP TABLE IF EXISTS historico_status_reclamacoes CASCADE;
DROP TABLE IF EXISTS reclamacoes CASCADE;
DROP TABLE IF EXISTS audiencias CASCADE;
DROP TABLE IF EXISTS reclamados CASCADE;
DROP TABLE IF EXISTS reclamantes CASCADE;
DROP TABLE IF EXISTS procuradores CASCADE;
DROP TABLE IF EXISTS funcionarios CASCADE;
DROP TABLE IF EXISTS enderecos CASCADE;
DROP TABLE IF EXISTS diretorios CASCADE;
DROP TABLE IF EXISTS motivos CASCADE;
DROP TABLE IF EXISTS cargos CASCADE;


-- =============================================================================
-- SEÇÃO 2: EXCLUSÃO DAS FUNÇÕES
-- =============================================================================

DROP FUNCTION IF EXISTS trigger_set_timestamp();
DROP FUNCTION IF EXISTS trigger_registrar_mudanca_status();


-- =============================================================================
-- SEÇÃO 3: EXCLUSÃO DOS TIPOS E DOMÍNIOS
-- =============================================================================
-- Tipos e domínios são removidos por último, após todos os objetos que os utilizam.

DROP TYPE IF EXISTS meio_audiencia_enum;
DROP TYPE IF EXISTS status_reclamacao_enum;
DROP TYPE IF EXISTS tipo_pessoa_enum;
DROP TYPE IF EXISTS uf_enum;

DROP DOMAIN IF EXISTS d_cpf;
DROP DOMAIN IF EXISTS d_cnpj;
DROP DOMAIN IF EXISTS d_email;
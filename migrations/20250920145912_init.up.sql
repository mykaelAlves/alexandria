CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE OR REPLACE FUNCTION trigger_set_null_fk_procurador()
RETURNS TRIGGER AS $$
BEGIN
  UPDATE FROM reclamacoes SET id_procurador = NULL WHERE id_procurador = OLD.id_procurador;
  RETURN OLD;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION trigger_delete_relacao_reclamacao_reclamado()
RETURNS TRIGGER AS $$
BEGIN
  UPDATE FROM relacao_reclamacao_reclamado SET deleted_at = NOW() WHERE id_reclamacao = OLD.id_reclamacao;
  RETURN OLD;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION trigger_delete_relacao_reclamacao_audiencia()
RETURNS TRIGGER AS $$
BEGIN
  UPDATE FROM relacao_reclamacao_audiencia SET deleted_at = NOW() WHERE id_reclamacao = OLD.id_reclamacao;
  RETURN OLD;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION validar_cpf(cpf TEXT)
RETURNS BOOLEAN AS $$
DECLARE
    cpf_limpo TEXT;
    soma INT;
    resto INT;
    digito1 INT;
    digito2 INT;
BEGIN
    -- Remove caracteres não numéricos
    cpf_limpo := REGEXP_REPLACE(cpf, '[^0-9]', '', 'g');

    -- Verifica se possui 11 dígitos e se não são todos iguais
    IF LENGTH(cpf_limpo) != 11 OR cpf_limpo ~ '^(.)\1+$' THEN
        RETURN FALSE;
    END IF;

    -- Cálculo do primeiro dígito verificador
    soma := 0;
    FOR i IN 1..9 LOOP
        soma := soma + CAST(SUBSTRING(cpf_limpo FROM i FOR 1) AS INT) * (11 - i);
    END LOOP;
    resto := soma % 11;
    digito1 := CASE WHEN resto < 2 THEN 0 ELSE 11 - resto END;

    -- Verifica o primeiro dígito
    IF digito1 != CAST(SUBSTRING(cpf_limpo FROM 10 FOR 1) AS INT) THEN
        RETURN FALSE;
    END IF;

    -- Cálculo do segundo dígito verificador
    soma := 0;
    FOR i IN 1..10 LOOP
        soma := soma + CAST(SUBSTRING(cpf_limpo FROM i FOR 1) AS INT) * (12 - i);
    END LOOP;
    resto := soma % 11;
    digito2 := CASE WHEN resto < 2 THEN 0 ELSE 11 - resto END;

    -- Verifica o segundo dígito
    IF digito2 != CAST(SUBSTRING(cpf_limpo FROM 11 FOR 1) AS INT) THEN
        RETURN FALSE;
    END IF;

    RETURN TRUE;
END;
$$ LANGUAGE plpgsql IMMUTABLE;

CREATE OR REPLACE FUNCTION validar_cnpj(cnpj TEXT)
RETURNS BOOLEAN AS $$
DECLARE
    cnpj_limpo TEXT;
    soma INT;
    resto INT;
    digito1 INT;
    digito2 INT;
    pesos1 INT[] := ARRAY[5,4,3,2,9,8,7,6,5,4,3,2];
    pesos2 INT[] := ARRAY[6,5,4,3,2,9,8,7,6,5,4,3,2];
BEGIN
    -- Remove caracteres não numéricos
    cnpj_limpo := REGEXP_REPLACE(cnpj, '[^0-9]', '', 'g');

    -- Verifica se possui 14 dígitos e se não são todos iguais
    IF LENGTH(cnpj_limpo) != 14 OR cnpj_limpo ~ '^(.)\1+$' THEN
        RETURN FALSE;
    END IF;

    -- Cálculo do primeiro dígito verificador
    soma := 0;
    FOR i IN 1..12 LOOP
        soma := soma + CAST(SUBSTRING(cnpj_limpo FROM i FOR 1) AS INT) * pesos1[i];
    END LOOP;
    resto := soma % 11;
    digito1 := CASE WHEN resto < 2 THEN 0 ELSE 11 - resto END;

    -- Verifica o primeiro dígito
    IF digito1 != CAST(SUBSTRING(cnpj_limpo FROM 13 FOR 1) AS INT) THEN
        RETURN FALSE;
    END IF;

    -- Cálculo do segundo dígito verificador
    soma := 0;
    FOR i IN 1..13 LOOP
        soma := soma + CAST(SUBSTRING(cnpj_limpo FROM i FOR 1) AS INT) * pesos2[i];
    END LOOP;
    resto := soma % 11;
    digito2 := CASE WHEN resto < 2 THEN 0 ELSE 11 - resto END;

    -- Verifica o segundo dígito
    IF digito2 != CAST(SUBSTRING(cnpj_limpo FROM 14 FOR 1) AS INT) THEN
        RETURN FALSE;
    END IF;

    RETURN TRUE;
END;
$$ LANGUAGE plpgsql IMMUTABLE;


CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION trigger_registrar_mudanca_status()
RETURNS TRIGGER AS $$
BEGIN
  INSERT INTO historico_status_reclamacoes (id_reclamacao, status_anterior, status_novo)
  VALUES (OLD.id_reclamacao, OLD.status, NEW.status);
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;


CREATE TYPE meio_audiencia_enum AS ENUM ('Remoto', 'Hibrido', 'Presencial');
CREATE TYPE status_reclamacao_enum AS ENUM ('EmTramitacao', 'Arquivado', 'Desarquivado');
CREATE TYPE tipo_pessoa_enum AS ENUM ('Fisica', 'Juridica');
CREATE TYPE uf_enum AS ENUM (
  'AC', 'AL', 'AP', 'AM', 'BA', 'CE', 'DF', 'ES', 'GO', 'MA', 'MT', 'MS',
  'MG', 'PA', 'PB', 'PR', 'PE', 'PI', 'RJ', 'RN', 'RS', 'RO', 'RR', 'SC',
  'SP', 'SE', 'TO'
);


CREATE DOMAIN d_cpf AS VARCHAR(11)
  CHECK (VALUE ~ '^[0-9]{11}$')
  CHECK (validar_cpf(VALUE));

CREATE DOMAIN d_cnpj AS VARCHAR(14)
  CHECK (VALUE ~ '^[0-9]{14}$')
  CHECK (validar_cnpj(VALUE));

CREATE DOMAIN d_email AS VARCHAR(255)
  CHECK (VALUE ~ '^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$');


CREATE TABLE cargos (
  id_cargo INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  titulo VARCHAR(100) NOT NULL UNIQUE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ NULL
);

CREATE TABLE motivos (
  id_motivo INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  nome VARCHAR(255) NOT NULL UNIQUE,
  artigo SMALLINT NOT NULL,
  paragrafo_unico BOOLEAN NOT NULL,
  inciso SMALLINT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ NULL
);

CREATE TABLE diretorios (
  id_diretorio INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  caminho VARCHAR(255) NOT NULL UNIQUE,
  modificavel BOOLEAN NOT NULL DEFAULT TRUE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ NULL
);

CREATE TABLE enderecos (
  id_endereco INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  cep VARCHAR(8) NOT NULL CHECK (cep ~ '^[0-9]{8}$'),
  logradouro VARCHAR(255) NOT NULL,
  numero VARCHAR(20) NOT NULL,
  complemento VARCHAR(100) NULL,
  bairro VARCHAR(100) NOT NULL,
  cidade VARCHAR(100) NOT NULL,
  estado uf_enum NOT NULL,
  pais VARCHAR(50) NOT NULL DEFAULT 'Brasil',
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ NULL
);

CREATE TABLE funcionarios (
  id_funcionario INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  nome VARCHAR(255) NOT NULL,
  id_cargo INT NOT NULL REFERENCES cargos(id_cargo) ON DELETE RESTRICT ON UPDATE CASCADE,
  email d_email NULL UNIQUE,
  num_telefone VARCHAR(20) NULL,
  username VARCHAR(100) NOT NULL UNIQUE,
  pwd_hash VARCHAR(255) NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ NULL
);

CREATE TABLE procuradores (
  id_procurador INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  nome VARCHAR(255) NOT NULL,
  cpf d_cpf NOT NULL UNIQUE,
  id_endereco INT NOT NULL REFERENCES enderecos(id_endereco) ON DELETE RESTRICT ON UPDATE CASCADE,
  email d_email NULL,
  num_telefone VARCHAR(20) NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ NULL
);

CREATE TABLE reclamantes (
  id_reclamante INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  tipo_pessoa tipo_pessoa_enum NOT NULL,
  nome VARCHAR(255) NOT NULL,
  cpf d_cpf NULL UNIQUE,
  cnpj d_cnpj NULL UNIQUE,
  id_endereco INT NOT NULL REFERENCES enderecos(id_endereco) ON DELETE RESTRICT ON UPDATE CASCADE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ NULL,
  CONSTRAINT chk_reclamante_tipo_pessoa
    CHECK ((tipo_pessoa = 'Fisica' AND cpf IS NOT NULL AND cnpj IS NULL) OR
           (tipo_pessoa = 'Juridica' AND cnpj IS NOT NULL AND cpf IS NULL))
);

CREATE TABLE reclamados (
  id_reclamado INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  tipo_pessoa tipo_pessoa_enum NOT NULL,
  nome VARCHAR(255) NULL,
  razao_social VARCHAR(255) NULL,
  nome_fantasia VARCHAR(255) NULL,
  cpf d_cpf NULL UNIQUE,
  cnpj d_cnpj NULL UNIQUE,
  email d_email NULL,
  num_telefone VARCHAR(20) NULL,
  id_endereco INT NOT NULL REFERENCES enderecos(id_endereco) ON DELETE RESTRICT ON UPDATE CASCADE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ NULL,
  CONSTRAINT chk_reclamado_tipo_pessoa
    CHECK ((tipo_pessoa = 'Fisica' AND cpf IS NOT NULL AND nome IS NOT NULL AND cnpj IS NULL AND razao_social IS NULL) OR
           (tipo_pessoa = 'Juridica' AND cnpj IS NOT NULL AND razao_social IS NOT NULL AND cpf IS NULL AND nome IS NULL))
);

CREATE TABLE audiencias (
  id_audiencia INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  id_conciliador INT NOT NULL REFERENCES funcionarios(id_funcionario) ON DELETE RESTRICT ON UPDATE CASCADE,
  data_hora TIMESTAMPTZ NOT NULL,
  meio meio_audiencia_enum NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ NULL
);

CREATE TABLE reclamacoes (
  id_reclamacao INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  numero INT NOT NULL CHECK (numero > 0),
  ano INT NOT NULL CHECK (ano >= 1900),
  protocolo VARCHAR(20) GENERATED ALWAYS AS (numero || '-' || ano) STORED,
  id_reclamante INT NOT NULL REFERENCES reclamantes(id_reclamante) ON DELETE RESTRICT ON UPDATE CASCADE,
  id_motivo INT NOT NULL REFERENCES motivos(id_motivo) ON DELETE RESTRICT ON UPDATE CASCADE,
  id_procurador INT NULL REFERENCES procuradores(id_procurador) ON DELETE SET NULL ON UPDATE CASCADE,
  observacao TEXT NULL,
  atendido BOOLEAN NULL,
  id_criador INT NOT NULL REFERENCES funcionarios(id_funcionario) ON DELETE RESTRICT ON UPDATE CASCADE,
  status status_reclamacao_enum NOT NULL,
  id_diretorio INT NOT NULL REFERENCES diretorios(id_diretorio) ON DELETE RESTRICT ON UPDATE CASCADE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ NULL,
  UNIQUE (numero, ano)
);

CREATE TABLE historico_status_reclamacoes (
  id_historico BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  id_reclamacao INT NOT NULL REFERENCES reclamacoes(id_reclamacao) ON DELETE CASCADE ON UPDATE CASCADE,
  status_anterior status_reclamacao_enum NOT NULL,
  status_novo status_reclamacao_enum NOT NULL,
  data_mudanca TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE relacao_reclamacao_reclamado (
  id_reclamacao INT NOT NULL REFERENCES reclamacoes(id_reclamacao) ON DELETE CASCADE ON UPDATE CASCADE,
  id_reclamado INT NOT NULL REFERENCES reclamados(id_reclamado) ON DELETE CASCADE ON UPDATE CASCADE,
  deleted_at TIMESTAMPTZ NULL,
  PRIMARY KEY (id_reclamacao, id_reclamado)
);

CREATE TABLE relacao_reclamacao_audiencia (
  id_reclamacao INT NOT NULL REFERENCES reclamacoes(id_reclamacao) ON DELETE CASCADE ON UPDATE CASCADE,
  id_audiencia INT NOT NULL REFERENCES audiencias(id_audiencia) ON DELETE CASCADE ON UPDATE CASCADE,
  deleted_at TIMESTAMPTZ NULL,
  PRIMARY KEY (id_reclamacao, id_audiencia)
);


CREATE TRIGGER set_timestamp_diretorios BEFORE UPDATE ON diretorios FOR EACH ROW EXECUTE FUNCTION trigger_set_timestamp();
CREATE TRIGGER set_timestamp_enderecos BEFORE UPDATE ON enderecos FOR EACH ROW EXECUTE FUNCTION trigger_set_timestamp();
CREATE TRIGGER set_timestamp_funcionarios BEFORE UPDATE ON funcionarios FOR EACH ROW EXECUTE FUNCTION trigger_set_timestamp();
CREATE TRIGGER set_timestamp_procuradores BEFORE UPDATE ON procuradores FOR EACH ROW EXECUTE FUNCTION trigger_set_timestamp();
CREATE TRIGGER set_timestamp_reclamantes BEFORE UPDATE ON reclamantes FOR EACH ROW EXECUTE FUNCTION trigger_set_timestamp();
CREATE TRIGGER set_timestamp_reclamados BEFORE UPDATE ON reclamados FOR EACH ROW EXECUTE FUNCTION trigger_set_timestamp();
CREATE TRIGGER set_timestamp_audiencias BEFORE UPDATE ON audiencias FOR EACH ROW EXECUTE FUNCTION trigger_set_timestamp();
CREATE TRIGGER set_timestamp_reclamacoes BEFORE UPDATE ON reclamacoes FOR EACH ROW EXECUTE FUNCTION trigger_set_timestamp();
CREATE TRIGGER set_timestamp_motivos BEFORE UPDATE ON motivos FOR EACH ROW EXECUTE FUNCTION trigger_set_timestamp();
CREATE TRIGGER delete_after_update_reclamacoes BEFORE UPDATE ON reclamacoes FOR WHEN (NEW.deleted_at IS NOT NULL AND OLD.deleted_at IS NULL) EXECUTE FUNCTION trigger_delete_relacao_reclamacao_reclamado();
CREATE TRIGGER delete_after_update_reclamacoes_audiencia BEFORE UPDATE ON reclamacoes FOR WHEN (NEW.deleted_at IS NOT NULL AND OLD.deleted_at IS NULL) EXECUTE FUNCTION trigger_delete_relacao_reclamacao_audiencia();
CREATE TRIGGER registrar_mudanca_status AFTER UPDATE ON reclamacoes FOR EACH ROW WHEN (OLD.status IS DISTINCT FROM NEW.status) EXECUTE FUNCTION trigger_registrar_mudanca_status();
CREATE TRIGGER set_null_fk_procurador BEFORE UPDATE ON procuradores FOR EACH ROW WHEN (NEW.deleted_at IS NOT NULL AND OLD.deleted_at IS NULL) EXECUTE PROCEDURE trigger_set_null_fk_procurador();


CREATE INDEX idx_funcionarios_id_cargo ON funcionarios(id_cargo);
CREATE INDEX idx_reclamantes_id_endereco ON reclamantes(id_endereco);
CREATE INDEX idx_reclamados_id_endereco ON reclamados(id_endereco);
CREATE INDEX idx_audiencias_id_conciliador ON audiencias(id_conciliador);
CREATE INDEX idx_reclamacoes_id_reclamante ON reclamacoes(id_reclamante);
CREATE INDEX idx_reclamacoes_id_motivo ON reclamacoes(id_motivo);
CREATE INDEX idx_reclamacoes_id_procurador ON reclamacoes(id_procurador);
CREATE INDEX idx_reclamacoes_id_criador ON reclamacoes(id_criador);
CREATE INDEX idx_reclamacoes_id_diretorio ON reclamacoes(id_diretorio);
CREATE INDEX idx_reclamacoes_status ON reclamacoes(status);
CREATE INDEX idx_historico_id_reclamacao ON historico_status_reclamacoes(id_reclamacao);
CREATE INDEX idx_rel_reclamado_id_reclamado ON relacao_reclamacao_reclamado(id_reclamado);
CREATE INDEX idx_rel_audiencia_id_audiencia ON relacao_reclamacao_audiencia(id_audiencia);


COMMENT ON FUNCTION validar_cpf IS 'Valida os dígitos verificadores de um CPF, garantindo sua autenticidade matemática.';
COMMENT ON FUNCTION validar_cnpj IS 'Valida os dígitos verificadores de um CNPJ, garantindo sua autenticidade matemática.';

COMMENT ON DOMAIN d_cpf IS 'Domínio para armazenar CPF, garantindo que contenha 11 dígitos numéricos e que seja um número válido conforme o algoritmo do governo.';
COMMENT ON DOMAIN d_cnpj IS 'Domínio para armazenar CNPJ, garantindo que contenha 14 dígitos numéricos e que seja um número válido conforme o algoritmo do governo.';

COMMENT ON COLUMN funcionarios.pwd_hash IS 'Hash da senha do funcionário. Gerado por um algoritmo seguro (ex: bcrypt via pgcrypto), o hash já contém o salt.';
COMMENT ON FUNCTION trigger_set_timestamp IS 'Atualiza a coluna updated_at para a data e hora atuais sempre que uma linha é atualizada.';
COMMENT ON FUNCTION trigger_registrar_mudanca_status IS 'Registra a mudança de status de uma reclamação na tabela de histórico.';
COMMENT ON DOMAIN d_email IS 'Domínio para validar e armazenar endereços de e-mail.';
COMMENT ON TABLE cargos IS 'Armazena os diferentes cargos que um funcionário pode ocupar.';
COMMENT ON TABLE motivos IS 'Catálogo de motivos que podem originar uma reclamação.';
COMMENT ON TABLE diretorios IS 'Armazena caminhos de diretórios no sistema de arquivos para associar a reclamações.';
COMMENT ON TABLE enderecos IS 'Armazena informações de endereço para reclamantes e reclamados.';
COMMENT ON TABLE funcionarios IS 'Registros dos funcionários do sistema, incluindo credenciais de acesso.';
COMMENT ON TABLE procuradores IS 'Registros dos procuradores (advogados) que podem representar partes.';
COMMENT ON TABLE reclamantes IS 'Armazena os dados da parte que inicia a reclamação (pessoa física ou jurídica).';
COMMENT ON TABLE reclamados IS 'Armazena os dados da parte contra quem a reclamação é feita (pessoa física ou jurídica).';
COMMENT ON TABLE audiencias IS 'Registra as audiências de conciliação associadas a uma ou mais reclamações.';
COMMENT ON TABLE reclamacoes IS 'Tabela principal, contendo os detalhes de cada reclamação registrada.';
COMMENT ON TABLE historico_status_reclamacoes IS 'Tabela de auditoria que armazena todas as mudanças de status de uma reclamação.';
COMMENT ON TABLE relacao_reclamacao_reclamado IS 'Tabela de ligação (N-para-N) entre reclamações e reclamados.';
COMMENT ON TABLE relacao_reclamacao_audiencia IS 'Tabela de ligação (N-para-N) entre reclamações e audiências.';
COMMENT ON COLUMN reclamacoes.protocolo IS 'Coluna gerada automaticamente que combina número e ano para formar um protocolo único e legível (ex: 123/2024).';
COMMENT ON COLUMN reclamantes.tipo_pessoa IS 'Define se o reclamante é Pessoa Física (Fisica) ou Jurídica (Juridica).';
COMMENT ON COLUMN reclamados.tipo_pessoa IS 'Define se o reclamado é Pessoa Física (Fisica) ou Jurídica (Juridica).';
COMMENT ON TRIGGER registrar_mudanca_status ON reclamacoes IS 'Acionado após uma atualização na tabela de reclamações para registrar a mudança de status no histórico.';
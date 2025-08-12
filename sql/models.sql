CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.data_modificacao = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TYPE meio_audiencia_enum AS ENUM ('Remoto', 'Hibrido', 'Presencial');
CREATE TYPE status_reclamacao_enum AS ENUM ('EmTramitacao', 'Arquivado', 'Desarquivado');

CREATE TABLE Cargo (
  id_cargo INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  titulo VARCHAR(100) NOT NULL UNIQUE,
  data_criacao TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE Motivo (
  id_motivo INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  nome VARCHAR(255) NOT NULL UNIQUE,
  data_criacao TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE Diretorio (
  id_diretorio INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  caminho VARCHAR(255) NOT NULL UNIQUE,
  modificavel BOOLEAN NOT NULL DEFAULT TRUE,
  data_criacao TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  data_modificacao TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER set_timestamp_diretorio
BEFORE UPDATE ON Diretorio
FOR EACH ROW
EXECUTE FUNCTION trigger_set_timestamp();

CREATE TABLE Endereco (
  id_endereco INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  cep VARCHAR(10) NOT NULL,
  logradouro VARCHAR(255) NOT NULL,
  numero VARCHAR(20) NOT NULL,
  complemento VARCHAR(100) NULL,
  bairro VARCHAR(100) NOT NULL,
  cidade VARCHAR(100) NOT NULL,
  estado VARCHAR(50) NOT NULL,
  pais VARCHAR(50) NOT NULL DEFAULT 'Brasil',
  data_criacao TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  data_modificacao TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER set_timestamp_endereco
BEFORE UPDATE ON Endereco
FOR EACH ROW
EXECUTE FUNCTION trigger_set_timestamp();

CREATE TABLE Funcionario (
  id_funcionario INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  nome VARCHAR(255) NOT NULL,
  id_cargo INT NOT NULL REFERENCES Cargo(id_cargo) ON DELETE RESTRICT ON UPDATE CASCADE,
  email VARCHAR(255) NULL UNIQUE,
  num_telefone VARCHAR(20) NULL,
  username VARCHAR(100) NOT NULL UNIQUE,
  pwd_hash VARCHAR(255) NOT NULL,
  salt VARCHAR(255) NOT NULL,
  data_criacao TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  data_modificacao TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  data_desligamento TIMESTAMPTZ NULL
);

CREATE TRIGGER set_timestamp_funcionario
BEFORE UPDATE ON Funcionario
FOR EACH ROW
EXECUTE FUNCTION trigger_set_timestamp();

CREATE TABLE Procurador (
  id_procurador INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  nome VARCHAR(255) NOT NULL,
  cpf VARCHAR(11) NOT NULL UNIQUE,
  oab VARCHAR(20) NOT NULL UNIQUE,
  email VARCHAR(255) NULL,
  num_telefone VARCHAR(20) NULL,
  data_criacao TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  data_modificacao TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER set_timestamp_procurador
BEFORE UPDATE ON Procurador
FOR EACH ROW
EXECUTE FUNCTION trigger_set_timestamp();

CREATE TABLE Reclamante (
  id_reclamante INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  tipo_pessoa CHAR(1) NOT NULL,
  nome VARCHAR(255) NOT NULL,
  cpf VARCHAR(11) NULL UNIQUE,
  cnpj VARCHAR(14) NULL UNIQUE,
  rg VARCHAR(20) NULL UNIQUE,
  id_endereco INT NOT NULL REFERENCES Endereco(id_endereco) ON DELETE RESTRICT ON UPDATE CASCADE,
  data_criacao TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  data_modificacao TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  CONSTRAINT chk_reclamante_tipo_pessoa
    CHECK ((tipo_pessoa = 'F' AND cpf IS NOT NULL AND cnpj IS NULL) OR 
           (tipo_pessoa = 'J' AND cnpj IS NOT NULL AND cpf IS NULL))
);

CREATE TRIGGER set_timestamp_reclamante
BEFORE UPDATE ON Reclamante
FOR EACH ROW
EXECUTE FUNCTION trigger_set_timestamp();

CREATE TABLE Reclamado (
  id_reclamado INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  tipo_pessoa CHAR(1) NOT NULL,
  nome VARCHAR(255) NULL,
  razao_social VARCHAR(255) NULL,
  nome_fantasia VARCHAR(255) NULL,
  cpf VARCHAR(11) NULL UNIQUE,
  cnpj VARCHAR(14) NULL UNIQUE,
  email VARCHAR(255) NULL,
  num_telefone VARCHAR(20) NULL,
  id_endereco INT NOT NULL REFERENCES Endereco(id_endereco) ON DELETE RESTRICT ON UPDATE CASCADE,
  data_criacao TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  data_modificacao TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  CONSTRAINT chk_reclamado_tipo_pessoa
    CHECK ((tipo_pessoa = 'F' AND cpf IS NOT NULL AND nome IS NOT NULL AND cnpj IS NULL AND razao_social IS NULL) OR
           (tipo_pessoa = 'J' AND cnpj IS NOT NULL AND razao_social IS NOT NULL AND cpf IS NULL AND nome IS NULL))
);

CREATE TRIGGER set_timestamp_reclamado
BEFORE UPDATE ON Reclamado
FOR EACH ROW
EXECUTE FUNCTION trigger_set_timestamp();

CREATE TABLE Audiencia (
  id_audiencia INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  id_conciliador INT NOT NULL REFERENCES Funcionario(id_funcionario) ON DELETE RESTRICT ON UPDATE CASCADE,
  data TIMESTAMPTZ NOT NULL,
  meio meio_audiencia_enum NULL,
  data_criacao TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  data_modificacao TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER set_timestamp_audiencia
BEFORE UPDATE ON Audiencia
FOR EACH ROW
EXECUTE FUNCTION trigger_set_timestamp();

CREATE TABLE Reclamacao (
  id_reclamacao INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  numero INT NOT NULL,
  ano INT NOT NULL,
  id_reclamante INT NOT NULL REFERENCES Reclamante(id_reclamante) ON DELETE RESTRICT ON UPDATE CASCADE,
  id_motivo INT NOT NULL REFERENCES Motivo(id_motivo) ON DELETE RESTRICT ON UPDATE CASCADE,
  id_procurador INT NULL REFERENCES Procurador(id_procurador) ON DELETE SET NULL ON UPDATE CASCADE,
  observacao TEXT NULL,
  atendido BOOLEAN NULL,
  id_criador INT NOT NULL REFERENCES Funcionario(id_funcionario) ON DELETE RESTRICT ON UPDATE CASCADE,
  status status_reclamacao_enum NOT NULL,
  id_diretorio INT NOT NULL REFERENCES Diretorio(id_diretorio) ON DELETE RESTRICT ON UPDATE CASCADE,
  data_criacao TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  data_modificacao TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  UNIQUE (numero, ano)
);

CREATE TRIGGER set_timestamp_reclamacao
BEFORE UPDATE ON Reclamacao
FOR EACH ROW
EXECUTE FUNCTION trigger_set_timestamp();

CREATE TABLE HistoricoStatusReclamacao (
  id_reclamacao INT NOT NULL REFERENCES Reclamacao(id_reclamacao) ON DELETE CASCADE ON UPDATE CASCADE,
  status_old status_reclamacao_enum NOT NULL,
  status_new status_reclamacao_enum NOT NULL,
  data_mudanca TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (id_reclamacao, data_mudanca)
);

CREATE TABLE RelacaoReclamacaoReclamado (
  id_reclamacao INT NOT NULL REFERENCES Reclamacao(id_reclamacao) ON DELETE CASCADE ON UPDATE CASCADE,
  id_reclamado INT NOT NULL REFERENCES Reclamado(id_reclamado) ON DELETE CASCADE ON UPDATE CASCADE,
  PRIMARY KEY (id_reclamacao, id_reclamado)
);

CREATE TABLE RelacaoReclamacaoAudiencia (
  id_reclamacao INT NOT NULL REFERENCES Reclamacao(id_reclamacao) ON DELETE CASCADE ON UPDATE CASCADE,
  id_audiencia INT NOT NULL REFERENCES Audiencia(id_audiencia) ON DELETE CASCADE ON UPDATE CASCADE,
  PRIMARY KEY (id_reclamacao, id_audiencia)
);

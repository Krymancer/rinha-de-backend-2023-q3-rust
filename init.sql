
CREATE TABLE IF NOT EXISTS PESSOAS (
    ID VARCHAR(36),
    APELIDO VARCHAR(32) CONSTRAINT ID_PK PRIMARY KEY,
    NOME VARCHAR(100),
    NASCIMENTO CHAR(10),
    STACK VARCHAR(1024),
);
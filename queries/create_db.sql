CREATE TABLE IF NOT EXISTS Treno(Numero INTEGER PRIMARY KEY, Categoria TEXT NOT NULL);
DO
$$
    BEGIN
IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'tipopdp') THEN
CREATE TYPE TipoPdP AS ENUM ('Stazione', 'Semplice', 'Scambio')§
END IF§
END
$$;
CREATE TABLE IF NOT EXISTS PuntoDiPassaggio(ID SERIAL PRIMARY KEY, Latitudine REAL NOT NULL, Longitudine REAL NOT NULL, Tipo TipoPdP NOT NULL);
CREATE TABLE IF NOT EXISTS AttraversamentoTeorico(IDTreno INTEGER NOT NULL, IDPdP INTEGER NOT NULL, Orario TIME NOT NULL,
    CONSTRAINT fk_treno FOREIGN KEY (IDTreno) REFERENCES Treno(Numero),
    CONSTRAINT fk_pdp FOREIGN KEY (IDPdP) REFERENCES PuntoDiPassaggio(ID));
CREATE TABLE IF NOT EXISTS Attraversamento(IDTreno INTEGER UNIQUE NOT NULL, IDPdP INTEGER UNIQUE NOT NULL, Data TIMESTAMP NOT NULL,
    CONSTRAINT fk_treno FOREIGN KEY (IDTreno) REFERENCES Treno(Numero),
    CONSTRAINT fk_pdp FOREIGN KEY (IDPdP) REFERENCES PuntoDiPassaggio(ID));
CREATE TABLE IF NOT EXISTS PdPStazione(ID SERIAL PRIMARY KEY, IDPdP INTEGER UNIQUE NOT NULL, Nome TEXT NOT NULL, Binario TEXT NOT NULL,
    CONSTRAINT fk_pdp FOREIGN KEY (IDPdP) REFERENCES PuntoDiPassaggio(ID));
DO
$$
    BEGIN
IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'tiporuolo') THEN
CREATE TYPE TipoRuolo AS ENUM ('Macchinista', 'Capotreno', 'Controllore')§
END IF§
END
$$;
CREATE TABLE IF NOT EXISTS Turno(IDPersona INTEGER NOT NULL, Ruolo TipoRuolo, IDTreno INTEGER NOT NULL, Data DATE NOT NULL);
CREATE TABLE IF NOT EXISTS Persona(ID SERIAL PRIMARY KEY, Nome TEXT, Cognome TEXT);
CREATE TABLE IF NOT EXISTS Locomotiva(ID VARCHAR(12) PRIMARY KEY, Velocita INTEGER NOT NULL, Tensione TEXT NOT NULL);
CREATE TABLE IF NOT EXISTS Carrozza(ID VARCHAR(12) PRIMARY KEY, Classe INTEGER NOT NULL, Posti INTEGER NOT NULL);
CREATE TABLE IF NOT EXISTS Esercizio(IDConvoglio VARCHAR(12) NOT NULL, IDTreno INTEGER NOT NULL, Data TIMESTAMP NOT NULL);
CREATE OR REPLACE VIEW RitardoPdP AS SELECT categoria, numero, at.idpdp, (CURRENT_DATE + COALESCE(OrarioPartenza, OrarioArrivo)) AS orario, data,
       GREATEST(0, EXTRACT(EPOCH FROM (COALESCE(a.data AT TIME ZONE 'Europe/Rome', NOW())) -
       (CURRENT_DATE + COALESCE(at.OrarioPartenza, at.OrarioArrivo)) AT TIME ZONE 'Europe/Rome') / 60) AS ritardo
    FROM treno
        JOIN attraversamentoteorico at on treno.numero = at.idtreno
        LEFT OUTER JOIN (SELECT * FROM attraversamento WHERE data::date = now()::date) a on treno.numero = a.idtreno AND a.idpdp = at.idpdp AND a.idtreno = at.idtreno;
CREATE OR REPLACE VIEW RitardoTreno AS SELECT DISTINCT LAST_VALUE(ritardo) OVER(PARTITION BY numero ORDER BY orario RANGE BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING) ritardo, numero FROM ritardopdp;
CREATE OR REPLACE VIEW DestinazioneTreno AS SELECT DISTINCT LAST_VALUE(Nome) OVER(PARTITION BY numero ORDER BY orario RANGE BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING) Nome, numero FROM ritardopdp JOIN PdPStazione s ON s.IDPdP = RitardoPdP.IDPdP;
ALTER TABLE Attraversamento RENAME COLUMN Data TO DataArrivo;
ALTER TABLE AttraversamentoTeorico RENAME COLUMN Orario TO OrarioArrivo;
ALTER TABLE Attraversamento ADD COLUMN DataPartenza TIMESTAMP;
ALTER TABLE AttraversamentoTeorico ADD COLUMN OrarioPartenza TIME;
ALTER TABLE Attraversamento ALTER COLUMN DataArrivo DROP NOT NULL;
ALTER TABLE AttraversamentoTeorico ALTER COLUMN OrarioArrivo DROP NOT NULL;

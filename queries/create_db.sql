CREATE TABLE IF NOT EXISTS Treno
(
    Numero    INTEGER PRIMARY KEY,
    Categoria TEXT NOT NULL
);
DO
$$
    BEGIN
IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'tipopdp') THEN
CREATE TYPE TipoPdP AS ENUM ('Stazione', 'Semplice', 'Scambio')§
END IF§
END
$$;
CREATE TABLE IF NOT EXISTS PuntoDiPassaggioAstratto
(
    ID   SERIAL PRIMARY KEY,
    Nome Text
);
CREATE TABLE IF NOT EXISTS PuntoDiPassaggio
(
    ID          SERIAL PRIMARY KEY,
    IDAstratto  INTEGER NOT NULL,
    Latitudine  REAL    NOT NULL,
    Longitudine REAL    NOT NULL,
    Tipo TipoPdP NOT NULL,
    CONSTRAINT fk_pdpa FOREIGN KEY (IDAstratto) REFERENCES PuntoDiPassaggioAstratto (ID)
);
CREATE TABLE IF NOT EXISTS AttraversamentoTeorico
(
    IDTreno        INTEGER NOT NULL,
    IDPdP          INTEGER NOT NULL,
    OrarioArrivo   TIME,
    OrarioPartenza TIME,
    CONSTRAINT fk_treno FOREIGN KEY (IDTreno) REFERENCES Treno (Numero),
    CONSTRAINT fk_pdp FOREIGN KEY (IDPdP) REFERENCES PuntoDiPassaggioAstratto (ID)
);
CREATE TABLE IF NOT EXISTS Attraversamento
(
    IDTreno      INTEGER UNIQUE NOT NULL,
    IDPdP        INTEGER UNIQUE NOT NULL,
    DataArrivo   TIMESTAMP,
    DataPartenza TIMESTAMP,
    CONSTRAINT fk_treno FOREIGN KEY (IDTreno) REFERENCES Treno (Numero),
    CONSTRAINT fk_pdp FOREIGN KEY (IDPdP) REFERENCES PuntoDiPassaggio (ID)
);
CREATE TABLE IF NOT EXISTS PdPStazione
(
    ID      SERIAL PRIMARY KEY,
    IDPdP   INTEGER UNIQUE NOT NULL,
    Nome    TEXT           NOT NULL,
    Binario TEXT           NOT NULL,
    CONSTRAINT fk_pdp FOREIGN KEY (IDPdP) REFERENCES PuntoDiPassaggio (ID)
);
DO
$$
    BEGIN
IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'tiporuolo') THEN
CREATE TYPE TipoRuolo AS ENUM ('Macchinista', 'Capotreno', 'Controllore')§
END IF§
END
$$;
CREATE TABLE IF NOT EXISTS Turno
(
    IDPersona INTEGER NOT NULL,
    Ruolo     TipoRuolo NOT NULL,
    IDTreno   INTEGER NOT NULL,
    Data      DATE    NOT NULL
);
CREATE TABLE IF NOT EXISTS Persona
(
    ID      SERIAL PRIMARY KEY,
    Nome    TEXT,
    Cognome TEXT
);
CREATE TABLE IF NOT EXISTS Locomotiva
(
    ID       VARCHAR(12) PRIMARY KEY,
    Velocita INTEGER NOT NULL,
    Tensione TEXT    NOT NULL
);
CREATE TABLE IF NOT EXISTS Carrozza
(
    ID     VARCHAR(12) PRIMARY KEY,
    Classe INTEGER NOT NULL,
    Posti  INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS Convoglio
(
    ID         INTEGER     NOT NULL,
    IDCarrozza VARCHAR(12) NOT NULL
);
CREATE TABLE IF NOT EXISTS Esercizio
(
    IDConvoglio INTEGER NOT NULL,
    IDTreno     INTEGER NOT NULL,
    Data        DATE    NOT NULL
);

CREATE OR REPLACE VIEW RitardoPdP AS
SELECT at.OrarioArrivo,
       at.OrarioPartenza,
       a.DataArrivo,
       a.DataPartenza,
       categoria,
       numero,
       at.idpdp,
       (CURRENT_DATE + COALESCE(OrarioPartenza, OrarioArrivo)) AS orario,
       COALESCE(datapartenza, dataarrivo)                      as data,
       GREATEST(0, EXTRACT(EPOCH FROM (COALESCE(a.datapartenza, a.dataarrivo, NOW()) AT TIME ZONE 'Europe/Rome') -
                                      (CURRENT_DATE + COALESCE(at.OrarioPartenza, at.OrarioArrivo)) AT TIME ZONE
                           'Europe/Rome') / 60)                AS ritardo
FROM treno
         JOIN attraversamentoteorico at on treno.numero = at.idtreno
         JOIN puntodipassaggioastratto pdpa on at.idpdp = pdpa.id
         JOIN puntodipassaggio pdp on pdpa.id = pdp.idastratto
         LEFT OUTER JOIN (SELECT * FROM attraversamento WHERE COALESCE(datapartenza, dataarrivo)::date = now()::date) a
                         on treno.numero = a.idtreno AND a.idtreno = at.idtreno AND a.idpdp = pdp.id AND
                            pdpa.id = pdp.idastratto;


CREATE OR REPLACE VIEW RitardoTreno AS
SELECT DISTINCT LAST_VALUE(ritardo)
                           OVER (PARTITION BY numero ORDER BY orario RANGE BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING) ritardo,
                numero
FROM ritardopdp;
CREATE OR REPLACE VIEW DestinazioneTreno AS
SELECT DISTINCT LAST_VALUE(Nome)
                           OVER (PARTITION BY numero ORDER BY orario RANGE BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING) Nome,
                numero
FROM ritardopdp
         JOIN PdPStazione s ON s.IDPdP = RitardoPdP.IDPdP;
CREATE OR REPLACE VIEW TurniPersona AS
SELECT *
FROM Persona
         JOIN Turno ON Turno.IDPersona = Persona.ID;
CREATE OR REPLACE VIEW Composizione AS
SELECT esercizio.*, carrozza.*
FROM Esercizio
         JOIN Convoglio ON Esercizio.IDConvoglio = Convoglio.ID
         JOIN Carrozza on Convoglio.IDCarrozza = Carrozza.ID;

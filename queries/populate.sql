/* treno */
insert into treno (numero, categoria) values (1, 'RV');
insert into treno (numero, categoria) values (2, 'RV');

/* pdp */
insert into puntodipassaggioastratto (id, nome) values (101, 'Bologna Centrale');
insert into puntodipassaggioastratto (id, nome) values (102, 'Imola');
insert into puntodipassaggioastratto (id, nome) values (103, 'Faenza');
insert into puntodipassaggioastratto (id, nome) values (104, 'Cesena');
insert into puntodipassaggio (id, idastratto, latitudine, longitudine, tipo) values (101, 101, 44.5062984, 11.3434174, 'Stazione');
insert into puntodipassaggio (id, idastratto, latitudine, longitudine, tipo) values (102, 102, 44.3589670, 11.7187788, 'Stazione');
insert into puntodipassaggio (id, idastratto, latitudine, longitudine, tipo) values (103, 103, 44.2935799, 11.8832186, 'Stazione');
insert into puntodipassaggio (id, idastratto, latitudine, longitudine, tipo) values (104, 104, 44.1455039, 12.2495058, 'Stazione');
insert into pdpstazione (id, idpdp, binario) values (101, 101, '3');
insert into pdpstazione (id, idpdp, binario) values (102, 102, '2');
insert into pdpstazione (id, idpdp, binario) values (103, 103, '2');
insert into pdpstazione (id, idpdp, binario) values (104, 104, '1');

/* attraversamento teorico */
insert into attraversamentoteorico (idtreno, idpdp, orarioarrivo, orariopartenza) values (1,101,null,'10:01:00');
insert into attraversamentoteorico (idtreno, idpdp, orarioarrivo, orariopartenza) values (1,102,'10:20:00','10:21:00');
insert into attraversamentoteorico (idtreno, idpdp, orarioarrivo, orariopartenza) values (1,103,'10:35:00','10:36:00');
insert into attraversamentoteorico (idtreno, idpdp, orarioarrivo, orariopartenza) values (1,104,'11:00:00',null);

insert into attraversamentoteorico (idtreno, idpdp, orarioarrivo, orariopartenza) values (2,104,null,'11:05:00');
insert into attraversamentoteorico (idtreno, idpdp, orarioarrivo, orariopartenza) values (2,103,'11:25:00','11:26:00');
insert into attraversamentoteorico (idtreno, idpdp, orarioarrivo, orariopartenza) values (2,102,'11:45:00',null);

/* carrozza, locomotiva, convoglio */
insert into locomotiva (id, velocita, tensione) values ('l10000000000',180,'AC');
insert into carrozza (id, classe, posti) values('c10000000000',1,30);
insert into carrozza (id, classe, posti) values('c10000000001',2,30);
insert into carrozza (id, classe, posti) values('c10000000002',2,30);
insert into carrozza (id, classe, posti) values('c10000000003',2,30);
insert into carrozza (id, classe, posti) values('c10000000004',2,30);
insert into convoglio (id,idcarrozza) values (1,'c10000000000');
insert into convoglio (id,idcarrozza) values (1,'c10000000001');
insert into convoglio (id,idcarrozza) values (1,'c10000000002');
insert into convoglio (id,idcarrozza) values (1,'c10000000003');
insert into convoglio (id,idcarrozza) values (1,'c10000000004');

insert into locomotiva (id, velocita, tensione) values ('l20000000000',180,'CC');
insert into carrozza (id, classe, posti) values('c20000000000',1,30);
insert into carrozza (id, classe, posti) values('c20000000001',2,30);
insert into carrozza (id, classe, posti) values('c20000000002',2,30);
insert into carrozza (id, classe, posti) values('c20000000003',2,30);
insert into carrozza (id, classe, posti) values('c20000000004',2,30);
insert into convoglio (id,idcarrozza) values (2,'c20000000000');
insert into convoglio (id,idcarrozza) values (2,'c20000000001');
insert into convoglio (id,idcarrozza) values (2,'c20000000002');
insert into convoglio (id,idcarrozza) values (2,'c20000000003');
insert into convoglio (id,idcarrozza) values (2,'c20000000004');

/* esercizio */
insert into esercizio (idconvoglio, idtreno, idlocomotiva, data) values (1,1,'l10000000000','2022-07-14');
insert into esercizio (idconvoglio, idtreno, idlocomotiva, data) values (1,1,'l10000000000','2022-07-15');
insert into esercizio (idconvoglio, idtreno, idlocomotiva, data) values (1,1,'l10000000000','2022-07-16');
insert into esercizio (idconvoglio, idtreno, idlocomotiva, data) values (1,1,'l10000000000','2022-07-17');
insert into esercizio (idconvoglio, idtreno, idlocomotiva, data) values (1,1,'l10000000000','2022-07-18');
insert into esercizio (idconvoglio, idtreno, idlocomotiva, data) values (1,1,'l10000000000','2022-07-19');
insert into esercizio (idconvoglio, idtreno, idlocomotiva, data) values (1,1,'l10000000000','2022-07-20');
insert into esercizio (idconvoglio, idtreno, idlocomotiva, data) values (1,1,'l10000000000','2022-07-21');
insert into esercizio (idconvoglio, idtreno, idlocomotiva, data) values (1,1,'l10000000000','2022-07-22');
insert into esercizio (idconvoglio, idtreno, idlocomotiva, data) values (1,1,'l10000000000','2022-07-23');
insert into esercizio (idconvoglio, idtreno, idlocomotiva, data) values (1,1,'l10000000000','2022-07-24');
insert into esercizio (idconvoglio, idtreno, idlocomotiva, data) values (1,1,'l10000000000','2022-07-25');
insert into esercizio (idconvoglio, idtreno, idlocomotiva, data) values (1,1,'l10000000000','2022-07-26');
insert into esercizio (idconvoglio, idtreno, idlocomotiva, data) values (1,1,'l10000000000','2022-07-27');
insert into esercizio (idconvoglio, idtreno, idlocomotiva, data) values (1,1,'l10000000000','2022-07-28');
insert into esercizio (idconvoglio, idtreno, idlocomotiva, data) values (1,1,'l10000000000','2022-07-29');
insert into esercizio (idconvoglio, idtreno, idlocomotiva, data) values (1,1,'l10000000000','2022-07-30');
insert into esercizio (idconvoglio, idtreno, idlocomotiva, data) values (1,1,'l10000000000','2022-07-31');

insert into esercizio (idconvoglio, idtreno, idlocomotiva, data) values (2,2,'l20000000000','2022-07-14');
insert into esercizio (idconvoglio, idtreno, idlocomotiva, data) values (2,2,'l20000000000','2022-07-15');
insert into esercizio (idconvoglio, idtreno, idlocomotiva, data) values (2,2,'l20000000000','2022-07-16');
insert into esercizio (idconvoglio, idtreno, idlocomotiva, data) values (2,2,'l20000000000','2022-07-17');
insert into esercizio (idconvoglio, idtreno, idlocomotiva, data) values (2,2,'l20000000000','2022-07-18');
insert into esercizio (idconvoglio, idtreno, idlocomotiva, data) values (2,2,'l20000000000','2022-07-19');
insert into esercizio (idconvoglio, idtreno, idlocomotiva, data) values (2,2,'l20000000000','2022-07-20');
insert into esercizio (idconvoglio, idtreno, idlocomotiva, data) values (2,2,'l20000000000','2022-07-21');
insert into esercizio (idconvoglio, idtreno, idlocomotiva, data) values (2,2,'l20000000000','2022-07-22');
insert into esercizio (idconvoglio, idtreno, idlocomotiva, data) values (2,2,'l20000000000','2022-07-23');
insert into esercizio (idconvoglio, idtreno, idlocomotiva, data) values (2,2,'l20000000000','2022-07-24');
insert into esercizio (idconvoglio, idtreno, idlocomotiva, data) values (2,2,'l20000000000','2022-07-25');
insert into esercizio (idconvoglio, idtreno, idlocomotiva, data) values (2,2,'l20000000000','2022-07-26');
insert into esercizio (idconvoglio, idtreno, idlocomotiva, data) values (2,2,'l20000000000','2022-07-27');
insert into esercizio (idconvoglio, idtreno, idlocomotiva, data) values (2,2,'l20000000000','2022-07-28');
insert into esercizio (idconvoglio, idtreno, idlocomotiva, data) values (2,2,'l20000000000','2022-07-29');
insert into esercizio (idconvoglio, idtreno, idlocomotiva, data) values (2,2,'l20000000000','2022-07-30');
insert into esercizio (idconvoglio, idtreno, idlocomotiva, data) values (2,2,'l20000000000','2022-07-31');

/* persona */
insert into persona (id, nome, cognome, ruolo) values (101, 'Mario', 'Rossi', 'Macchinista');
insert into persona (id ,nome, cognome, ruolo) values (102, 'Luigi', 'Verdi', 'Capotreno');

/* turno */
insert into turno (idpersona, idtreno, data) values (101,1,'2022-07-14');
insert into turno (idpersona, idtreno, data) values (102,1,'2022-07-14');
insert into turno (idpersona, idtreno, data) values (101,2,'2022-07-14');
insert into turno (idpersona, idtreno, data) values (102,2,'2022-07-14');

insert into turno (idpersona, idtreno, data) values (101,1,'2022-07-15');
insert into turno (idpersona, idtreno, data) values (102,1,'2022-07-15');
insert into turno (idpersona, idtreno, data) values (101,2,'2022-07-15');
insert into turno (idpersona, idtreno, data) values (102,2,'2022-07-15');

insert into turno (idpersona, idtreno, data) values (101,1,'2022-07-16');
insert into turno (idpersona, idtreno, data) values (102,1,'2022-07-16');
insert into turno (idpersona, idtreno, data) values (101,2,'2022-07-16');
insert into turno (idpersona, idtreno, data) values (102,2,'2022-07-16');

insert into turno (idpersona, idtreno, data) values (101,1,'2022-07-17');
insert into turno (idpersona, idtreno, data) values (102,1,'2022-07-17');
insert into turno (idpersona, idtreno, data) values (101,2,'2022-07-17');
insert into turno (idpersona, idtreno, data) values (102,2,'2022-07-17');

insert into turno (idpersona, idtreno, data) values (101,1,'2022-07-18');
insert into turno (idpersona, idtreno, data) values (102,1,'2022-07-18');
insert into turno (idpersona, idtreno, data) values (101,2,'2022-07-18');
insert into turno (idpersona, idtreno, data) values (102,2,'2022-07-18');

insert into turno (idpersona, idtreno, data) values (101,1,'2022-07-19');
insert into turno (idpersona, idtreno, data) values (102,1,'2022-07-19');
insert into turno (idpersona, idtreno, data) values (101,2,'2022-07-19');
insert into turno (idpersona, idtreno, data) values (102,2,'2022-07-19');

insert into turno (idpersona, idtreno, data) values (101,1,'2022-07-20');
insert into turno (idpersona, idtreno, data) values (102,1,'2022-07-20');
insert into turno (idpersona, idtreno, data) values (101,2,'2022-07-20');
insert into turno (idpersona, idtreno, data) values (102,2,'2022-07-20');

insert into turno (idpersona, idtreno, data) values (101,1,'2022-07-21');
insert into turno (idpersona, idtreno, data) values (102,1,'2022-07-21');
insert into turno (idpersona, idtreno, data) values (101,2,'2022-07-21');
insert into turno (idpersona, idtreno, data) values (102,2,'2022-07-21');

insert into turno (idpersona, idtreno, data) values (101,1,'2022-07-22');
insert into turno (idpersona, idtreno, data) values (102,1,'2022-07-22');
insert into turno (idpersona, idtreno, data) values (101,2,'2022-07-22');
insert into turno (idpersona, idtreno, data) values (102,2,'2022-07-22');

insert into turno (idpersona, idtreno, data) values (101,1,'2022-07-23');
insert into turno (idpersona, idtreno, data) values (102,1,'2022-07-23');
insert into turno (idpersona, idtreno, data) values (101,2,'2022-07-23');
insert into turno (idpersona, idtreno, data) values (102,2,'2022-07-23');

insert into turno (idpersona, idtreno, data) values (101,1,'2022-07-24');
insert into turno (idpersona, idtreno, data) values (102,1,'2022-07-24');
insert into turno (idpersona, idtreno, data) values (101,2,'2022-07-24');
insert into turno (idpersona, idtreno, data) values (102,2,'2022-07-24');

insert into turno (idpersona, idtreno, data) values (101,1,'2022-07-25');
insert into turno (idpersona, idtreno, data) values (102,1,'2022-07-25');
insert into turno (idpersona, idtreno, data) values (101,2,'2022-07-25');
insert into turno (idpersona, idtreno, data) values (102,2,'2022-07-25');

insert into turno (idpersona, idtreno, data) values (101,1,'2022-07-26');
insert into turno (idpersona, idtreno, data) values (102,1,'2022-07-26');
insert into turno (idpersona, idtreno, data) values (101,2,'2022-07-26');
insert into turno (idpersona, idtreno, data) values (102,2,'2022-07-26');

insert into turno (idpersona, idtreno, data) values (101,1,'2022-07-27');
insert into turno (idpersona, idtreno, data) values (102,1,'2022-07-27');
insert into turno (idpersona, idtreno, data) values (101,2,'2022-07-27');
insert into turno (idpersona, idtreno, data) values (102,2,'2022-07-27');

insert into turno (idpersona, idtreno, data) values (101,1,'2022-07-28');
insert into turno (idpersona, idtreno, data) values (102,1,'2022-07-28');
insert into turno (idpersona, idtreno, data) values (101,2,'2022-07-28');
insert into turno (idpersona, idtreno, data) values (102,2,'2022-07-28');

insert into turno (idpersona, idtreno, data) values (101,1,'2022-07-29');
insert into turno (idpersona, idtreno, data) values (102,1,'2022-07-29');
insert into turno (idpersona, idtreno, data) values (101,2,'2022-07-29');
insert into turno (idpersona, idtreno, data) values (102,2,'2022-07-29');

insert into turno (idpersona, idtreno, data) values (101,1,'2022-07-30');
insert into turno (idpersona, idtreno, data) values (102,1,'2022-07-30');
insert into turno (idpersona, idtreno, data) values (101,2,'2022-07-30');
insert into turno (idpersona, idtreno, data) values (102,2,'2022-07-30');

insert into turno (idpersona, idtreno, data) values (101,1,'2022-07-31');
insert into turno (idpersona, idtreno, data) values (102,1,'2022-07-31');
insert into turno (idpersona, idtreno, data) values (101,2,'2022-07-31');
insert into turno (idpersona, idtreno, data) values (102,2,'2022-07-31');



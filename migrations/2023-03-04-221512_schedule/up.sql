CREATE TABLE IF NOT EXISTS PanelSchedule (
     id serial NOT NULL,
     name varchar(50) not null,
     description varchar(50),
     created_on TIMESTAMP NOT NULL,
     starts_on TIMESTAMP NOT NULL,
     ends_on TIMESTAMP NOT NULL,
     starts_on_originally TIMESTAMP NOT NULL,
     ends_on_originally TIMESTAMP NOT NULL,
     CONSTRAINT Events PRIMARY KEY (id)
);

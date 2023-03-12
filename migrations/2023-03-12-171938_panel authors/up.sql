CREATE TABLE IF NOT EXISTS PanelAuthor (
    id serial NOT NULL,
    panelScheduleId serial NOT NULL,
    name VARCHAR(50) NOT NULL,
    CONSTRAINT fk_author_id PRIMARY KEY (id),
    CONSTRAINT fk_author_panel_id FOREIGN KEY(id)
    REFERENCES PanelSchedule(id) 
);
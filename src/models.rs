use crate::schema::panelschedule;

#[derive(Debug, Insertable, Queryable)]
#[diesel(table_name = panelschedule)]
pub struct NewScheduledPanel {
    pub name: String,
    pub description: Option<String>,
    pub created_on : chrono::naive::NaiveDateTime,
    pub starts_on : chrono::naive::NaiveDateTime,
    pub ends_on : chrono::naive::NaiveDateTime,
    pub starts_on_originally: chrono::naive::NaiveDateTime,
    pub ends_on_originally: chrono::naive::NaiveDateTime,
}

#[derive(Debug, Queryable, Insertable)]
#[diesel(table_name = panelschedule)]
pub struct ScheduledPanel {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_on : chrono::naive::NaiveDateTime,
    pub starts_on : chrono::naive::NaiveDateTime,
    pub ends_on : chrono::naive::NaiveDateTime,
    pub starts_on_originally: chrono::naive::NaiveDateTime,
    pub ends_on_originally: chrono::naive::NaiveDateTime,
}

use crate::schema::quiz;

#[derive(Serialize, Deserialize, Queryable)]
pub struct Quiz {
    pub id: i32,
    pub question: String,
    pub answer: String,
    pub hint: Option<String>,
    pub correct_answers: i32,
    pub total_answers: i32
}

#[derive(Deserialize, Insertable)]
#[table_name = "quiz"]
pub struct InsertableQuiz {
    pub question: String,
    pub answer: String,
    pub hint: Option<String>,
}
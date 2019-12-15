table! {
    quiz (id) {
        id -> Int4,
        question -> Varchar,
        answer -> Varchar,
        hint -> Nullable<Varchar>,
        correct_answers -> Int4,
        total_answers -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    quiz,
);

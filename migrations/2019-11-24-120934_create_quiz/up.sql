-- Your SQL goes here
CREATE TABLE quiz (
  id SERIAL PRIMARY KEY,
  question VARCHAR NOT NULL,
  answer VARCHAR NOT NULL,
  hint VARCHAR,
  correct_answers INTEGER DEFAULT 0 NOT NULL,
  total_answers INTEGER DEFAULT 0 NOT NULL
);
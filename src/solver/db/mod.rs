/// Database.

#[allow(unused_variables)]
pub fn create_solutions_table() {
    let s = "CREATE TABLE solutions (
  game BIT(152) PRIMARY KEY,
  play SMALLINT,
  winner SMALLINT
    CONSTRAINT winner_constraint
    CHECK (winner >= 0 AND winner <= 2),
  turns SMALLINT NOT NULL
    CONSTRAINT turns_constraint
    CHECK (turns >= 0)
);";
    unimplemented!()
}

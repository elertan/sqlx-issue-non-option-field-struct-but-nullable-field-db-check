use sqlx::postgres::PgQueryAs;
use sqlx::PgPool;

#[derive(Debug, sqlx::FromRow)]
pub struct TblTest {
    pub id: String,
    pub field: String,
}

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    let url = "postgres://docker:docker@localhost/app";

    let pool = PgPool::builder().build(url).await?;

    sqlx::query(
        "
create table if not exists tbl_test
(
	id varchar(2) not null
		constraint tbl_test_pk
			primary key,
	field varchar(255) not null
);
    ",
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "
insert into tbl_test
(id, field)
values
('OK', 'Ok!');
    ",
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "
insert into tbl_test
(id, field)
values
('NO', 'Not ok!');
    ",
    )
    .execute(&pool)
    .await?;

    let results: Vec<TblTest> = sqlx::query_as("SELECT * FROM tbl_test WHERE id = any($1);")
        .bind(vec!["OK"])
        .fetch_all(&pool)
        .await?;

    println!("{:?}", results);

    Ok(())
}

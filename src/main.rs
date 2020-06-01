use sqlx::PgPool;

// Create a table like this
//
// create table tbl_test
// (
// 	id serial not null
// 		constraint tbl_test_pk
// 			primary key,
// 	nullable_field varchar(50)
// );

#[derive(Debug)]
pub struct TblTest {
    pub id: i32,
    pub nullable_field: String,
}
// ^
// |
// -- Struct doesn't get checked at compile to equal non nullable field

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    let url = "postgres://docker:docker@localhost/app";

    let pool = PgPool::builder().build(url).await?;

    sqlx::query!("INSERT INTO tbl_test (nullable_field) VALUES (NULL);")
        .execute(&pool)
        .await?;

    let results: Vec<TblTest> = sqlx::query_as!(TblTest, "SELECT * FROM tbl_test",)
        .fetch_all(&pool)
        .await?;

    println!("{:?}", results);

    Ok(())
}



fn sql_table() {
    let connection = sqlite::open(":memory:").unwrap();
    connection.execute(
        "
        CREATE TABLE users (name TEXT, age INTEGER);
        ",
    ).unwrap();}
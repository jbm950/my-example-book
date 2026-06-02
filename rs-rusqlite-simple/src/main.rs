use rusqlite::{Connection, Error, params};

#[derive(Debug)]
struct Person {
    name: String,
    age: i64,
}

fn main() -> Result<(), Error> {
    let conn = Connection::open("./my_db.db3")?;

    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS person (
            name TEXT NOT NULL,
            age INTEGER
        )",
    )?;

    let people = [
        Person {
            name: "Steve".into(),
            age: 24,
        },
        Person {
            name: "Martha".into(),
            age: 45,
        },
    ];

    // Tuple for parameters
    conn.execute(
        "INSERT INTO person (name, age) VALUES (?1, ?2)",
        (&people[0].name, people[0].age),
    )?;

    // Using `params![...]`
    conn.execute(
        "INSERT INTO person (name, age) VALUES (?1, ?2)",
        params![&people[1].name, people[1].age],
    )?;

    let mut stmt = conn.prepare("SELECT name, age FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            name: row.get("name")?,
            age: row.get("age")?,
        })
    })?;

    for person in person_iter {
        println!("Found person: {:?}", person?);
    }

    Ok(())
}

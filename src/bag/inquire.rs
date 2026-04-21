use inquire::error::CustomUserError;
use std::collections::HashSet;
use rusqlite::Connection; // Assume coffees exist!

pub fn coffee_suggestor(input: &str) -> Result<Vec<String>, CustomUserError> {
    suggestor(input, "SELECT name FROM coffee")
}

fn suggestor(input: &str, sql: &str) -> Result<Vec<String>, CustomUserError> {
    let input = input.to_lowercase();

    Ok(get_suggestions_from_db(sql)?
        .into_iter()
        .collect::<HashSet<_>>() // Remove duplicates
        .into_iter()
        .filter(|p| !p.is_empty())
        .filter(|p| p.to_lowercase().contains(&input))
        .map(|p| String::from(p))
        .collect())
}

fn get_suggestions_from_db(sql: &str) -> Result<Vec<String>, rusqlite::Error>{
    let conn = Connection::open("./kaffe.db")?;

    // Prepare the query
    let mut stmt = conn.prepare(sql)?;

    // Query and map rows to strings
    let suggestions_iter = stmt.query_map([], |row| {
        row.get::<_, String>(0) // Get the first column as String
    })?;

    // Collect into a Vec
    let mut suggestion_list = Vec::new();
    for suggestion in suggestions_iter {
        suggestion_list.push(suggestion?);
    }

    Ok(suggestion_list)
}
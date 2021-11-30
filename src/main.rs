// task: get random cat facts from catfact.ninja asynchronously
// API: GET from /fact (parameter max_length for string length limit)
// receive json object with fact and length property

use async_std::fs::ReadDir;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct CatFact {
    fact: String,
    length: u64,
}

fn blocking_fact(source: &str) -> Result<CatFact, surf::Error> {
    async_std::task::block_on(surf::get(source).recv_json::<CatFact>())
}

fn main() {
    let cf = "https://catfact.ninja/fact";
    let fact = match blocking_fact(cf) {
        Ok(x) => x.fact,
        Err(e) => format!(
            "Sorry, there was an error getting a cat fact: \n\n       {}",
            e
        ),
    };

    println!("\n{}\n", fact);
}

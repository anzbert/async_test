// task: get random cat facts from catfact.ninja asynchronously
// API: GET from /fact (parameter max_length for string length limit)
// receive json object with fact and length property

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct CatFact {
    fact: String,
    length: u64,
}

fn main() {
    let cf = "https://catfact.ninja/fact";
    let fact = match async_std::task::block_on(surf::get(cf).recv_json::<CatFact>()) {
        Ok(x) => x.fact,
        Err(e) => format!("Sorry, there was an error getting a cat fact: {}", e),
    };

    println!("Cat Fact: {}", fact);
}

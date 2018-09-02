#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "{ 'ticker': 'vti', 'fee': .03, 'currentGoal': 33, 'currentPercent': 33 }"
    // "hello"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}

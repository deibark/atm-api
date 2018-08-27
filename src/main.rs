#![feature(plugin)]
#![plugin(rocket_codegen)]

mod atm;

extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::{Json, Value};

#[get("/withdraw/<amount>")]
fn index(amount: u32) -> Json<Value> {

    match atm::withdraw(amount) {
        Ok(val) => { Json(json!(val)) },
        Err(err) => { Json(json!(err)) }
    }

}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/atm", routes![index])
}

fn main() {
    rocket().launch();
}

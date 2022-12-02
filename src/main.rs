use rocket_okapi::{openapi_get_routes, swagger_ui::*};

#[rocket::main]
async fn main() {
    let launch_result = rocket::build()
        .mount("/", openapi_get_routes![])
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .launch()
        .await;
    match launch_result {
        Ok(_) => println!("Rocket shut down gracefully."),
        Err(err) => println!("Rocket had an error: {}", err),
    };
}

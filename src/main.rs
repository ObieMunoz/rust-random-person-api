use rand::Rng;
use serde::Serialize;
use warp::Filter;

#[derive(Serialize)]
struct Person {
    name: String,
    address: String,
    phone_number: String,
}

#[tokio::main]
async fn main() {
    let api = warp::path("api")
        .and(warp::path("random_person"))
        .and(warp::get())
        .map(random_person)
        .with(warp::reply::with::header("Content-Type", "application/json"));

    let routes = api.with(warp::cors().allow_any_origin());

    let log = warp::log::custom(|info| {
        let user_agent = info
            .request_headers()
            .get("User-Agent")
            .map_or_else(|| "unknown", |header_value| header_value.to_str().unwrap_or("unknown"));

        let remote_addr = info.remote_addr().map_or_else(|| "unknown".to_string(), |addr| format!("{}", addr));

        println!(
            "{} \"{} {} {:?}\" {:?} \"{}\" \"{}\"",
            remote_addr,
            info.method(),
            info.path(),
            info.version(),
            info.status(),
            user_agent,
            info.referer().unwrap_or("unknown"),
        );
    });

    let routes = routes.with(log);

    println!("Starting server on 127.0.0.1:4173");
    warp::serve(routes).run(([127, 0, 0, 1], 4173)).await;
}

fn random_person() -> warp::reply::Json {
    let name = generate_random_name();
    let address = generate_random_address();
    let phone_number = generate_random_phone_number();

    let person = Person {
        name,
        address,
        phone_number,
    };

    warp::reply::json(&person)
}

fn generate_random_name() -> String {
    let names = vec!["Alice", "Bob", "Charlie", "David"];
    let rng = rand::thread_rng().gen_range(0..names.len());
    names[rng].to_string()
}

fn generate_random_address() -> String {
    let addresses = vec!["123 Main St", "456 Oak St", "789 Birch St", "1011 Pine St"];
    let rng = rand::thread_rng().gen_range(0..addresses.len());
    addresses[rng].to_string()
}

fn generate_random_phone_number() -> String {
    let mut rng = rand::thread_rng();
    format!(
        "+1 ({:03}) {:03}-{:04}",
        rng.gen_range(100..999),
        rng.gen_range(100..999),
        rng.gen_range(1000..9999)
    )
}

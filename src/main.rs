use zero2prod_rs::{
    configuration::get_configuration,
    startup::build,
    telemetry::{get_subscriber, init_subscriber},
};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let subscriber = get_subscriber("zero2prod_rs".into(), "debug".into(), std::io::stdout);
    init_subscriber(subscriber);
    let (server, _) = build(configuration).await?;
    server.await?;
    Ok(())
}

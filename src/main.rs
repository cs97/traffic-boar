
use actix_web::{get, web, HttpResponse};
use actix_web::HttpServer;
use actix_web::App;

use awc::Client;

use actix_proxy::{IntoHttpResponse, SendRequestError};

#[derive(serde::Deserialize)]
struct BoarState {
    url: String,
    api_url: String,
}


#[get("/")]
async fn index(data: web::Data<BoarState>) -> Result<HttpResponse, SendRequestError> {

	let url = format!("{}", &data.url);

	let client = Client::default();

	client.get(&url).send().await?.into_wrapped_http_response()
}


#[get("/{url:.*}")]
async fn proxy(data: web::Data<BoarState>, path: web::Path<(String,)>, ) -> Result<HttpResponse, SendRequestError> {

	let (url,) = path.into_inner();

	let url = format!("{}/{}", &data.url, url);

	let client = Client::default();

	client.get(&url).send().await?.into_wrapped_http_response()
}


#[get("/api/{url:.*}")]
async fn gateway(data: web::Data<BoarState>, path: web::Path<(String,)>, ) -> Result<HttpResponse, SendRequestError> {

	let (url,) = path.into_inner();

	let url = format!("{}/{}", &data.api_url, url);

	let client = Client::default();

	client.get(&url).send().await?.into_wrapped_http_response()
}



fn get_configuration() -> BoarState {
	let settings = config::Config::builder()
		.add_source(config::File::new("configuration.yaml", config::FileFormat::Yaml))
		.build().expect("REASON");
	settings.try_deserialize::<BoarState>().expect("REASON")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
	

    HttpServer::new(|| {
		let config = get_configuration();
		println!("{}", config.url);
        App::new()
			.app_data(web::Data::new(config))
			.service(index)
			.service(gateway)
			.service(proxy)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


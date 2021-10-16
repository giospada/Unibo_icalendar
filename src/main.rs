use clap::{App, Arg, ArgMatches};
use chrono::{NaiveDate};
use json::{JsonValue};
use reqwest;
use std::fs;

mod uniboevent;


fn main() {
	let arguments = create_app(); 

	let data_inizio = get_naivedate_from_arg(&arguments, "data d'inizio");
	let data_fine =  get_naivedate_from_arg(&arguments, "data di fine");

	let url = get_request_string(data_inizio, data_fine);
	let risposta_api = get_calendar(url);

	let cal = uniboevent::create_cal(risposta_api);

	let posizione = arguments.value_of("file output").unwrap();
	fs::write(posizione, cal.to_string()).expect("Non sono riuscito a scrivere sul file");
	print!("file scritto con successo su :{}\n", posizione);
}

fn create_app() -> ArgMatches<'static> {
	// definiamo gli argomenti del programma
	let args = App::new("unicalendar to icalendar")
		.author("Giovanni Spadaccini")
		.version("0.1.0")
		.about("È un programma che scarica dalle API il calendario e lo trasforma in formato icalendar")
		.arg(Arg::with_name("data d'inizio")
			.required(true)
			.index(1))
		.arg(Arg::with_name("data di fine")
			.required(true)
			.index(2))
		.arg(Arg::with_name("file output")
			.index(3)
			.required(true))
		.after_help("le date vanno inserite in formato Y-m-d")
		.get_matches();

	args
}

fn get_request_string(data_inizio: NaiveDate, data_fine: NaiveDate) -> String {

	let mut url=String::from("https://corsi.unibo.it/laurea/informatica/orario-lezioni/@@orario_reale_json?");
	url.push_str("start=");
	url.push_str(data_inizio.format("%Y-%m-%d").to_string().as_str());
	url.push_str("&end=");
	url.push_str(data_fine.format("%Y-%m-%d").to_string().as_str());

	url
}


fn get_naivedate_from_arg(args: &ArgMatches<'static>, arg_name: &str) -> NaiveDate {
	// prendiamo la data di inizio e di fine dai parametri che l'utente ha passato
	let date = NaiveDate::parse_from_str(
		args.value_of(arg_name)
		.unwrap(),"%Y-%m-%d")
		// Non so cosa sto facendo qua a complicarmi la vita con &format e .to_string
		.expect(&format!("Errore: la {} non è in formato Y-m-d", arg_name).to_string());

	date
}

#[tokio::main]
async fn get_calendar(url: String) -> JsonValue {
	
	let client = reqwest::Client::new();
	let risposta_api = client.get(url).send().await.unwrap();
	let risposta_api = json::parse(
		risposta_api.text().await
		.unwrap().as_str())
		.unwrap();

	risposta_api
}

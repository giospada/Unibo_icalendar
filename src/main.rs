use clap::{App,Arg};
use chrono::{NaiveDate, DateTime, Utc, Local, NaiveDateTime};
use reqwest;
use json;
use icalendar::{Component,CalendarDateTime};
use std::borrow::Borrow;
use std::error::Error;
use std::fs;
use std::env::temp_dir;

#[tokio::main]
async fn main() {
    // definiamo gli argomenti del programma
    let arguments=App::new("unicalendar to icalendar")
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
    // prendiamo la data di inizio e di fine dai parametri che l'utente ha passato

    let data_inizio= NaiveDate::parse_from_str(arguments.value_of("data d'inizio").unwrap(),"%Y-%m-%d")
        .expect("la data d'inizio è in un formato sbagliato");
    let data_fine=  NaiveDate::parse_from_str(arguments.value_of("data di fine").unwrap(), "%Y-%m-%d")
        .expect("la data di fine è in un formato sbagliato");

    let mut url=String::from("https://corsi.unibo.it/laurea/informatica/orario-lezioni/@@orario_reale_json?");
    url.push_str("start=");
    url.push_str(data_inizio.format("%Y-%m-%d").to_string().as_str());
    url.push_str("&end=");
    url.push_str(data_fine.format("%Y-%m-%d").to_string().as_str());
    let client = reqwest::Client::new();
    let risposta_api = client.get(url).send().await.unwrap();
    let risposta_api=json::parse(risposta_api.text().await.unwrap().as_str()).unwrap();
    let mut iter=0;
    let mut cal=icalendar::Calendar::new();
    cal.name("Università").description("calendario delle lezioni dell'università");
    while risposta_api.len()>iter{
        let current=risposta_api[iter].borrow();

        let mut inizio=current["start"].to_string();
        inizio.push_str(".000000000+02:00");
        let mut fine=current["end"].to_string();
        fine.push_str(".000000000+02:00");
        let inizio=DateTime::parse_from_rfc3339(inizio.as_str()).unwrap().with_timezone(&Utc);
        let fine=DateTime::parse_from_rfc3339(fine.as_str()).unwrap().with_timezone(&Utc);

        let location=current["aule"][0].borrow();

        let mut description=String::new();
        description.push_str("Docente: ");
        description.push_str(current["docente"].to_string().as_str());
        description.push_str(" , Link: ");
        description.push_str(current["teams"].to_string().as_str());

        let mut location_str=String::new();
        location_str.push_str("Indirizzo: ");
        location_str.push_str(location["des_indirizzo"].to_string().as_str());
        location_str.push_str(" , Edificio:  ");
        location_str.push_str(location["des_edificio"].to_string().as_str());
        location_str.push_str(", Aula: ");
        location_str.push_str(location["des_aula"].to_string().as_str());
        let evento=icalendar::Event::new()
            .summary(current["title"].to_string().as_str())
            .description(description.as_str())
            .location(location_str.as_str())
            .starts(CalendarDateTime::Utc(inizio))
            .ends(CalendarDateTime::Utc(fine))
            .done();

        cal.push(evento);
        iter+=1;
    }
    let posizione=arguments.value_of("file output").unwrap();
    fs::write(posizione,cal.to_string());
    print!("file scritto con successo su :{}\n", posizione);
}

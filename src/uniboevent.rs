use icalendar::{CalendarDateTime, Component, Calendar};
use chrono::{DateTime, Utc};
use json::{JsonValue};
use std::borrow::Borrow;

pub fn create_cal(response: JsonValue) -> Calendar {
	let mut cal = icalendar::Calendar::new();
	cal.name("Università").description("calendario delle lezioni dell'università");

	for i in 1..response.len() {
		let current = response[i].borrow();

		let current_event = UniboEvent::construct(&current);

		let evento = icalendar::Event::new()
			.summary(current_event.title.as_str())
			.description(current_event.description.as_str())
			.location(current_event.location.as_str())
			.starts(current_event.date.0)
			.ends(current_event.date.1)
			.done();

		cal.push(evento);
	}

	cal
}

struct UniboEvent {
	title: String,
	description: String,
	location: String,
	date: (CalendarDateTime, CalendarDateTime)
}

impl UniboEvent {
	fn construct(response: &JsonValue) -> UniboEvent {
		UniboEvent{
			title: response["title"].to_string(),
			description: UniboEvent::parse_description(response),
			location: UniboEvent::parse_location(response),
			date: UniboEvent::parse_date(response)
		}
	}

	fn parse_description(response: &JsonValue) -> String {
		let mut description = String::new();
		description.push_str("Docente: ");
		description.push_str(response["docente"].to_string().as_str());
		description.push_str(" , Link: ");
		description.push_str(response["teams"].to_string().as_str());

		description
	}

	fn parse_location(response: &JsonValue) -> String {
		let location = response["aule"][0].borrow();
		let mut location_str=String::new();
		location_str.push_str("Indirizzo: ");
		location_str.push_str(location["des_indirizzo"].to_string().as_str());
		location_str.push_str(" , Edificio:  ");
		location_str.push_str(location["des_edificio"].to_string().as_str());
		location_str.push_str(", Aula: ");
		location_str.push_str(location["des_aula"].to_string().as_str());

		location_str
	}

	fn parse_date(response: &JsonValue) -> (CalendarDateTime, CalendarDateTime) {
		let mut inizio = response["start"].to_string();
		inizio.push_str(".000000000+02:00");
		let mut fine = response["end"].to_string();
		fine.push_str(".000000000+02:00"); 

		let inizio = DateTime::parse_from_rfc3339(inizio.as_str()).unwrap().with_timezone(&Utc);
		let fine = DateTime::parse_from_rfc3339(fine.as_str()).unwrap().with_timezone(&Utc);
		
		(CalendarDateTime::Utc(inizio), CalendarDateTime::Utc(fine))
	}
}
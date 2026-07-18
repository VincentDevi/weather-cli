use chrono::NaiveDate;
use comfy_table::{ContentArrangement, Table, presets::UTF8_FULL};
use termimad::{
    MadSkin,
    crossterm::style::{Attribute, Color},
};

#[derive(Debug, PartialEq)]
pub struct WeatherTableRow {
    city: String,
    date: Option<NaiveDate>,
    temperature_celsius: Option<f64>,
    humidity_percent: Option<i64>,
    precipitation_probability: Option<f64>,
}

impl WeatherTableRow {
    pub fn new(
        city: impl Into<String>,
        date: Option<NaiveDate>,
        temperature_celsius: Option<f64>,
        humidity_percent: Option<i64>,
        precipitation_probability: Option<f64>,
    ) -> Self {
        Self {
            city: city.into(),
            date,
            temperature_celsius,
            humidity_percent,
            precipitation_probability,
        }
    }
}

pub fn render_weather_table(rows: &[WeatherTableRow]) -> String {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(["City", "Date", "Temperature", "Humidity", "Precipitation"]);

    for row in rows {
        table.add_row([
            row.city.clone(),
            format_date(row.date),
            format_temperature(row.temperature_celsius),
            format_percentage(row.humidity_percent.map(|value| value as f64)),
            format_percentage(
                row.precipitation_probability
                    .map(|probability| probability * 100.0),
            ),
        ]);
    }

    table.to_string()
}

pub fn render_markdown(markdown: &str, styled: bool) -> String {
    let normalized = markdown.replace("\r\n", "\n");
    let normalized = normalized.trim();

    if normalized.is_empty() {
        return String::new();
    }

    if styled {
        markdown_skin().term_text(normalized).to_string()
    } else {
        normalized.to_owned()
    }
}

fn markdown_skin() -> MadSkin {
    let mut skin = MadSkin::default();

    for header in &mut skin.headers {
        header.set_fg(Color::Cyan);
        header.add_attr(Attribute::Bold);
    }
    skin.bullet.set_fg(Color::Yellow);
    skin.inline_code.set_fg(Color::DarkGrey);

    skin
}

fn format_date(value: Option<NaiveDate>) -> String {
    value
        .map(|date| date.format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| "N/A".to_owned())
}

fn format_temperature(value: Option<f64>) -> String {
    value
        .map(|temperature| format!("{temperature:.1} °C"))
        .unwrap_or_else(|| "N/A".to_owned())
}

fn format_percentage(value: Option<f64>) -> String {
    value
        .map(|percentage| format!("{percentage:.0}%"))
        .unwrap_or_else(|| "N/A".to_owned())
}

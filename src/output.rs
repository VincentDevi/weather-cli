use comfy_table::{ContentArrangement, Table, presets::UTF8_FULL};

#[derive(Debug, PartialEq)]
pub struct WeatherTableRow {
    city: String,
    temperature_celsius: Option<f64>,
    humidity_percent: Option<i64>,
    precipitation_probability: Option<f64>,
}

impl WeatherTableRow {
    pub fn new(
        city: impl Into<String>,
        temperature_celsius: Option<f64>,
        humidity_percent: Option<i64>,
        precipitation_probability: Option<f64>,
    ) -> Self {
        Self {
            city: city.into(),
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
        .set_header(["City", "Temperature", "Humidity", "Precipitation"]);

    for row in rows {
        table.add_row([
            row.city.clone(),
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

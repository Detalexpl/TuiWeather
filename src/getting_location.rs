use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}
#[derive(Deserialize, Debug)]

struct Record {
    city: String,
    lat: f64,
    lng: f64,
    country: String,
    iso3: String,
    admin_name: String,
    capital: String,
}
pub fn get_location(path: &PathBuf, query: &str, ) -> Result<Option<Location>, Box<dyn std::error::Error>> {
    let mut rdr = csv::ReaderBuilder::new().from_path(path)?;
    for result in rdr.deserialize() {
        let record: Record = result?;
        if record.city == query {
            return Ok(Some(Location {
                latitude: record.lat,
                longitude: record.lng,
            }));
        }
    }

    Ok(None)
}
#[cfg(test)]
mod tests {
    use crate::getting_location::get_location;

    #[test]
    fn test_getting_location() {
        use std::path::PathBuf;
        let query = "Warsaw";
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("test_resources/cities.csv");
        let location = get_location(&path, query).expect("ty był error").expect("tu był none");
        assert_eq!(location.latitude, 52.2300);
        assert_eq!(location.longitude, 21.0111);
    }
}
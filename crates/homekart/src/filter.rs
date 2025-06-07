use crate::models::property::Property;

pub trait PropertyFilter {
    fn filter_properties(&self, properties: Vec<Property>) -> Vec<Property>;
}

pub struct DefaultPropertyFilter;

impl PropertyFilter for DefaultPropertyFilter {
    fn filter_properties(&self, properties: Vec<Property>) -> Vec<Property> {
        properties
    }
}

pub struct PriceBasedFilter;

impl PropertyFilter for PriceBasedFilter {
    fn filter_properties(&self, properties: Vec<Property>) -> Vec<Property> {
        let mut properties = properties;
        properties.sort_by(|a, b| a.get_price().partial_cmp(&b.get_price()).unwrap());
        properties
    }
}

pub struct RommBasedFilter;

impl PropertyFilter for RommBasedFilter {
    fn filter_properties(&self, properties: Vec<Property>) -> Vec<Property> {
        let mut properties = properties;
        properties.sort_by(|a, b| a.get_rooms().partial_cmp(&b.get_rooms()).unwrap());
        properties
    }
}

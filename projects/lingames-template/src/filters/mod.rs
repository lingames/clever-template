use convert_case::{Case, Casing};

pub fn camel_case<T: std::fmt::Display>(s: T) -> askama::Result<String> {
    let s = s.to_string();
    Ok(s.to_case(Case::Camel))
}

pub fn pascal_case<T: std::fmt::Display>(s: T) -> askama::Result<String> {
    let s = s.to_string();
    Ok(s.to_case(Case::Pascal))
}

pub fn upper_case<T: std::fmt::Display>(s: T) -> askama::Result<String> {
    let s = s.to_string();
    Ok(s.to_case(Case::Constant))
}

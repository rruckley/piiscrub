///! Document types
/// 
/// 
/// 
/// 
use regex::Regex;
use strum_macros::EnumIter;

#[derive(Debug)]

#[derive(EnumIter, PartialEq)]
pub enum DocType {
    CurrentPassport,
    ExpiredPassport,
    BirthCertificate,
    CitizenCertificate,
    DriverLicense,
    ForeignPassport,
    Medicare,
    Iccid,
    MobileServiceNumber,
}

impl std::fmt::Display for DocType {
    fn fmt(&self, f: & mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::CurrentPassport => write!(f,"Passport"),
            Self::ExpiredPassport => write!(f,"Expired Passport"),
            Self::BirthCertificate => write!(f,"Birth Certificate"),
            Self::CitizenCertificate => write!(f,"Citizen Certificate"),
            Self::DriverLicense => write!(f,"Drivers License"),
            Self::ForeignPassport => write!(f,"Foreign Passport"),
            Self::Medicare => write!(f,"Medicare Cart"),
            Self::Iccid=> write!(f,"SIM Card"),
            Self::MobileServiceNumber => write!(f,"MSN"),

        }
    }
}

pub struct Document {
    pub doc_type : DocType,
    pub points : u32 ,
    pub pattern : regex::Regex,
}

impl Document {
    pub fn new(doc_type : DocType, points : u32) -> Self {
        let pattern = Document::get_regex(&doc_type);
        Self {
            doc_type,
            points,
            pattern,
        }
    }
    pub fn get_regex_pattern(doc_type: &DocType) -> String {
        match doc_type {
            DocType::CurrentPassport => "[A-Z][A-Z]?\\d{7}".to_owned(),
            DocType::ExpiredPassport => "EPASS".to_owned(),
            DocType::BirthCertificate => "\\d+/\\d{4}".to_owned(),
            DocType::CitizenCertificate => "CCERT".to_owned(),
            DocType::DriverLicense => "DRIVER".to_owned(),
            DocType::ForeignPassport  => "FPASS".to_owned(),
            DocType::Medicare => "\\d{4}-\\d{5}-\\d".to_owned(),
            DocType::Iccid => "\\d{13}".to_owned(),
            DocType::MobileServiceNumber => "04\\d{8}".to_owned(),
        }
    }
    fn get_regex(doc_type: &DocType) -> regex::Regex {
        Regex::new(Document::get_regex_pattern(doc_type).as_str()).unwrap()
    }
}
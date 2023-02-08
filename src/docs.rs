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
    DriverLicenseNSW,
    DriverLicenseNSWCard,
    ForeignPassport,
    Medicare,
    Iccid,
    MobileServiceNumber,
    MarriageNSW,
}

impl std::fmt::Display for DocType {
    fn fmt(&self, f: & mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::CurrentPassport => write!(f,"Passport"),
            Self::ExpiredPassport => write!(f,"Expired Passport"),
            Self::BirthCertificate => write!(f,"Birth Certificate"),
            Self::CitizenCertificate => write!(f,"Citizen Certificate"),
            Self::DriverLicenseNSW => write!(f,"Drivers License NSW"),
            Self::DriverLicenseNSWCard => write!(f,"Drivers License NSW (Card)"),
            Self::ForeignPassport => write!(f,"Foreign Passport"),
            Self::Medicare => write!(f,"Medicare Card"),
            Self::Iccid=> write!(f,"SIM Card"),
            Self::MobileServiceNumber => write!(f,"MSN"),
            Self::MarriageNSW => write!(f,"Marriage NSW"),

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

    /// Lots of useful info here: https://www.ato.gov.au/misc/downloads/pdf/qc48092.pdf 
    pub fn get_regex_pattern(doc_type: &DocType) -> String {
        match doc_type {
            DocType::CurrentPassport => "[A-Z][A-Z]?\\d{7}".to_owned(),
            DocType::ExpiredPassport => "[A-Z][A-Z]?\\d{7}".to_owned(),
            DocType::BirthCertificate => "\\d+/\\d{4}".to_owned(),
            DocType::CitizenCertificate => "0\\d{10}|ACC \\d{6}|CAS \\d{4}".to_owned(),
            DocType::DriverLicenseNSW => "[0-9A-Z]{6}".to_owned(),
            DocType::DriverLicenseNSWCard => "\\d \\d{3} \\d{3} \\d{3}|\\d{10}".to_owned(),
            DocType::ForeignPassport  => "FPASS".to_owned(),
            DocType::Medicare => "\\d{4}-\\d{5}-\\d".to_owned(),
            DocType::Iccid => "8961\\d+".to_owned(),
            DocType::MobileServiceNumber => "(04\\d{8}|04\\d{2} \\d{3} \\d{3})".to_owned(),
            DocType::MarriageNSW => "\\d+/\\d{4}".to_owned(),
        }
    }
    fn get_regex(doc_type: &DocType) -> regex::Regex {
        Regex::new(Document::get_regex_pattern(doc_type).as_str()).unwrap()
    }
}
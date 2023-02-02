
use crate::docs::{Document,DocType};
use regex::*;
use strum::IntoEnumIterator;

pub struct RegexFilter {
    docs : Vec<Document>,
}

struct RegexMatch<'a> {
    doc_type : String,
    matched : Match<'a>,
}

impl RegexFilter {
    pub fn new() -> Self {
        let cpass = Document::new(DocType::CurrentPassport,70);
        let epass = Document::new(DocType::ExpiredPassport,50);
        let fpass = Document::new(DocType::ForeignPassport,50);
        let bcert = Document::new(DocType::BirthCertificate,70);
        let ccert = Document::new(DocType::CitizenCertificate, 50);
        let driver_nsw = Document::new(DocType::DriverLicenseNSW, 50);
        let driver_nsw_card = Document::new(DocType::DriverLicenseNSWCard, 50);
        let medicare = Document::new(DocType::Medicare, 30);
        let iccid = Document::new(DocType::Iccid, 0);
        let msn = Document::new(DocType::MobileServiceNumber, 10);
        let docs = vec![cpass,epass,fpass,bcert,ccert,driver_nsw,medicare,iccid,msn,driver_nsw_card];
        Self {
            docs,
        }
    }

    /// Iterate through context looking for all document identifiers
    pub fn filter(&self, context : String) -> Result<String,String> {
        let mut output = context.clone();
        let mut all_matches = vec![];
        for doc in self.docs.iter() {
            // Look for regex matches
            let matches = doc.pattern.find_iter(context.as_str());
            matches.for_each(|m| {
                all_matches.push(RegexMatch {
                    doc_type : doc.doc_type.to_string(),
                    matched : m,
                });
            })
          
        }
        // Need to iterate in reverse so that we don't invalidate the offsets
        for m in all_matches.iter().rev() {
            //output.push_str(format!("{} : From {} to {}",doc.doc_type,m.start(),m.end()).as_str());
            let start = m.matched.start();
            let end = m.matched.end();
            let fill = "X".repeat(end-start);
            let _span = format!("<span class=\"{}\">{}</span>",m.doc_type,fill);
            output.replace_range(start..end, &fill)
        }
        
        Ok(format!("<div class=\"regex\">{}</div>",output))
    }

    /// Output current document configuration
    pub fn config(&self) -> Result<String,String> {
        let mut output = "<ul>".to_owned();
        // Iterate through docs
        for doc in DocType::iter() {
            output.push_str(format!("<li>{} - {}</li>",doc.to_string(),Document::get_regex_pattern(&doc)).as_str());
        }
        output.push_str("</ul>");
        Ok(format!("<div class=\"regex-config\">{}</div>",output))
    }
}
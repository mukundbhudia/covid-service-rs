use crate::schema::CountyStatistic;
use std::collections::HashMap;

pub fn alpha_codes() -> HashMap<String, CountyStatistic> {
    let mut alpha3 = HashMap::new();
    alpha3.insert(
        "Afghanistan".to_string(),
        CountyStatistic {
            iso_code: "AFG".to_string(),
        },
    );
    alpha3.insert(
        "Albania".to_string(),
        CountyStatistic {
            iso_code: "ALB".to_string(),
        },
    );
    alpha3.insert(
        "Algeria".to_string(),
        CountyStatistic {
            iso_code: "DZA".to_string(),
        },
    );
    alpha3.insert(
        "American Samoa".to_string(),
        CountyStatistic {
            iso_code: "ASM".to_string(),
        },
    );
    alpha3.insert(
        "Andorra".to_string(),
        CountyStatistic {
            iso_code: "AND".to_string(),
        },
    );
    alpha3.insert(
        "Angola".to_string(),
        CountyStatistic {
            iso_code: "AGO".to_string(),
        },
    );
    alpha3.insert(
        "Anguilla".to_string(),
        CountyStatistic {
            iso_code: "AIA".to_string(),
        },
    );
    alpha3.insert(
        "Antarctica".to_string(),
        CountyStatistic {
            iso_code: "ATA".to_string(),
        },
    );
    alpha3.insert(
        "Antigua and Barbuda".to_string(),
        CountyStatistic {
            iso_code: "ATG".to_string(),
        },
    );
    alpha3.insert(
        "Argentina".to_string(),
        CountyStatistic {
            iso_code: "ARG".to_string(),
        },
    );
    alpha3.insert(
        "Armenia".to_string(),
        CountyStatistic {
            iso_code: "ARM".to_string(),
        },
    );
    alpha3.insert(
        "Aruba".to_string(),
        CountyStatistic {
            iso_code: "ABW".to_string(),
        },
    );
    alpha3.insert(
        "Australia".to_string(),
        CountyStatistic {
            iso_code: "AUS".to_string(),
        },
    );
    alpha3.insert(
        "Austria".to_string(),
        CountyStatistic {
            iso_code: "AUT".to_string(),
        },
    );
    alpha3.insert(
        "Azerbaijan".to_string(),
        CountyStatistic {
            iso_code: "AZE".to_string(),
        },
    );
    alpha3.insert(
        "Bahamas".to_string(),
        CountyStatistic {
            iso_code: "BHS".to_string(),
        },
    );
    alpha3.insert(
        "Bahrain".to_string(),
        CountyStatistic {
            iso_code: "BHR".to_string(),
        },
    );
    alpha3.insert(
        "Bangladesh".to_string(),
        CountyStatistic {
            iso_code: "BGD".to_string(),
        },
    );
    alpha3.insert(
        "Barbados".to_string(),
        CountyStatistic {
            iso_code: "BRB".to_string(),
        },
    );
    alpha3.insert(
        "Belarus".to_string(),
        CountyStatistic {
            iso_code: "BLR".to_string(),
        },
    );
    alpha3.insert(
        "Belgium".to_string(),
        CountyStatistic {
            iso_code: "BEL".to_string(),
        },
    );
    alpha3.insert(
        "Belize".to_string(),
        CountyStatistic {
            iso_code: "BLZ".to_string(),
        },
    );
    alpha3.insert(
        "Benin".to_string(),
        CountyStatistic {
            iso_code: "BEN".to_string(),
        },
    );
    alpha3.insert(
        "Bermuda".to_string(),
        CountyStatistic {
            iso_code: "BMU".to_string(),
        },
    );
    alpha3.insert(
        "Bhutan".to_string(),
        CountyStatistic {
            iso_code: "BTN".to_string(),
        },
    );
    alpha3.insert(
        "Bolivia".to_string(),
        CountyStatistic {
            iso_code: "BOL".to_string(),
        },
    );
    alpha3.insert(
        "Bonaire, Sint Eustatius and Saba".to_string(),
        CountyStatistic {
            iso_code: "BES".to_string(),
        },
    );
    alpha3.insert(
        "Bosnia and Herzegovina".to_string(),
        CountyStatistic {
            iso_code: "BIH".to_string(),
        },
    );
    alpha3.insert(
        "Botswana".to_string(),
        CountyStatistic {
            iso_code: "BWA".to_string(),
        },
    );
    alpha3.insert(
        "Bouvet Island".to_string(),
        CountyStatistic {
            iso_code: "BVT".to_string(),
        },
    );
    alpha3.insert(
        "Brazil".to_string(),
        CountyStatistic {
            iso_code: "BRA".to_string(),
        },
    );
    alpha3.insert(
        "British Indian Ocean Territory".to_string(),
        CountyStatistic {
            iso_code: "IOT".to_string(),
        },
    );
    alpha3.insert(
        "Brunei".to_string(),
        CountyStatistic {
            iso_code: "BRN".to_string(),
        },
    );
    alpha3.insert(
        "Bulgaria".to_string(),
        CountyStatistic {
            iso_code: "BGR".to_string(),
        },
    );
    alpha3.insert(
        "Burkina Faso".to_string(),
        CountyStatistic {
            iso_code: "BFA".to_string(),
        },
    );
    alpha3.insert(
        "Burundi".to_string(),
        CountyStatistic {
            iso_code: "BDI".to_string(),
        },
    );
    alpha3.insert(
        "Cabo Verde".to_string(),
        CountyStatistic {
            iso_code: "CPV".to_string(),
        },
    );
    alpha3.insert(
        "Cambodia".to_string(),
        CountyStatistic {
            iso_code: "KHM".to_string(),
        },
    );
    alpha3.insert(
        "Cameroon".to_string(),
        CountyStatistic {
            iso_code: "CMR".to_string(),
        },
    );
    alpha3.insert(
        "Canada".to_string(),
        CountyStatistic {
            iso_code: "CAN".to_string(),
        },
    );
    alpha3.insert(
        "Cayman Islands".to_string(),
        CountyStatistic {
            iso_code: "CYM".to_string(),
        },
    );
    alpha3.insert(
        "Central African Republic".to_string(),
        CountyStatistic {
            iso_code: "CAF".to_string(),
        },
    );
    alpha3.insert(
        "Chad".to_string(),
        CountyStatistic {
            iso_code: "TCD".to_string(),
        },
    );
    alpha3.insert(
        "Chile".to_string(),
        CountyStatistic {
            iso_code: "CHL".to_string(),
        },
    );
    alpha3.insert(
        "China".to_string(),
        CountyStatistic {
            iso_code: "CHN".to_string(),
        },
    );
    alpha3.insert(
        "Christmas Island".to_string(),
        CountyStatistic {
            iso_code: "CXR".to_string(),
        },
    );
    alpha3.insert(
        "Cocos (Keeling) Islands".to_string(),
        CountyStatistic {
            iso_code: "CCK".to_string(),
        },
    );
    alpha3.insert(
        "Colombia".to_string(),
        CountyStatistic {
            iso_code: "COL".to_string(),
        },
    );
    alpha3.insert(
        "Comoros".to_string(),
        CountyStatistic {
            iso_code: "COM".to_string(),
        },
    );
    alpha3.insert(
        "Democratic Republic of Congo".to_string(),
        CountyStatistic {
            iso_code: "COD".to_string(),
        },
    );
    alpha3.insert(
        "Congo".to_string(),
        CountyStatistic {
            iso_code: "COG".to_string(),
        },
    );
    alpha3.insert(
        "Cook Islands".to_string(),
        CountyStatistic {
            iso_code: "COK".to_string(),
        },
    );
    alpha3.insert(
        "Costa Rica".to_string(),
        CountyStatistic {
            iso_code: "CRI".to_string(),
        },
    );
    alpha3.insert(
        "Croatia".to_string(),
        CountyStatistic {
            iso_code: "HRV".to_string(),
        },
    );
    alpha3.insert(
        "Cuba".to_string(),
        CountyStatistic {
            iso_code: "CUB".to_string(),
        },
    );
    alpha3.insert(
        "Curaçao".to_string(),
        CountyStatistic {
            iso_code: "CUW".to_string(),
        },
    );
    alpha3.insert(
        "Cyprus".to_string(),
        CountyStatistic {
            iso_code: "CYP".to_string(),
        },
    );
    alpha3.insert(
        "Czechia".to_string(),
        CountyStatistic {
            iso_code: "CZE".to_string(),
        },
    );
    alpha3.insert(
        "Cote d'Ivoire".to_string(),
        CountyStatistic {
            iso_code: "CIV".to_string(),
        },
    );
    alpha3.insert(
        "Denmark".to_string(),
        CountyStatistic {
            iso_code: "DNK".to_string(),
        },
    );
    alpha3.insert(
        "Djibouti".to_string(),
        CountyStatistic {
            iso_code: "DJI".to_string(),
        },
    );
    alpha3.insert(
        "Dominica".to_string(),
        CountyStatistic {
            iso_code: "DMA".to_string(),
        },
    );
    alpha3.insert(
        "Dominican Republic".to_string(),
        CountyStatistic {
            iso_code: "DOM".to_string(),
        },
    );
    alpha3.insert(
        "Ecuador".to_string(),
        CountyStatistic {
            iso_code: "ECU".to_string(),
        },
    );
    alpha3.insert(
        "Egypt".to_string(),
        CountyStatistic {
            iso_code: "EGY".to_string(),
        },
    );
    alpha3.insert(
        "El Salvador".to_string(),
        CountyStatistic {
            iso_code: "SLV".to_string(),
        },
    );
    alpha3.insert(
        "Equatorial Guinea".to_string(),
        CountyStatistic {
            iso_code: "GNQ".to_string(),
        },
    );
    alpha3.insert(
        "Eritrea".to_string(),
        CountyStatistic {
            iso_code: "ERI".to_string(),
        },
    );
    alpha3.insert(
        "Estonia".to_string(),
        CountyStatistic {
            iso_code: "EST".to_string(),
        },
    );
    alpha3.insert(
        "Eswatini".to_string(),
        CountyStatistic {
            iso_code: "SWZ".to_string(),
        },
    );
    alpha3.insert(
        "Ethiopia".to_string(),
        CountyStatistic {
            iso_code: "ETH".to_string(),
        },
    );
    alpha3.insert(
        "Falkland Islands [Malvinas]".to_string(),
        CountyStatistic {
            iso_code: "FLK".to_string(),
        },
    );
    alpha3.insert(
        "Faroe Islands".to_string(),
        CountyStatistic {
            iso_code: "FRO".to_string(),
        },
    );
    alpha3.insert(
        "Fiji".to_string(),
        CountyStatistic {
            iso_code: "FJI".to_string(),
        },
    );
    alpha3.insert(
        "Finland".to_string(),
        CountyStatistic {
            iso_code: "FIN".to_string(),
        },
    );
    alpha3.insert(
        "France".to_string(),
        CountyStatistic {
            iso_code: "FRA".to_string(),
        },
    );
    alpha3.insert(
        "French Guiana".to_string(),
        CountyStatistic {
            iso_code: "GUF".to_string(),
        },
    );
    alpha3.insert(
        "French Polynesia".to_string(),
        CountyStatistic {
            iso_code: "PYF".to_string(),
        },
    );
    alpha3.insert(
        "French Southern Territories".to_string(),
        CountyStatistic {
            iso_code: "ATF".to_string(),
        },
    );
    alpha3.insert(
        "Gabon".to_string(),
        CountyStatistic {
            iso_code: "GAB".to_string(),
        },
    );
    alpha3.insert(
        "Gambia".to_string(),
        CountyStatistic {
            iso_code: "GMB".to_string(),
        },
    );
    alpha3.insert(
        "Georgia".to_string(),
        CountyStatistic {
            iso_code: "GEO".to_string(),
        },
    );
    alpha3.insert(
        "Germany".to_string(),
        CountyStatistic {
            iso_code: "DEU".to_string(),
        },
    );
    alpha3.insert(
        "Ghana".to_string(),
        CountyStatistic {
            iso_code: "GHA".to_string(),
        },
    );
    alpha3.insert(
        "Gibraltar".to_string(),
        CountyStatistic {
            iso_code: "GIB".to_string(),
        },
    );
    alpha3.insert(
        "Greece".to_string(),
        CountyStatistic {
            iso_code: "GRC".to_string(),
        },
    );
    alpha3.insert(
        "Greenland".to_string(),
        CountyStatistic {
            iso_code: "GRL".to_string(),
        },
    );
    alpha3.insert(
        "Grenada".to_string(),
        CountyStatistic {
            iso_code: "GRD".to_string(),
        },
    );
    alpha3.insert(
        "Guadeloupe".to_string(),
        CountyStatistic {
            iso_code: "GLP".to_string(),
        },
    );
    alpha3.insert(
        "Guam".to_string(),
        CountyStatistic {
            iso_code: "GUM".to_string(),
        },
    );
    alpha3.insert(
        "Guatemala".to_string(),
        CountyStatistic {
            iso_code: "GTM".to_string(),
        },
    );
    alpha3.insert(
        "Guernsey".to_string(),
        CountyStatistic {
            iso_code: "GGY".to_string(),
        },
    );
    alpha3.insert(
        "Guinea".to_string(),
        CountyStatistic {
            iso_code: "GIN".to_string(),
        },
    );
    alpha3.insert(
        "Guinea-Bissau".to_string(),
        CountyStatistic {
            iso_code: "GNB".to_string(),
        },
    );
    alpha3.insert(
        "Guyana".to_string(),
        CountyStatistic {
            iso_code: "GUY".to_string(),
        },
    );
    alpha3.insert(
        "Haiti".to_string(),
        CountyStatistic {
            iso_code: "HTI".to_string(),
        },
    );
    alpha3.insert(
        "Heard Island and McDonald Islands".to_string(),
        CountyStatistic {
            iso_code: "HMD".to_string(),
        },
    );
    alpha3.insert(
        "Holy See".to_string(),
        CountyStatistic {
            iso_code: "VAT".to_string(),
        },
    );
    alpha3.insert(
        "Honduras".to_string(),
        CountyStatistic {
            iso_code: "HND".to_string(),
        },
    );
    alpha3.insert(
        "Hong Kong".to_string(),
        CountyStatistic {
            iso_code: "HKG".to_string(),
        },
    );
    alpha3.insert(
        "Hungary".to_string(),
        CountyStatistic {
            iso_code: "HUN".to_string(),
        },
    );
    alpha3.insert(
        "Iceland".to_string(),
        CountyStatistic {
            iso_code: "ISL".to_string(),
        },
    );
    alpha3.insert(
        "India".to_string(),
        CountyStatistic {
            iso_code: "IND".to_string(),
        },
    );
    alpha3.insert(
        "Indonesia".to_string(),
        CountyStatistic {
            iso_code: "IDN".to_string(),
        },
    );
    alpha3.insert(
        "Iran".to_string(),
        CountyStatistic {
            iso_code: "IRN".to_string(),
        },
    );
    alpha3.insert(
        "Iraq".to_string(),
        CountyStatistic {
            iso_code: "IRQ".to_string(),
        },
    );
    alpha3.insert(
        "Ireland".to_string(),
        CountyStatistic {
            iso_code: "IRL".to_string(),
        },
    );
    alpha3.insert(
        "Isle of Man".to_string(),
        CountyStatistic {
            iso_code: "IMN".to_string(),
        },
    );
    alpha3.insert(
        "Israel".to_string(),
        CountyStatistic {
            iso_code: "ISR".to_string(),
        },
    );
    alpha3.insert(
        "Italy".to_string(),
        CountyStatistic {
            iso_code: "ITA".to_string(),
        },
    );
    alpha3.insert(
        "Jamaica".to_string(),
        CountyStatistic {
            iso_code: "JAM".to_string(),
        },
    );
    alpha3.insert(
        "Japan".to_string(),
        CountyStatistic {
            iso_code: "JPN".to_string(),
        },
    );
    alpha3.insert(
        "Jersey".to_string(),
        CountyStatistic {
            iso_code: "JEY".to_string(),
        },
    );
    alpha3.insert(
        "Jordan".to_string(),
        CountyStatistic {
            iso_code: "JOR".to_string(),
        },
    );
    alpha3.insert(
        "Kazakhstan".to_string(),
        CountyStatistic {
            iso_code: "KAZ".to_string(),
        },
    );
    alpha3.insert(
        "Kenya".to_string(),
        CountyStatistic {
            iso_code: "KEN".to_string(),
        },
    );
    alpha3.insert(
        "Kiribati".to_string(),
        CountyStatistic {
            iso_code: "KIR".to_string(),
        },
    );
    alpha3.insert(
        "Korea (the Democratic People's Republic of)".to_string(),
        CountyStatistic {
            iso_code: "PRK".to_string(),
        },
    );
    alpha3.insert(
        "South Korea".to_string(),
        CountyStatistic {
            iso_code: "KOR".to_string(),
        },
    );
    alpha3.insert(
        "Kosovo".to_string(),
        CountyStatistic {
            iso_code: "KOS".to_string(),
        },
    );
    alpha3.insert(
        "Kuwait".to_string(),
        CountyStatistic {
            iso_code: "KWT".to_string(),
        },
    );
    alpha3.insert(
        "Kyrgyzstan".to_string(),
        CountyStatistic {
            iso_code: "KGZ".to_string(),
        },
    );
    alpha3.insert(
        "Laos".to_string(),
        CountyStatistic {
            iso_code: "LAO".to_string(),
        },
    );
    alpha3.insert(
        "Latvia".to_string(),
        CountyStatistic {
            iso_code: "LVA".to_string(),
        },
    );
    alpha3.insert(
        "Lebanon".to_string(),
        CountyStatistic {
            iso_code: "LBN".to_string(),
        },
    );
    alpha3.insert(
        "Lesotho".to_string(),
        CountyStatistic {
            iso_code: "LSO".to_string(),
        },
    );
    alpha3.insert(
        "Liberia".to_string(),
        CountyStatistic {
            iso_code: "LBR".to_string(),
        },
    );
    alpha3.insert(
        "Libya".to_string(),
        CountyStatistic {
            iso_code: "LBY".to_string(),
        },
    );
    alpha3.insert(
        "Liechtenstein".to_string(),
        CountyStatistic {
            iso_code: "LIE".to_string(),
        },
    );
    alpha3.insert(
        "Lithuania".to_string(),
        CountyStatistic {
            iso_code: "LTU".to_string(),
        },
    );
    alpha3.insert(
        "Luxembourg".to_string(),
        CountyStatistic {
            iso_code: "LUX".to_string(),
        },
    );
    alpha3.insert(
        "Macao".to_string(),
        CountyStatistic {
            iso_code: "MAC".to_string(),
        },
    );
    alpha3.insert(
        "Madagascar".to_string(),
        CountyStatistic {
            iso_code: "MDG".to_string(),
        },
    );
    alpha3.insert(
        "Malawi".to_string(),
        CountyStatistic {
            iso_code: "MWI".to_string(),
        },
    );
    alpha3.insert(
        "Malaysia".to_string(),
        CountyStatistic {
            iso_code: "MYS".to_string(),
        },
    );
    alpha3.insert(
        "Maldives".to_string(),
        CountyStatistic {
            iso_code: "MDV".to_string(),
        },
    );
    alpha3.insert(
        "Mali".to_string(),
        CountyStatistic {
            iso_code: "MLI".to_string(),
        },
    );
    alpha3.insert(
        "Malta".to_string(),
        CountyStatistic {
            iso_code: "MLT".to_string(),
        },
    );
    alpha3.insert(
        "Marshall Islands".to_string(),
        CountyStatistic {
            iso_code: "MHL".to_string(),
        },
    );
    alpha3.insert(
        "Martinique".to_string(),
        CountyStatistic {
            iso_code: "MTQ".to_string(),
        },
    );
    alpha3.insert(
        "Mauritania".to_string(),
        CountyStatistic {
            iso_code: "MRT".to_string(),
        },
    );
    alpha3.insert(
        "Mauritius".to_string(),
        CountyStatistic {
            iso_code: "MUS".to_string(),
        },
    );
    alpha3.insert(
        "Mayotte".to_string(),
        CountyStatistic {
            iso_code: "MYT".to_string(),
        },
    );
    alpha3.insert(
        "Mexico".to_string(),
        CountyStatistic {
            iso_code: "MEX".to_string(),
        },
    );
    alpha3.insert(
        "Micronesia".to_string(),
        CountyStatistic {
            iso_code: "FSM".to_string(),
        },
    );
    alpha3.insert(
        "Moldova".to_string(),
        CountyStatistic {
            iso_code: "MDA".to_string(),
        },
    );
    alpha3.insert(
        "Monaco".to_string(),
        CountyStatistic {
            iso_code: "MCO".to_string(),
        },
    );
    alpha3.insert(
        "Mongolia".to_string(),
        CountyStatistic {
            iso_code: "MNG".to_string(),
        },
    );
    alpha3.insert(
        "Montenegro".to_string(),
        CountyStatistic {
            iso_code: "MNE".to_string(),
        },
    );
    alpha3.insert(
        "Montserrat".to_string(),
        CountyStatistic {
            iso_code: "MSR".to_string(),
        },
    );
    alpha3.insert(
        "Morocco".to_string(),
        CountyStatistic {
            iso_code: "MAR".to_string(),
        },
    );
    alpha3.insert(
        "Mozambique".to_string(),
        CountyStatistic {
            iso_code: "MOZ".to_string(),
        },
    );
    alpha3.insert(
        "Myanmar".to_string(),
        CountyStatistic {
            iso_code: "MMR".to_string(),
        },
    );
    alpha3.insert(
        "Namibia".to_string(),
        CountyStatistic {
            iso_code: "NAM".to_string(),
        },
    );
    alpha3.insert(
        "Nauru".to_string(),
        CountyStatistic {
            iso_code: "NRU".to_string(),
        },
    );
    alpha3.insert(
        "Nepal".to_string(),
        CountyStatistic {
            iso_code: "NPL".to_string(),
        },
    );
    alpha3.insert(
        "Netherlands".to_string(),
        CountyStatistic {
            iso_code: "NLD".to_string(),
        },
    );
    alpha3.insert(
        "New Caledonia".to_string(),
        CountyStatistic {
            iso_code: "NCL".to_string(),
        },
    );
    alpha3.insert(
        "New Zealand".to_string(),
        CountyStatistic {
            iso_code: "NZL".to_string(),
        },
    );
    alpha3.insert(
        "Nicaragua".to_string(),
        CountyStatistic {
            iso_code: "NIC".to_string(),
        },
    );
    alpha3.insert(
        "Niger".to_string(),
        CountyStatistic {
            iso_code: "NER".to_string(),
        },
    );
    alpha3.insert(
        "Nigeria".to_string(),
        CountyStatistic {
            iso_code: "NGA".to_string(),
        },
    );
    alpha3.insert(
        "Niue".to_string(),
        CountyStatistic {
            iso_code: "NIU".to_string(),
        },
    );
    alpha3.insert(
        "Norfolk Island".to_string(),
        CountyStatistic {
            iso_code: "NFK".to_string(),
        },
    );
    alpha3.insert(
        "Northern Mariana Islands".to_string(),
        CountyStatistic {
            iso_code: "MNP".to_string(),
        },
    );
    alpha3.insert(
        "Norway".to_string(),
        CountyStatistic {
            iso_code: "NOR".to_string(),
        },
    );
    alpha3.insert(
        "Oman".to_string(),
        CountyStatistic {
            iso_code: "OMN".to_string(),
        },
    );
    alpha3.insert(
        "Pakistan".to_string(),
        CountyStatistic {
            iso_code: "PAK".to_string(),
        },
    );
    alpha3.insert(
        "Palau".to_string(),
        CountyStatistic {
            iso_code: "PLW".to_string(),
        },
    );
    alpha3.insert(
        "Palestine".to_string(),
        CountyStatistic {
            iso_code: "PSE".to_string(),
        },
    );
    alpha3.insert(
        "Panama".to_string(),
        CountyStatistic {
            iso_code: "PAN".to_string(),
        },
    );
    alpha3.insert(
        "Papua New Guinea".to_string(),
        CountyStatistic {
            iso_code: "PNG".to_string(),
        },
    );
    alpha3.insert(
        "Paraguay".to_string(),
        CountyStatistic {
            iso_code: "PRY".to_string(),
        },
    );
    alpha3.insert(
        "Peru".to_string(),
        CountyStatistic {
            iso_code: "PER".to_string(),
        },
    );
    alpha3.insert(
        "Philippines".to_string(),
        CountyStatistic {
            iso_code: "PHL".to_string(),
        },
    );
    alpha3.insert(
        "Pitcairn".to_string(),
        CountyStatistic {
            iso_code: "PCN".to_string(),
        },
    );
    alpha3.insert(
        "Poland".to_string(),
        CountyStatistic {
            iso_code: "POL".to_string(),
        },
    );
    alpha3.insert(
        "Portugal".to_string(),
        CountyStatistic {
            iso_code: "PRT".to_string(),
        },
    );
    alpha3.insert(
        "Puerto Rico".to_string(),
        CountyStatistic {
            iso_code: "PRI".to_string(),
        },
    );
    alpha3.insert(
        "Qatar".to_string(),
        CountyStatistic {
            iso_code: "QAT".to_string(),
        },
    );
    alpha3.insert(
        "North Macedonia".to_string(),
        CountyStatistic {
            iso_code: "MKD".to_string(),
        },
    );
    alpha3.insert(
        "Romania".to_string(),
        CountyStatistic {
            iso_code: "ROU".to_string(),
        },
    );
    alpha3.insert(
        "Russia".to_string(),
        CountyStatistic {
            iso_code: "RUS".to_string(),
        },
    );
    alpha3.insert(
        "Rwanda".to_string(),
        CountyStatistic {
            iso_code: "RWA".to_string(),
        },
    );
    alpha3.insert(
        "Réunion".to_string(),
        CountyStatistic {
            iso_code: "REU".to_string(),
        },
    );
    alpha3.insert(
        "Saint Barthélemy".to_string(),
        CountyStatistic {
            iso_code: "BLM".to_string(),
        },
    );
    alpha3.insert(
        "Saint Helena, Ascension and Tristan da Cunha".to_string(),
        CountyStatistic {
            iso_code: "SHN".to_string(),
        },
    );
    alpha3.insert(
        "Saint Kitts and Nevis".to_string(),
        CountyStatistic {
            iso_code: "KNA".to_string(),
        },
    );
    alpha3.insert(
        "Saint Lucia".to_string(),
        CountyStatistic {
            iso_code: "LCA".to_string(),
        },
    );
    alpha3.insert(
        "Saint Martin (French part)".to_string(),
        CountyStatistic {
            iso_code: "MAF".to_string(),
        },
    );
    alpha3.insert(
        "Saint Pierre and Miquelon".to_string(),
        CountyStatistic {
            iso_code: "SPM".to_string(),
        },
    );
    alpha3.insert(
        "Saint Vincent and the Grenadines".to_string(),
        CountyStatistic {
            iso_code: "VCT".to_string(),
        },
    );
    alpha3.insert(
        "Samoa".to_string(),
        CountyStatistic {
            iso_code: "WSM".to_string(),
        },
    );
    alpha3.insert(
        "San Marino".to_string(),
        CountyStatistic {
            iso_code: "SMR".to_string(),
        },
    );
    alpha3.insert(
        "Sao Tome and Principe".to_string(),
        CountyStatistic {
            iso_code: "STP".to_string(),
        },
    );
    alpha3.insert(
        "Saudi Arabia".to_string(),
        CountyStatistic {
            iso_code: "SAU".to_string(),
        },
    );
    alpha3.insert(
        "Senegal".to_string(),
        CountyStatistic {
            iso_code: "SEN".to_string(),
        },
    );
    alpha3.insert(
        "Serbia".to_string(),
        CountyStatistic {
            iso_code: "SRB".to_string(),
        },
    );
    alpha3.insert(
        "Seychelles".to_string(),
        CountyStatistic {
            iso_code: "SYC".to_string(),
        },
    );
    alpha3.insert(
        "Sierra Leone".to_string(),
        CountyStatistic {
            iso_code: "SLE".to_string(),
        },
    );
    alpha3.insert(
        "Singapore".to_string(),
        CountyStatistic {
            iso_code: "SGP".to_string(),
        },
    );
    alpha3.insert(
        "Sint Maarten (Dutch part)".to_string(),
        CountyStatistic {
            iso_code: "SXM".to_string(),
        },
    );
    alpha3.insert(
        "Slovakia".to_string(),
        CountyStatistic {
            iso_code: "SVK".to_string(),
        },
    );
    alpha3.insert(
        "Slovenia".to_string(),
        CountyStatistic {
            iso_code: "SVN".to_string(),
        },
    );
    alpha3.insert(
        "Solomon Islands".to_string(),
        CountyStatistic {
            iso_code: "SLB".to_string(),
        },
    );
    alpha3.insert(
        "Somalia".to_string(),
        CountyStatistic {
            iso_code: "SOM".to_string(),
        },
    );
    alpha3.insert(
        "South Africa".to_string(),
        CountyStatistic {
            iso_code: "ZAF".to_string(),
        },
    );
    alpha3.insert(
        "South Georgia and the South Sandwich Islands".to_string(),
        CountyStatistic {
            iso_code: "SGS".to_string(),
        },
    );
    alpha3.insert(
        "South Sudan".to_string(),
        CountyStatistic {
            iso_code: "SSD".to_string(),
        },
    );
    alpha3.insert(
        "Spain".to_string(),
        CountyStatistic {
            iso_code: "ESP".to_string(),
        },
    );
    alpha3.insert(
        "Sri Lanka".to_string(),
        CountyStatistic {
            iso_code: "LKA".to_string(),
        },
    );
    alpha3.insert(
        "Sudan".to_string(),
        CountyStatistic {
            iso_code: "SDN".to_string(),
        },
    );
    alpha3.insert(
        "Suriname".to_string(),
        CountyStatistic {
            iso_code: "SUR".to_string(),
        },
    );
    alpha3.insert(
        "Svalbard and Jan Mayen".to_string(),
        CountyStatistic {
            iso_code: "SJM".to_string(),
        },
    );
    alpha3.insert(
        "Sweden".to_string(),
        CountyStatistic {
            iso_code: "SWE".to_string(),
        },
    );
    alpha3.insert(
        "Switzerland".to_string(),
        CountyStatistic {
            iso_code: "CHE".to_string(),
        },
    );
    alpha3.insert(
        "Syria".to_string(),
        CountyStatistic {
            iso_code: "SYR".to_string(),
        },
    );
    alpha3.insert(
        "Taiwan".to_string(),
        CountyStatistic {
            iso_code: "TWN".to_string(),
        },
    );
    alpha3.insert(
        "Tajikistan".to_string(),
        CountyStatistic {
            iso_code: "TJK".to_string(),
        },
    );
    alpha3.insert(
        "Tanzania".to_string(),
        CountyStatistic {
            iso_code: "TZA".to_string(),
        },
    );
    alpha3.insert(
        "Thailand".to_string(),
        CountyStatistic {
            iso_code: "THA".to_string(),
        },
    );
    alpha3.insert(
        "Timor-Leste".to_string(),
        CountyStatistic {
            iso_code: "TLS".to_string(),
        },
    );
    alpha3.insert(
        "Togo".to_string(),
        CountyStatistic {
            iso_code: "TGO".to_string(),
        },
    );
    alpha3.insert(
        "Tokelau".to_string(),
        CountyStatistic {
            iso_code: "TKL".to_string(),
        },
    );
    alpha3.insert(
        "Tonga".to_string(),
        CountyStatistic {
            iso_code: "TON".to_string(),
        },
    );
    alpha3.insert(
        "Trinidad and Tobago".to_string(),
        CountyStatistic {
            iso_code: "TTO".to_string(),
        },
    );
    alpha3.insert(
        "Tunisia".to_string(),
        CountyStatistic {
            iso_code: "TUN".to_string(),
        },
    );
    alpha3.insert(
        "Turkey".to_string(),
        CountyStatistic {
            iso_code: "TUR".to_string(),
        },
    );
    alpha3.insert(
        "Turkmenistan".to_string(),
        CountyStatistic {
            iso_code: "TKM".to_string(),
        },
    );
    alpha3.insert(
        "Turks and Caicos Islands".to_string(),
        CountyStatistic {
            iso_code: "TCA".to_string(),
        },
    );
    alpha3.insert(
        "Tuvalu".to_string(),
        CountyStatistic {
            iso_code: "TUV".to_string(),
        },
    );
    alpha3.insert(
        "Uganda".to_string(),
        CountyStatistic {
            iso_code: "UGA".to_string(),
        },
    );
    alpha3.insert(
        "Ukraine".to_string(),
        CountyStatistic {
            iso_code: "UKR".to_string(),
        },
    );
    alpha3.insert(
        "United Arab Emirates".to_string(),
        CountyStatistic {
            iso_code: "ARE".to_string(),
        },
    );
    alpha3.insert(
        "United Kingdom".to_string(),
        CountyStatistic {
            iso_code: "GBR".to_string(),
        },
    );
    alpha3.insert(
        "United States Minor Outlying Islands".to_string(),
        CountyStatistic {
            iso_code: "UMI".to_string(),
        },
    );
    alpha3.insert(
        "United States".to_string(),
        CountyStatistic {
            iso_code: "USA".to_string(),
        },
    );
    alpha3.insert(
        "Uruguay".to_string(),
        CountyStatistic {
            iso_code: "URY".to_string(),
        },
    );
    alpha3.insert(
        "Uzbekistan".to_string(),
        CountyStatistic {
            iso_code: "UZB".to_string(),
        },
    );
    alpha3.insert(
        "Vanuatu".to_string(),
        CountyStatistic {
            iso_code: "VUT".to_string(),
        },
    );
    alpha3.insert(
        "Venezuela".to_string(),
        CountyStatistic {
            iso_code: "VEN".to_string(),
        },
    );
    alpha3.insert(
        "Vietnam".to_string(),
        CountyStatistic {
            iso_code: "VNM".to_string(),
        },
    );
    alpha3.insert(
        "Virgin Islands (British)".to_string(),
        CountyStatistic {
            iso_code: "VGB".to_string(),
        },
    );
    alpha3.insert(
        "Virgin Islands (U.S.)".to_string(),
        CountyStatistic {
            iso_code: "VIR".to_string(),
        },
    );
    alpha3.insert(
        "Wallis and Futuna".to_string(),
        CountyStatistic {
            iso_code: "WLF".to_string(),
        },
    );
    alpha3.insert(
        "Western Sahara".to_string(),
        CountyStatistic {
            iso_code: "ESH".to_string(),
        },
    );
    alpha3.insert(
        "Yemen".to_string(),
        CountyStatistic {
            iso_code: "YEM".to_string(),
        },
    );
    alpha3.insert(
        "Zambia".to_string(),
        CountyStatistic {
            iso_code: "ZMB".to_string(),
        },
    );
    alpha3.insert(
        "Zimbabwe".to_string(),
        CountyStatistic {
            iso_code: "ZWE".to_string(),
        },
    );
    alpha3.insert(
        "Åland Islands".to_string(),
        CountyStatistic {
            iso_code: "ALA".to_string(),
        },
    );

    alpha3
}

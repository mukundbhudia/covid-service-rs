use std::collections::HashMap;

// pub mod alpha3_country_codes {
pub fn alpha_codes() -> HashMap<String, String> {
    let mut alpha3 = HashMap::new();
    alpha3.insert("Afghanistan".to_string(), "AFG".to_string());
    alpha3.insert("Albania".to_string(), "ALB".to_string());
    alpha3.insert("Algeria".to_string(), "DZA".to_string());
    alpha3.insert("American Samoa".to_string(), "ASM".to_string());
    alpha3.insert("Andorra".to_string(), "AND".to_string());
    alpha3.insert("Angola".to_string(), "AGO".to_string());
    alpha3.insert("Anguilla".to_string(), "AIA".to_string());
    alpha3.insert("Antarctica".to_string(), "ATA".to_string());
    alpha3.insert("Antigua and Barbuda".to_string(), "ATG".to_string());
    alpha3.insert("Argentina".to_string(), "ARG".to_string());
    alpha3.insert("Armenia".to_string(), "ARM".to_string());
    alpha3.insert("Aruba".to_string(), "ABW".to_string());
    alpha3.insert("Australia".to_string(), "AUS".to_string());
    alpha3.insert("Austria".to_string(), "AUT".to_string());
    alpha3.insert("Azerbaijan".to_string(), "AZE".to_string());
    alpha3.insert("Bahamas (the)".to_string(), "BHS".to_string());
    alpha3.insert("Bahrain".to_string(), "BHR".to_string());
    alpha3.insert("Bangladesh".to_string(), "BGD".to_string());
    alpha3.insert("Barbados".to_string(), "BRB".to_string());
    alpha3.insert("Belarus".to_string(), "BLR".to_string());
    alpha3.insert("Belgium".to_string(), "BEL".to_string());
    alpha3.insert("Belize".to_string(), "BLZ".to_string());
    alpha3.insert("Benin".to_string(), "BEN".to_string());
    alpha3.insert("Bermuda".to_string(), "BMU".to_string());
    alpha3.insert("Bhutan".to_string(), "BTN".to_string());
    alpha3.insert("Bolivia".to_string(), "BOL".to_string());
    alpha3.insert(
        "Bonaire, Sint Eustatius and Saba".to_string(),
        "BES".to_string(),
    );
    alpha3.insert("Bosnia and Herzegovina".to_string(), "BIH".to_string());
    alpha3.insert("Botswana".to_string(), "BWA".to_string());
    alpha3.insert("Bouvet Island".to_string(), "BVT".to_string());
    alpha3.insert("Brazil".to_string(), "BRA".to_string());
    alpha3.insert(
        "British Indian Ocean Territory (the)".to_string(),
        "IOT".to_string(),
    );
    alpha3.insert("Brunei".to_string(), "BRN".to_string());
    alpha3.insert("Bulgaria".to_string(), "BGR".to_string());
    alpha3.insert("Burkina Faso".to_string(), "BFA".to_string());
    alpha3.insert("Burundi".to_string(), "BDI".to_string());
    alpha3.insert("Cabo Verde".to_string(), "CPV".to_string());
    alpha3.insert("Cambodia".to_string(), "KHM".to_string());
    alpha3.insert("Cameroon".to_string(), "CMR".to_string());
    alpha3.insert("Canada".to_string(), "CAN".to_string());
    alpha3.insert("Cayman Islands (the)".to_string(), "CYM".to_string());
    alpha3.insert(
        "Central African Republic (the)".to_string(),
        "CAF".to_string(),
    );
    alpha3.insert("Chad".to_string(), "TCD".to_string());
    alpha3.insert("Chile".to_string(), "CHL".to_string());
    alpha3.insert("China".to_string(), "CHN".to_string());
    alpha3.insert("Christmas Island".to_string(), "CXR".to_string());
    alpha3.insert(
        "Cocos (Keeling) Islands (the)".to_string(),
        "CCK".to_string(),
    );
    alpha3.insert("Colombia".to_string(), "COL".to_string());
    alpha3.insert("Comoros (the)".to_string(), "COM".to_string());
    alpha3.insert("Congo (Kinshasa)".to_string(), "COD".to_string());
    alpha3.insert("Congo (Brazzaville)".to_string(), "COG".to_string());
    alpha3.insert("Cook Islands (the)".to_string(), "COK".to_string());
    alpha3.insert("Costa Rica".to_string(), "CRI".to_string());
    alpha3.insert("Croatia".to_string(), "HRV".to_string());
    alpha3.insert("Cuba".to_string(), "CUB".to_string());
    alpha3.insert("Curaçao".to_string(), "CUW".to_string());
    alpha3.insert("Cyprus".to_string(), "CYP".to_string());
    alpha3.insert("Czechia".to_string(), "CZE".to_string());
    alpha3.insert("Cote d'Ivoire".to_string(), "CIV".to_string());
    alpha3.insert("Denmark".to_string(), "DNK".to_string());
    alpha3.insert("Djibouti".to_string(), "DJI".to_string());
    alpha3.insert("Dominica".to_string(), "DMA".to_string());
    alpha3.insert("Dominican Republic (the)".to_string(), "DOM".to_string());
    alpha3.insert("Ecuador".to_string(), "ECU".to_string());
    alpha3.insert("Egypt".to_string(), "EGY".to_string());
    alpha3.insert("El Salvador".to_string(), "SLV".to_string());
    alpha3.insert("Equatorial Guinea".to_string(), "GNQ".to_string());
    alpha3.insert("Eritrea".to_string(), "ERI".to_string());
    alpha3.insert("Estonia".to_string(), "EST".to_string());
    alpha3.insert("Eswatini".to_string(), "SWZ".to_string());
    alpha3.insert("Ethiopia".to_string(), "ETH".to_string());
    alpha3.insert(
        "Falkland Islands (the) [Malvinas]".to_string(),
        "FLK".to_string(),
    );
    alpha3.insert("Faroe Islands (the)".to_string(), "FRO".to_string());
    alpha3.insert("Fiji".to_string(), "FJI".to_string());
    alpha3.insert("Finland".to_string(), "FIN".to_string());
    alpha3.insert("France".to_string(), "FRA".to_string());
    alpha3.insert("French Guiana".to_string(), "GUF".to_string());
    alpha3.insert("French Polynesia".to_string(), "PYF".to_string());
    alpha3.insert(
        "French Southern Territories (the)".to_string(),
        "ATF".to_string(),
    );
    alpha3.insert("Gabon".to_string(), "GAB".to_string());
    alpha3.insert("Gambia (the)".to_string(), "GMB".to_string());
    alpha3.insert("Georgia".to_string(), "GEO".to_string());
    alpha3.insert("Germany".to_string(), "DEU".to_string());
    alpha3.insert("Ghana".to_string(), "GHA".to_string());
    alpha3.insert("Gibraltar".to_string(), "GIB".to_string());
    alpha3.insert("Greece".to_string(), "GRC".to_string());
    alpha3.insert("Greenland".to_string(), "GRL".to_string());
    alpha3.insert("Grenada".to_string(), "GRD".to_string());
    alpha3.insert("Guadeloupe".to_string(), "GLP".to_string());
    alpha3.insert("Guam".to_string(), "GUM".to_string());
    alpha3.insert("Guatemala".to_string(), "GTM".to_string());
    alpha3.insert("Guernsey".to_string(), "GGY".to_string());
    alpha3.insert("Guinea".to_string(), "GIN".to_string());
    alpha3.insert("Guinea-Bissau".to_string(), "GNB".to_string());
    alpha3.insert("Guyana".to_string(), "GUY".to_string());
    alpha3.insert("Haiti".to_string(), "HTI".to_string());
    alpha3.insert(
        "Heard Island and McDonald Islands".to_string(),
        "HMD".to_string(),
    );
    alpha3.insert("Holy See (the)".to_string(), "VAT".to_string());
    alpha3.insert("Honduras".to_string(), "HND".to_string());
    alpha3.insert("Hong Kong".to_string(), "HKG".to_string());
    alpha3.insert("Hungary".to_string(), "HUN".to_string());
    alpha3.insert("Iceland".to_string(), "ISL".to_string());
    alpha3.insert("India".to_string(), "IND".to_string());
    alpha3.insert("Indonesia".to_string(), "IDN".to_string());
    alpha3.insert("Iran".to_string(), "IRN".to_string());
    alpha3.insert("Iraq".to_string(), "IRQ".to_string());
    alpha3.insert("Ireland".to_string(), "IRL".to_string());
    alpha3.insert("Isle of Man".to_string(), "IMN".to_string());
    alpha3.insert("Israel".to_string(), "ISR".to_string());
    alpha3.insert("Italy".to_string(), "ITA".to_string());
    alpha3.insert("Jamaica".to_string(), "JAM".to_string());
    alpha3.insert("Japan".to_string(), "JPN".to_string());
    alpha3.insert("Jersey".to_string(), "JEY".to_string());
    alpha3.insert("Jordan".to_string(), "JOR".to_string());
    alpha3.insert("Kazakhstan".to_string(), "KAZ".to_string());
    alpha3.insert("Kenya".to_string(), "KEN".to_string());
    alpha3.insert("Kiribati".to_string(), "KIR".to_string());
    alpha3.insert(
        "Korea (the Democratic People's Republic of)".to_string(),
        "PRK".to_string(),
    );
    alpha3.insert("Korea, South".to_string(), "KOR".to_string());
    alpha3.insert("Kuwait".to_string(), "KWT".to_string());
    alpha3.insert("Kyrgyzstan".to_string(), "KGZ".to_string());
    alpha3.insert("Laos".to_string(), "LAO".to_string());
    alpha3.insert("Latvia".to_string(), "LVA".to_string());
    alpha3.insert("Lebanon".to_string(), "LBN".to_string());
    alpha3.insert("Lesotho".to_string(), "LSO".to_string());
    alpha3.insert("Liberia".to_string(), "LBR".to_string());
    alpha3.insert("Libya".to_string(), "LBY".to_string());
    alpha3.insert("Liechtenstein".to_string(), "LIE".to_string());
    alpha3.insert("Lithuania".to_string(), "LTU".to_string());
    alpha3.insert("Luxembourg".to_string(), "LUX".to_string());
    alpha3.insert("Macao".to_string(), "MAC".to_string());
    alpha3.insert("Madagascar".to_string(), "MDG".to_string());
    alpha3.insert("Malawi".to_string(), "MWI".to_string());
    alpha3.insert("Malaysia".to_string(), "MYS".to_string());
    alpha3.insert("Maldives".to_string(), "MDV".to_string());
    alpha3.insert("Mali".to_string(), "MLI".to_string());
    alpha3.insert("Malta".to_string(), "MLT".to_string());
    alpha3.insert("Marshall Islands (the)".to_string(), "MHL".to_string());
    alpha3.insert("Martinique".to_string(), "MTQ".to_string());
    alpha3.insert("Mauritania".to_string(), "MRT".to_string());
    alpha3.insert("Mauritius".to_string(), "MUS".to_string());
    alpha3.insert("Mayotte".to_string(), "MYT".to_string());
    alpha3.insert("Mexico".to_string(), "MEX".to_string());
    alpha3.insert(
        "Micronesia (Federated States of)".to_string(),
        "FSM".to_string(),
    );
    alpha3.insert("Moldova".to_string(), "MDA".to_string());
    alpha3.insert("Monaco".to_string(), "MCO".to_string());
    alpha3.insert("Mongolia".to_string(), "MNG".to_string());
    alpha3.insert("Montenegro".to_string(), "MNE".to_string());
    alpha3.insert("Montserrat".to_string(), "MSR".to_string());
    alpha3.insert("Morocco".to_string(), "MAR".to_string());
    alpha3.insert("Mozambique".to_string(), "MOZ".to_string());
    alpha3.insert("Burma".to_string(), "MMR".to_string());
    alpha3.insert("Namibia".to_string(), "NAM".to_string());
    alpha3.insert("Nauru".to_string(), "NRU".to_string());
    alpha3.insert("Nepal".to_string(), "NPL".to_string());
    alpha3.insert("Netherlands (the)".to_string(), "NLD".to_string());
    alpha3.insert("New Caledonia".to_string(), "NCL".to_string());
    alpha3.insert("New Zealand".to_string(), "NZL".to_string());
    alpha3.insert("Nicaragua".to_string(), "NIC".to_string());
    alpha3.insert("Niger (the)".to_string(), "NER".to_string());
    alpha3.insert("Nigeria".to_string(), "NGA".to_string());
    alpha3.insert("Niue".to_string(), "NIU".to_string());
    alpha3.insert("Norfolk Island".to_string(), "NFK".to_string());
    alpha3.insert(
        "Northern Mariana Islands (the)".to_string(),
        "MNP".to_string(),
    );
    alpha3.insert("Norway".to_string(), "NOR".to_string());
    alpha3.insert("Oman".to_string(), "OMN".to_string());
    alpha3.insert("Pakistan".to_string(), "PAK".to_string());
    alpha3.insert("Palau".to_string(), "PLW".to_string());
    alpha3.insert("Palestine, State of".to_string(), "PSE".to_string());
    alpha3.insert("Panama".to_string(), "PAN".to_string());
    alpha3.insert("Papua New Guinea".to_string(), "PNG".to_string());
    alpha3.insert("Paraguay".to_string(), "PRY".to_string());
    alpha3.insert("Peru".to_string(), "PER".to_string());
    alpha3.insert("Philippines (the)".to_string(), "PHL".to_string());
    alpha3.insert("Pitcairn".to_string(), "PCN".to_string());
    alpha3.insert("Poland".to_string(), "POL".to_string());
    alpha3.insert("Portugal".to_string(), "PRT".to_string());
    alpha3.insert("Puerto Rico".to_string(), "PRI".to_string());
    alpha3.insert("Qatar".to_string(), "QAT".to_string());
    alpha3.insert("Republic of North Macedonia".to_string(), "MKD".to_string());
    alpha3.insert("Romania".to_string(), "ROU".to_string());
    alpha3.insert("Russian".to_string(), "RUS".to_string());
    alpha3.insert("Rwanda".to_string(), "RWA".to_string());
    alpha3.insert("Réunion".to_string(), "REU".to_string());
    alpha3.insert("Saint Barthélemy".to_string(), "BLM".to_string());
    alpha3.insert(
        "Saint Helena, Ascension and Tristan da Cunha".to_string(),
        "SHN".to_string(),
    );
    alpha3.insert("Saint Kitts and Nevis".to_string(), "KNA".to_string());
    alpha3.insert("Saint Lucia".to_string(), "LCA".to_string());
    alpha3.insert("Saint Martin (French part)".to_string(), "MAF".to_string());
    alpha3.insert("Saint Pierre and Miquelon".to_string(), "SPM".to_string());
    alpha3.insert(
        "Saint Vincent and the Grenadines".to_string(),
        "VCT".to_string(),
    );
    alpha3.insert("Samoa".to_string(), "WSM".to_string());
    alpha3.insert("San Marino".to_string(), "SMR".to_string());
    alpha3.insert("Sao Tome and Principe".to_string(), "STP".to_string());
    alpha3.insert("Saudi Arabia".to_string(), "SAU".to_string());
    alpha3.insert("Senegal".to_string(), "SEN".to_string());
    alpha3.insert("Serbia".to_string(), "SRB".to_string());
    alpha3.insert("Seychelles".to_string(), "SYC".to_string());
    alpha3.insert("Sierra Leone".to_string(), "SLE".to_string());
    alpha3.insert("Singapore".to_string(), "SGP".to_string());
    alpha3.insert("Sint Maarten (Dutch part)".to_string(), "SXM".to_string());
    alpha3.insert("Slovakia".to_string(), "SVK".to_string());
    alpha3.insert("Slovenia".to_string(), "SVN".to_string());
    alpha3.insert("Solomon Islands".to_string(), "SLB".to_string());
    alpha3.insert("Somalia".to_string(), "SOM".to_string());
    alpha3.insert("South Africa".to_string(), "ZAF".to_string());
    alpha3.insert(
        "South Georgia and the South Sandwich Islands".to_string(),
        "SGS".to_string(),
    );
    alpha3.insert("South Sudan".to_string(), "SSD".to_string());
    alpha3.insert("Spain".to_string(), "ESP".to_string());
    alpha3.insert("Sri Lanka".to_string(), "LKA".to_string());
    alpha3.insert("Sudan (the)".to_string(), "SDN".to_string());
    alpha3.insert("Suriname".to_string(), "SUR".to_string());
    alpha3.insert("Svalbard and Jan Mayen".to_string(), "SJM".to_string());
    alpha3.insert("Sweden".to_string(), "SWE".to_string());
    alpha3.insert("Switzerland".to_string(), "CHE".to_string());
    alpha3.insert("Syrian".to_string(), "SYR".to_string());
    alpha3.insert("Taiwan*".to_string(), "TWN".to_string());
    alpha3.insert("Tajikistan".to_string(), "TJK".to_string());
    alpha3.insert("Tanzania".to_string(), "TZA".to_string());
    alpha3.insert("Thailand".to_string(), "THA".to_string());
    alpha3.insert("Timor-Leste".to_string(), "TLS".to_string());
    alpha3.insert("Togo".to_string(), "TGO".to_string());
    alpha3.insert("Tokelau".to_string(), "TKL".to_string());
    alpha3.insert("Tonga".to_string(), "TON".to_string());
    alpha3.insert("Trinidad and Tobago".to_string(), "TTO".to_string());
    alpha3.insert("Tunisia".to_string(), "TUN".to_string());
    alpha3.insert("Turkey".to_string(), "TUR".to_string());
    alpha3.insert("Turkmenistan".to_string(), "TKM".to_string());
    alpha3.insert(
        "Turks and Caicos Islands (the)".to_string(),
        "TCA".to_string(),
    );
    alpha3.insert("Tuvalu".to_string(), "TUV".to_string());
    alpha3.insert("Uganda".to_string(), "UGA".to_string());
    alpha3.insert("Ukraine".to_string(), "UKR".to_string());
    alpha3.insert("United Arab Emirates (the)".to_string(), "ARE".to_string());
    alpha3.insert("United Kingdom".to_string(), "GBR".to_string());
    alpha3.insert(
        "United States Minor Outlying Islands (the)".to_string(),
        "UMI".to_string(),
    );
    alpha3.insert("US".to_string(), "USA".to_string());
    alpha3.insert("Uruguay".to_string(), "URY".to_string());
    alpha3.insert("Uzbekistan".to_string(), "UZB".to_string());
    alpha3.insert("Vanuatu".to_string(), "VUT".to_string());
    alpha3.insert("Venezuela".to_string(), "VEN".to_string());
    alpha3.insert("Vietnam".to_string(), "VNM".to_string());
    alpha3.insert("Virgin Islands (British)".to_string(), "VGB".to_string());
    alpha3.insert("Virgin Islands (U.S.)".to_string(), "VIR".to_string());
    alpha3.insert("Wallis and Futuna".to_string(), "WLF".to_string());
    alpha3.insert("Western Sahara".to_string(), "ESH".to_string());
    alpha3.insert("Yemen".to_string(), "YEM".to_string());
    alpha3.insert("Zambia".to_string(), "ZMB".to_string());
    alpha3.insert("Zimbabwe".to_string(), "ZWE".to_string());
    alpha3.insert("Åland Islands".to_string(), "ALA".to_string());

    alpha3
}
// }

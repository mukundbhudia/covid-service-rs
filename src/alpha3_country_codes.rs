use crate::schema::CountyStatistic;
use std::collections::HashMap;

pub fn alpha_codes() -> HashMap<String, CountyStatistic> {
    let mut alpha3 = HashMap::new();
    alpha3.insert(
        "Afghanistan".to_string(),
        CountyStatistic {
            iso_code: "AFG".to_string(),
            country_name: "Afghanistan".to_string(),
            continent: "Asia".to_string(),
            population: 38928341,
            population_density: Some(54.4),
            median_age: Some(18.6),
            aged_65_older: Some(2.6),
            aged_70_older: Some(1.3),
            gdp_per_capita: Some(1804.0),
            diabetes_prevalence: Some(9.6),
            cardiovasc_death_rate: Some(597.0),
            life_expectancy: 64.8,
            human_development_index: Some(0.5),
        },
    );

    alpha3.insert(
        "Albania".to_string(),
        CountyStatistic {
            iso_code: "ALB".to_string(),
            country_name: "Albania".to_string(),
            continent: "Europe".to_string(),
            population: 2877800,
            population_density: Some(104.9),
            median_age: Some(38.0),
            aged_65_older: Some(13.2),
            aged_70_older: Some(8.6),
            gdp_per_capita: Some(11803.4),
            diabetes_prevalence: Some(10.1),
            cardiovasc_death_rate: Some(304.2),
            life_expectancy: 78.6,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Algeria".to_string(),
        CountyStatistic {
            iso_code: "DZA".to_string(),
            country_name: "Algeria".to_string(),
            continent: "Africa".to_string(),
            population: 43851043,
            population_density: Some(17.3),
            median_age: Some(29.1),
            aged_65_older: Some(6.2),
            aged_70_older: Some(3.9),
            gdp_per_capita: Some(13913.8),
            diabetes_prevalence: Some(6.7),
            cardiovasc_death_rate: Some(278.4),
            life_expectancy: 76.9,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Andorra".to_string(),
        CountyStatistic {
            iso_code: "AND".to_string(),
            country_name: "Andorra".to_string(),
            continent: "Europe".to_string(),
            population: 77265,
            population_density: Some(163.8),
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: Some(8.0),
            cardiovasc_death_rate: Some(109.1),
            life_expectancy: 83.7,
            human_development_index: Some(0.9),
        },
    );

    alpha3.insert(
        "Angola".to_string(),
        CountyStatistic {
            iso_code: "AGO".to_string(),
            country_name: "Angola".to_string(),
            continent: "Africa".to_string(),
            population: 32866268,
            population_density: Some(23.9),
            median_age: Some(16.8),
            aged_65_older: Some(2.4),
            aged_70_older: Some(1.4),
            gdp_per_capita: Some(5819.5),
            diabetes_prevalence: Some(3.9),
            cardiovasc_death_rate: Some(276.0),
            life_expectancy: 61.1,
            human_development_index: Some(0.6),
        },
    );

    alpha3.insert(
        "Antigua and Barbuda".to_string(),
        CountyStatistic {
            iso_code: "ATG".to_string(),
            country_name: "Antigua and Barbuda".to_string(),
            continent: "North America".to_string(),
            population: 97928,
            population_density: Some(231.8),
            median_age: Some(32.1),
            aged_65_older: Some(6.9),
            aged_70_older: Some(4.6),
            gdp_per_capita: Some(21490.9),
            diabetes_prevalence: Some(13.2),
            cardiovasc_death_rate: Some(191.5),
            life_expectancy: 77.0,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Argentina".to_string(),
        CountyStatistic {
            iso_code: "ARG".to_string(),
            country_name: "Argentina".to_string(),
            continent: "South America".to_string(),
            population: 45195777,
            population_density: Some(16.2),
            median_age: Some(31.9),
            aged_65_older: Some(11.2),
            aged_70_older: Some(7.4),
            gdp_per_capita: Some(18933.9),
            diabetes_prevalence: Some(5.5),
            cardiovasc_death_rate: Some(191.0),
            life_expectancy: 76.7,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Armenia".to_string(),
        CountyStatistic {
            iso_code: "ARM".to_string(),
            country_name: "Armenia".to_string(),
            continent: "Asia".to_string(),
            population: 2963234,
            population_density: Some(102.9),
            median_age: Some(35.7),
            aged_65_older: Some(11.2),
            aged_70_older: Some(7.6),
            gdp_per_capita: Some(8787.6),
            diabetes_prevalence: Some(7.1),
            cardiovasc_death_rate: Some(341.0),
            life_expectancy: 75.1,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Australia".to_string(),
        CountyStatistic {
            iso_code: "AUS".to_string(),
            country_name: "Australia".to_string(),
            continent: "Oceania".to_string(),
            population: 25499881,
            population_density: Some(3.2),
            median_age: Some(37.9),
            aged_65_older: Some(15.5),
            aged_70_older: Some(10.1),
            gdp_per_capita: Some(44648.7),
            diabetes_prevalence: Some(5.1),
            cardiovasc_death_rate: Some(107.8),
            life_expectancy: 83.4,
            human_development_index: Some(0.9),
        },
    );

    alpha3.insert(
        "Austria".to_string(),
        CountyStatistic {
            iso_code: "AUT".to_string(),
            country_name: "Austria".to_string(),
            continent: "Europe".to_string(),
            population: 9006400,
            population_density: Some(106.7),
            median_age: Some(44.4),
            aged_65_older: Some(19.2),
            aged_70_older: Some(13.7),
            gdp_per_capita: Some(45436.7),
            diabetes_prevalence: Some(6.3),
            cardiovasc_death_rate: Some(145.2),
            life_expectancy: 81.5,
            human_development_index: Some(0.9),
        },
    );

    alpha3.insert(
        "Azerbaijan".to_string(),
        CountyStatistic {
            iso_code: "AZE".to_string(),
            country_name: "Azerbaijan".to_string(),
            continent: "Asia".to_string(),
            population: 10139175,
            population_density: Some(119.3),
            median_age: Some(32.4),
            aged_65_older: Some(6.0),
            aged_70_older: Some(3.9),
            gdp_per_capita: Some(15847.4),
            diabetes_prevalence: Some(7.1),
            cardiovasc_death_rate: Some(559.8),
            life_expectancy: 73.0,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Bahamas".to_string(),
        CountyStatistic {
            iso_code: "BHS".to_string(),
            country_name: "Bahamas".to_string(),
            continent: "North America".to_string(),
            population: 393248,
            population_density: Some(39.5),
            median_age: Some(34.3),
            aged_65_older: Some(9.0),
            aged_70_older: Some(5.2),
            gdp_per_capita: Some(27717.8),
            diabetes_prevalence: Some(13.2),
            cardiovasc_death_rate: Some(236.0),
            life_expectancy: 73.9,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Bahrain".to_string(),
        CountyStatistic {
            iso_code: "BHR".to_string(),
            country_name: "Bahrain".to_string(),
            continent: "Asia".to_string(),
            population: 1701583,
            population_density: Some(1935.9),
            median_age: Some(32.4),
            aged_65_older: Some(2.4),
            aged_70_older: Some(1.4),
            gdp_per_capita: Some(43290.7),
            diabetes_prevalence: Some(16.5),
            cardiovasc_death_rate: Some(151.7),
            life_expectancy: 77.3,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Bangladesh".to_string(),
        CountyStatistic {
            iso_code: "BGD".to_string(),
            country_name: "Bangladesh".to_string(),
            continent: "Asia".to_string(),
            population: 164689383,
            population_density: Some(1265.0),
            median_age: Some(27.5),
            aged_65_older: Some(5.1),
            aged_70_older: Some(3.3),
            gdp_per_capita: Some(3524.0),
            diabetes_prevalence: Some(8.4),
            cardiovasc_death_rate: Some(298.0),
            life_expectancy: 72.6,
            human_development_index: Some(0.6),
        },
    );

    alpha3.insert(
        "Barbados".to_string(),
        CountyStatistic {
            iso_code: "BRB".to_string(),
            country_name: "Barbados".to_string(),
            continent: "North America".to_string(),
            population: 287371,
            population_density: Some(664.5),
            median_age: Some(39.8),
            aged_65_older: Some(15.0),
            aged_70_older: Some(9.5),
            gdp_per_capita: Some(16978.1),
            diabetes_prevalence: Some(13.6),
            cardiovasc_death_rate: Some(170.1),
            life_expectancy: 79.2,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Belarus".to_string(),
        CountyStatistic {
            iso_code: "BLR".to_string(),
            country_name: "Belarus".to_string(),
            continent: "Europe".to_string(),
            population: 9449321,
            population_density: Some(46.9),
            median_age: Some(40.3),
            aged_65_older: Some(14.8),
            aged_70_older: Some(9.8),
            gdp_per_capita: Some(17168.0),
            diabetes_prevalence: Some(5.2),
            cardiovasc_death_rate: Some(443.1),
            life_expectancy: 74.8,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Belgium".to_string(),
        CountyStatistic {
            iso_code: "BEL".to_string(),
            country_name: "Belgium".to_string(),
            continent: "Europe".to_string(),
            population: 11589616,
            population_density: Some(375.6),
            median_age: Some(41.8),
            aged_65_older: Some(18.6),
            aged_70_older: Some(12.8),
            gdp_per_capita: Some(42658.6),
            diabetes_prevalence: Some(4.3),
            cardiovasc_death_rate: Some(114.9),
            life_expectancy: 81.6,
            human_development_index: Some(0.9),
        },
    );

    alpha3.insert(
        "Belize".to_string(),
        CountyStatistic {
            iso_code: "BLZ".to_string(),
            country_name: "Belize".to_string(),
            continent: "North America".to_string(),
            population: 397621,
            population_density: Some(16.4),
            median_age: Some(25.0),
            aged_65_older: Some(3.9),
            aged_70_older: Some(2.3),
            gdp_per_capita: Some(7824.4),
            diabetes_prevalence: Some(17.1),
            cardiovasc_death_rate: Some(177.0),
            life_expectancy: 74.6,
            human_development_index: Some(0.7),
        },
    );

    alpha3.insert(
        "Benin".to_string(),
        CountyStatistic {
            iso_code: "BEN".to_string(),
            country_name: "Benin".to_string(),
            continent: "Africa".to_string(),
            population: 12123198,
            population_density: Some(99.1),
            median_age: Some(18.8),
            aged_65_older: Some(3.2),
            aged_70_older: Some(1.9),
            gdp_per_capita: Some(2064.2),
            diabetes_prevalence: Some(1.0),
            cardiovasc_death_rate: Some(235.8),
            life_expectancy: 61.8,
            human_development_index: Some(0.5),
        },
    );

    alpha3.insert(
        "Bhutan".to_string(),
        CountyStatistic {
            iso_code: "BTN".to_string(),
            country_name: "Bhutan".to_string(),
            continent: "Asia".to_string(),
            population: 771612,
            population_density: Some(21.2),
            median_age: Some(28.6),
            aged_65_older: Some(4.9),
            aged_70_older: Some(3.0),
            gdp_per_capita: Some(8708.6),
            diabetes_prevalence: Some(9.8),
            cardiovasc_death_rate: Some(217.1),
            life_expectancy: 71.8,
            human_development_index: Some(0.6),
        },
    );

    alpha3.insert(
        "Bolivia".to_string(),
        CountyStatistic {
            iso_code: "BOL".to_string(),
            country_name: "Bolivia".to_string(),
            continent: "South America".to_string(),
            population: 11673029,
            population_density: Some(10.2),
            median_age: Some(25.4),
            aged_65_older: Some(6.7),
            aged_70_older: Some(4.4),
            gdp_per_capita: Some(6885.8),
            diabetes_prevalence: Some(6.9),
            cardiovasc_death_rate: Some(204.3),
            life_expectancy: 71.5,
            human_development_index: Some(0.7),
        },
    );

    alpha3.insert(
        "Bosnia and Herzegovina".to_string(),
        CountyStatistic {
            iso_code: "BIH".to_string(),
            country_name: "Bosnia and Herzegovina".to_string(),
            continent: "Europe".to_string(),
            population: 3280815,
            population_density: Some(68.5),
            median_age: Some(42.5),
            aged_65_older: Some(16.6),
            aged_70_older: Some(10.7),
            gdp_per_capita: Some(11713.9),
            diabetes_prevalence: Some(10.1),
            cardiovasc_death_rate: Some(329.6),
            life_expectancy: 77.4,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Botswana".to_string(),
        CountyStatistic {
            iso_code: "BWA".to_string(),
            country_name: "Botswana".to_string(),
            continent: "Africa".to_string(),
            population: 2351625,
            population_density: Some(4.0),
            median_age: Some(25.8),
            aged_65_older: Some(3.9),
            aged_70_older: Some(2.2),
            gdp_per_capita: Some(15807.4),
            diabetes_prevalence: Some(4.8),
            cardiovasc_death_rate: Some(237.4),
            life_expectancy: 69.6,
            human_development_index: Some(0.7),
        },
    );

    alpha3.insert(
        "Brazil".to_string(),
        CountyStatistic {
            iso_code: "BRA".to_string(),
            country_name: "Brazil".to_string(),
            continent: "South America".to_string(),
            population: 212559409,
            population_density: Some(25.0),
            median_age: Some(33.5),
            aged_65_older: Some(8.6),
            aged_70_older: Some(5.1),
            gdp_per_capita: Some(14103.5),
            diabetes_prevalence: Some(8.1),
            cardiovasc_death_rate: Some(178.0),
            life_expectancy: 75.9,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Brunei".to_string(),
        CountyStatistic {
            iso_code: "BRN".to_string(),
            country_name: "Brunei".to_string(),
            continent: "Asia".to_string(),
            population: 437483,
            population_density: Some(81.3),
            median_age: Some(32.4),
            aged_65_older: Some(4.6),
            aged_70_older: Some(2.4),
            gdp_per_capita: Some(71809.3),
            diabetes_prevalence: Some(12.8),
            cardiovasc_death_rate: Some(201.3),
            life_expectancy: 75.9,
            human_development_index: Some(0.9),
        },
    );

    alpha3.insert(
        "Bulgaria".to_string(),
        CountyStatistic {
            iso_code: "BGR".to_string(),
            country_name: "Bulgaria".to_string(),
            continent: "Europe".to_string(),
            population: 6948445,
            population_density: Some(65.2),
            median_age: Some(44.7),
            aged_65_older: Some(20.8),
            aged_70_older: Some(13.3),
            gdp_per_capita: Some(18563.3),
            diabetes_prevalence: Some(5.8),
            cardiovasc_death_rate: Some(424.7),
            life_expectancy: 75.0,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Burkina Faso".to_string(),
        CountyStatistic {
            iso_code: "BFA".to_string(),
            country_name: "Burkina Faso".to_string(),
            continent: "Africa".to_string(),
            population: 20903278,
            population_density: Some(70.2),
            median_age: Some(17.6),
            aged_65_older: Some(2.4),
            aged_70_older: Some(1.4),
            gdp_per_capita: Some(1703.1),
            diabetes_prevalence: Some(2.4),
            cardiovasc_death_rate: Some(269.0),
            life_expectancy: 61.6,
            human_development_index: Some(0.4),
        },
    );

    alpha3.insert(
        "Burundi".to_string(),
        CountyStatistic {
            iso_code: "BDI".to_string(),
            country_name: "Burundi".to_string(),
            continent: "Africa".to_string(),
            population: 11890781,
            population_density: Some(423.1),
            median_age: Some(17.5),
            aged_65_older: Some(2.6),
            aged_70_older: Some(1.5),
            gdp_per_capita: Some(702.2),
            diabetes_prevalence: Some(6.0),
            cardiovasc_death_rate: Some(293.1),
            life_expectancy: 61.6,
            human_development_index: Some(0.4),
        },
    );

    alpha3.insert(
        "Cabo Verde".to_string(),
        CountyStatistic {
            iso_code: "CPV".to_string(),
            country_name: "Cabo Verde".to_string(),
            continent: "Africa".to_string(),
            population: 555988,
            population_density: Some(135.6),
            median_age: Some(25.7),
            aged_65_older: Some(4.5),
            aged_70_older: Some(3.4),
            gdp_per_capita: Some(6222.6),
            diabetes_prevalence: Some(2.4),
            cardiovasc_death_rate: Some(182.2),
            life_expectancy: 73.0,
            human_development_index: Some(0.7),
        },
    );

    alpha3.insert(
        "Cambodia".to_string(),
        CountyStatistic {
            iso_code: "KHM".to_string(),
            country_name: "Cambodia".to_string(),
            continent: "Asia".to_string(),
            population: 16718971,
            population_density: Some(90.7),
            median_age: Some(25.6),
            aged_65_older: Some(4.4),
            aged_70_older: Some(2.4),
            gdp_per_capita: Some(3645.1),
            diabetes_prevalence: Some(4.0),
            cardiovasc_death_rate: Some(270.9),
            life_expectancy: 69.8,
            human_development_index: Some(0.6),
        },
    );

    alpha3.insert(
        "Cameroon".to_string(),
        CountyStatistic {
            iso_code: "CMR".to_string(),
            country_name: "Cameroon".to_string(),
            continent: "Africa".to_string(),
            population: 26545864,
            population_density: Some(50.9),
            median_age: Some(18.8),
            aged_65_older: Some(3.2),
            aged_70_older: Some(1.9),
            gdp_per_capita: Some(3364.9),
            diabetes_prevalence: Some(7.2),
            cardiovasc_death_rate: Some(244.7),
            life_expectancy: 59.3,
            human_development_index: Some(0.6),
        },
    );

    alpha3.insert(
        "Canada".to_string(),
        CountyStatistic {
            iso_code: "CAN".to_string(),
            country_name: "Canada".to_string(),
            continent: "North America".to_string(),
            population: 37742157,
            population_density: Some(4.0),
            median_age: Some(41.4),
            aged_65_older: Some(17.0),
            aged_70_older: Some(10.8),
            gdp_per_capita: Some(44017.6),
            diabetes_prevalence: Some(7.4),
            cardiovasc_death_rate: Some(105.6),
            life_expectancy: 82.4,
            human_development_index: Some(0.9),
        },
    );

    alpha3.insert(
        "Central African Republic".to_string(),
        CountyStatistic {
            iso_code: "CAF".to_string(),
            country_name: "Central African Republic".to_string(),
            continent: "Africa".to_string(),
            population: 4829764,
            population_density: Some(7.5),
            median_age: Some(18.3),
            aged_65_older: Some(3.7),
            aged_70_older: Some(2.3),
            gdp_per_capita: Some(661.2),
            diabetes_prevalence: Some(6.1),
            cardiovasc_death_rate: Some(435.7),
            life_expectancy: 53.3,
            human_development_index: Some(0.4),
        },
    );

    alpha3.insert(
        "Chad".to_string(),
        CountyStatistic {
            iso_code: "TCD".to_string(),
            country_name: "Chad".to_string(),
            continent: "Africa".to_string(),
            population: 16425859,
            population_density: Some(11.8),
            median_age: Some(16.7),
            aged_65_older: Some(2.5),
            aged_70_older: Some(1.4),
            gdp_per_capita: Some(1768.2),
            diabetes_prevalence: Some(6.1),
            cardiovasc_death_rate: Some(281.0),
            life_expectancy: 54.2,
            human_development_index: Some(0.4),
        },
    );

    alpha3.insert(
        "Chile".to_string(),
        CountyStatistic {
            iso_code: "CHL".to_string(),
            country_name: "Chile".to_string(),
            continent: "South America".to_string(),
            population: 19116209,
            population_density: Some(24.3),
            median_age: Some(35.4),
            aged_65_older: Some(11.1),
            aged_70_older: Some(6.9),
            gdp_per_capita: Some(22767.0),
            diabetes_prevalence: Some(8.5),
            cardiovasc_death_rate: Some(128.0),
            life_expectancy: 80.2,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "China".to_string(),
        CountyStatistic {
            iso_code: "CHN".to_string(),
            country_name: "China".to_string(),
            continent: "Asia".to_string(),
            population: 1439323774,
            population_density: Some(147.7),
            median_age: Some(38.7),
            aged_65_older: Some(10.6),
            aged_70_older: Some(5.9),
            gdp_per_capita: Some(15308.7),
            diabetes_prevalence: Some(9.7),
            cardiovasc_death_rate: Some(261.9),
            life_expectancy: 76.9,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Colombia".to_string(),
        CountyStatistic {
            iso_code: "COL".to_string(),
            country_name: "Colombia".to_string(),
            continent: "South America".to_string(),
            population: 50882884,
            population_density: Some(44.2),
            median_age: Some(32.2),
            aged_65_older: Some(7.6),
            aged_70_older: Some(4.3),
            gdp_per_capita: Some(13254.9),
            diabetes_prevalence: Some(7.4),
            cardiovasc_death_rate: Some(124.2),
            life_expectancy: 77.3,
            human_development_index: Some(0.7),
        },
    );

    alpha3.insert(
        "Comoros".to_string(),
        CountyStatistic {
            iso_code: "COM".to_string(),
            country_name: "Comoros".to_string(),
            continent: "Africa".to_string(),
            population: 869595,
            population_density: Some(437.4),
            median_age: Some(20.4),
            aged_65_older: Some(3.0),
            aged_70_older: Some(1.7),
            gdp_per_capita: Some(1413.9),
            diabetes_prevalence: Some(11.9),
            cardiovasc_death_rate: Some(261.5),
            life_expectancy: 64.3,
            human_development_index: Some(0.5),
        },
    );

    alpha3.insert(
        "Democratic Republic of Congo".to_string(),
        CountyStatistic {
            iso_code: "COD".to_string(),
            country_name: "Democratic Republic of Congo".to_string(),
            continent: "Africa".to_string(),
            population: 89561404,
            population_density: Some(35.9),
            median_age: Some(17.0),
            aged_65_older: Some(3.0),
            aged_70_older: Some(1.7),
            gdp_per_capita: Some(808.1),
            diabetes_prevalence: Some(6.1),
            cardiovasc_death_rate: Some(318.9),
            life_expectancy: 60.7,
            human_development_index: Some(0.5),
        },
    );

    alpha3.insert(
        "Congo".to_string(),
        CountyStatistic {
            iso_code: "COG".to_string(),
            country_name: "Congo".to_string(),
            continent: "Africa".to_string(),
            population: 5518092,
            population_density: Some(15.4),
            median_age: Some(19.0),
            aged_65_older: Some(3.4),
            aged_70_older: Some(2.1),
            gdp_per_capita: Some(4881.4),
            diabetes_prevalence: Some(7.2),
            cardiovasc_death_rate: Some(344.1),
            life_expectancy: 64.6,
            human_development_index: Some(0.6),
        },
    );

    alpha3.insert(
        "Costa Rica".to_string(),
        CountyStatistic {
            iso_code: "CRI".to_string(),
            country_name: "Costa Rica".to_string(),
            continent: "North America".to_string(),
            population: 5094114,
            population_density: Some(96.1),
            median_age: Some(33.6),
            aged_65_older: Some(9.5),
            aged_70_older: Some(5.7),
            gdp_per_capita: Some(15525.0),
            diabetes_prevalence: Some(8.8),
            cardiovasc_death_rate: Some(138.0),
            life_expectancy: 80.3,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Croatia".to_string(),
        CountyStatistic {
            iso_code: "HRV".to_string(),
            country_name: "Croatia".to_string(),
            continent: "Europe".to_string(),
            population: 4105268,
            population_density: Some(73.7),
            median_age: Some(44.0),
            aged_65_older: Some(19.7),
            aged_70_older: Some(13.1),
            gdp_per_capita: Some(22669.8),
            diabetes_prevalence: Some(5.6),
            cardiovasc_death_rate: Some(253.8),
            life_expectancy: 78.5,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Cuba".to_string(),
        CountyStatistic {
            iso_code: "CUB".to_string(),
            country_name: "Cuba".to_string(),
            continent: "North America".to_string(),
            population: 11326616,
            population_density: Some(110.4),
            median_age: Some(43.1),
            aged_65_older: Some(14.7),
            aged_70_older: Some(9.7),
            gdp_per_capita: None,
            diabetes_prevalence: Some(8.3),
            cardiovasc_death_rate: Some(191.0),
            life_expectancy: 78.8,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Cyprus".to_string(),
        CountyStatistic {
            iso_code: "CYP".to_string(),
            country_name: "Cyprus".to_string(),
            continent: "Europe".to_string(),
            population: 875899,
            population_density: Some(127.7),
            median_age: Some(37.3),
            aged_65_older: Some(13.4),
            aged_70_older: Some(8.6),
            gdp_per_capita: Some(32415.1),
            diabetes_prevalence: Some(9.2),
            cardiovasc_death_rate: Some(141.2),
            life_expectancy: 81.0,
            human_development_index: Some(0.9),
        },
    );

    alpha3.insert(
        "Czechia".to_string(),
        CountyStatistic {
            iso_code: "CZE".to_string(),
            country_name: "Czechia".to_string(),
            continent: "Europe".to_string(),
            population: 10708982,
            population_density: Some(137.2),
            median_age: Some(43.3),
            aged_65_older: Some(19.0),
            aged_70_older: Some(11.6),
            gdp_per_capita: Some(32605.9),
            diabetes_prevalence: Some(6.8),
            cardiovasc_death_rate: Some(227.5),
            life_expectancy: 79.4,
            human_development_index: Some(0.9),
        },
    );

    alpha3.insert(
        "Cote d'Ivoire".to_string(),
        CountyStatistic {
            iso_code: "CIV".to_string(),
            country_name: "Cote d'Ivoire".to_string(),
            continent: "Africa".to_string(),
            population: 26378275,
            population_density: Some(76.4),
            median_age: Some(18.7),
            aged_65_older: Some(2.9),
            aged_70_older: Some(1.6),
            gdp_per_capita: Some(3601.0),
            diabetes_prevalence: Some(2.4),
            cardiovasc_death_rate: Some(303.7),
            life_expectancy: 57.8,
            human_development_index: Some(0.5),
        },
    );

    alpha3.insert(
        "Denmark".to_string(),
        CountyStatistic {
            iso_code: "DNK".to_string(),
            country_name: "Denmark".to_string(),
            continent: "Europe".to_string(),
            population: 5792203,
            population_density: Some(136.5),
            median_age: Some(42.3),
            aged_65_older: Some(19.7),
            aged_70_older: Some(12.3),
            gdp_per_capita: Some(46682.5),
            diabetes_prevalence: Some(6.4),
            cardiovasc_death_rate: Some(114.8),
            life_expectancy: 80.9,
            human_development_index: Some(0.9),
        },
    );

    alpha3.insert(
        "Djibouti".to_string(),
        CountyStatistic {
            iso_code: "DJI".to_string(),
            country_name: "Djibouti".to_string(),
            continent: "Africa".to_string(),
            population: 988002,
            population_density: Some(41.3),
            median_age: Some(25.4),
            aged_65_older: Some(4.2),
            aged_70_older: Some(2.4),
            gdp_per_capita: Some(2705.4),
            diabetes_prevalence: Some(6.0),
            cardiovasc_death_rate: Some(258.0),
            life_expectancy: 67.1,
            human_development_index: Some(0.5),
        },
    );

    alpha3.insert(
        "Dominica".to_string(),
        CountyStatistic {
            iso_code: "DMA".to_string(),
            country_name: "Dominica".to_string(),
            continent: "North America".to_string(),
            population: 71991,
            population_density: Some(98.6),
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: Some(9673.4),
            diabetes_prevalence: Some(11.6),
            cardiovasc_death_rate: Some(227.4),
            life_expectancy: 75.0,
            human_development_index: Some(0.7),
        },
    );

    alpha3.insert(
        "Dominican Republic".to_string(),
        CountyStatistic {
            iso_code: "DOM".to_string(),
            country_name: "Dominican Republic".to_string(),
            continent: "North America".to_string(),
            population: 10847904,
            population_density: Some(222.9),
            median_age: Some(27.6),
            aged_65_older: Some(7.0),
            aged_70_older: Some(4.4),
            gdp_per_capita: Some(14600.9),
            diabetes_prevalence: Some(8.2),
            cardiovasc_death_rate: Some(266.7),
            life_expectancy: 74.1,
            human_development_index: Some(0.7),
        },
    );

    alpha3.insert(
        "Ecuador".to_string(),
        CountyStatistic {
            iso_code: "ECU".to_string(),
            country_name: "Ecuador".to_string(),
            continent: "South America".to_string(),
            population: 17643060,
            population_density: Some(66.9),
            median_age: Some(28.1),
            aged_65_older: Some(7.1),
            aged_70_older: Some(4.5),
            gdp_per_capita: Some(10581.9),
            diabetes_prevalence: Some(5.5),
            cardiovasc_death_rate: Some(140.4),
            life_expectancy: 77.0,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Egypt".to_string(),
        CountyStatistic {
            iso_code: "EGY".to_string(),
            country_name: "Egypt".to_string(),
            continent: "Africa".to_string(),
            population: 102334403,
            population_density: Some(98.0),
            median_age: Some(25.3),
            aged_65_older: Some(5.2),
            aged_70_older: Some(2.9),
            gdp_per_capita: Some(10550.2),
            diabetes_prevalence: Some(17.3),
            cardiovasc_death_rate: Some(525.4),
            life_expectancy: 72.0,
            human_development_index: Some(0.7),
        },
    );

    alpha3.insert(
        "El Salvador".to_string(),
        CountyStatistic {
            iso_code: "SLV".to_string(),
            country_name: "El Salvador".to_string(),
            continent: "North America".to_string(),
            population: 6486201,
            population_density: Some(307.8),
            median_age: Some(27.6),
            aged_65_older: Some(8.3),
            aged_70_older: Some(5.4),
            gdp_per_capita: Some(7292.5),
            diabetes_prevalence: Some(8.9),
            cardiovasc_death_rate: Some(167.3),
            life_expectancy: 73.3,
            human_development_index: Some(0.7),
        },
    );

    alpha3.insert(
        "Equatorial Guinea".to_string(),
        CountyStatistic {
            iso_code: "GNQ".to_string(),
            country_name: "Equatorial Guinea".to_string(),
            continent: "Africa".to_string(),
            population: 1402985,
            population_density: Some(45.2),
            median_age: Some(22.4),
            aged_65_older: Some(2.8),
            aged_70_older: Some(1.8),
            gdp_per_capita: Some(22604.9),
            diabetes_prevalence: Some(7.8),
            cardiovasc_death_rate: Some(202.8),
            life_expectancy: 58.7,
            human_development_index: Some(0.6),
        },
    );

    alpha3.insert(
        "Eritrea".to_string(),
        CountyStatistic {
            iso_code: "ERI".to_string(),
            country_name: "Eritrea".to_string(),
            continent: "Africa".to_string(),
            population: 3546427,
            population_density: Some(44.3),
            median_age: Some(19.3),
            aged_65_older: Some(3.6),
            aged_70_older: Some(2.2),
            gdp_per_capita: Some(1510.5),
            diabetes_prevalence: Some(6.0),
            cardiovasc_death_rate: Some(311.1),
            life_expectancy: 66.3,
            human_development_index: Some(0.4),
        },
    );

    alpha3.insert(
        "Estonia".to_string(),
        CountyStatistic {
            iso_code: "EST".to_string(),
            country_name: "Estonia".to_string(),
            continent: "Europe".to_string(),
            population: 1326539,
            population_density: Some(31.0),
            median_age: Some(42.7),
            aged_65_older: Some(19.5),
            aged_70_older: Some(13.5),
            gdp_per_capita: Some(29481.3),
            diabetes_prevalence: Some(4.0),
            cardiovasc_death_rate: Some(255.6),
            life_expectancy: 78.7,
            human_development_index: Some(0.9),
        },
    );

    alpha3.insert(
        "Eswatini".to_string(),
        CountyStatistic {
            iso_code: "SWZ".to_string(),
            country_name: "Eswatini".to_string(),
            continent: "Africa".to_string(),
            population: 1160164,
            population_density: Some(79.5),
            median_age: Some(21.5),
            aged_65_older: Some(3.2),
            aged_70_older: Some(1.8),
            gdp_per_capita: Some(7739.0),
            diabetes_prevalence: Some(3.9),
            cardiovasc_death_rate: Some(333.4),
            life_expectancy: 60.2,
            human_development_index: Some(0.6),
        },
    );

    alpha3.insert(
        "Ethiopia".to_string(),
        CountyStatistic {
            iso_code: "ETH".to_string(),
            country_name: "Ethiopia".to_string(),
            continent: "Africa".to_string(),
            population: 114963583,
            population_density: Some(105.0),
            median_age: Some(19.8),
            aged_65_older: Some(3.5),
            aged_70_older: Some(2.1),
            gdp_per_capita: Some(1729.9),
            diabetes_prevalence: Some(7.5),
            cardiovasc_death_rate: Some(182.6),
            life_expectancy: 66.6,
            human_development_index: Some(0.5),
        },
    );

    alpha3.insert(
        "Fiji".to_string(),
        CountyStatistic {
            iso_code: "FJI".to_string(),
            country_name: "Fiji".to_string(),
            continent: "Oceania".to_string(),
            population: 896444,
            population_density: Some(49.6),
            median_age: Some(28.6),
            aged_65_older: Some(6.2),
            aged_70_older: Some(3.3),
            gdp_per_capita: Some(8703.0),
            diabetes_prevalence: Some(14.5),
            cardiovasc_death_rate: Some(412.8),
            life_expectancy: 67.4,
            human_development_index: Some(0.7),
        },
    );

    alpha3.insert(
        "Finland".to_string(),
        CountyStatistic {
            iso_code: "FIN".to_string(),
            country_name: "Finland".to_string(),
            continent: "Europe".to_string(),
            population: 5540718,
            population_density: Some(18.1),
            median_age: Some(42.8),
            aged_65_older: Some(21.2),
            aged_70_older: Some(13.3),
            gdp_per_capita: Some(40585.7),
            diabetes_prevalence: Some(5.8),
            cardiovasc_death_rate: Some(153.5),
            life_expectancy: 81.9,
            human_development_index: Some(0.9),
        },
    );

    alpha3.insert(
        "France".to_string(),
        CountyStatistic {
            iso_code: "FRA".to_string(),
            country_name: "France".to_string(),
            continent: "Europe".to_string(),
            population: 65273512,
            population_density: Some(122.6),
            median_age: Some(42.0),
            aged_65_older: Some(19.7),
            aged_70_older: Some(13.1),
            gdp_per_capita: Some(38605.7),
            diabetes_prevalence: Some(4.8),
            cardiovasc_death_rate: Some(86.1),
            life_expectancy: 82.7,
            human_development_index: Some(0.9),
        },
    );

    alpha3.insert(
        "Gabon".to_string(),
        CountyStatistic {
            iso_code: "GAB".to_string(),
            country_name: "Gabon".to_string(),
            continent: "Africa".to_string(),
            population: 2225728,
            population_density: Some(7.9),
            median_age: Some(23.1),
            aged_65_older: Some(4.5),
            aged_70_older: Some(3.0),
            gdp_per_capita: Some(16562.4),
            diabetes_prevalence: Some(7.2),
            cardiovasc_death_rate: Some(260.0),
            life_expectancy: 66.5,
            human_development_index: Some(0.7),
        },
    );

    alpha3.insert(
        "Gambia".to_string(),
        CountyStatistic {
            iso_code: "GMB".to_string(),
            country_name: "Gambia".to_string(),
            continent: "Africa".to_string(),
            population: 2416664,
            population_density: Some(207.6),
            median_age: Some(17.5),
            aged_65_older: Some(2.3),
            aged_70_older: Some(1.4),
            gdp_per_capita: Some(1561.8),
            diabetes_prevalence: Some(1.9),
            cardiovasc_death_rate: Some(331.4),
            life_expectancy: 62.0,
            human_development_index: Some(0.5),
        },
    );

    alpha3.insert(
        "Georgia".to_string(),
        CountyStatistic {
            iso_code: "GEO".to_string(),
            country_name: "Georgia".to_string(),
            continent: "Asia".to_string(),
            population: 3989175,
            population_density: Some(65.0),
            median_age: Some(38.7),
            aged_65_older: Some(14.9),
            aged_70_older: Some(10.2),
            gdp_per_capita: Some(9745.1),
            diabetes_prevalence: Some(7.1),
            cardiovasc_death_rate: Some(496.2),
            life_expectancy: 73.8,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Germany".to_string(),
        CountyStatistic {
            iso_code: "DEU".to_string(),
            country_name: "Germany".to_string(),
            continent: "Europe".to_string(),
            population: 83783945,
            population_density: Some(237.0),
            median_age: Some(46.6),
            aged_65_older: Some(21.5),
            aged_70_older: Some(16.0),
            gdp_per_capita: Some(45229.2),
            diabetes_prevalence: Some(8.3),
            cardiovasc_death_rate: Some(156.1),
            life_expectancy: 81.3,
            human_development_index: Some(0.9),
        },
    );

    alpha3.insert(
        "Ghana".to_string(),
        CountyStatistic {
            iso_code: "GHA".to_string(),
            country_name: "Ghana".to_string(),
            continent: "Africa".to_string(),
            population: 31072945,
            population_density: Some(126.7),
            median_age: Some(21.1),
            aged_65_older: Some(3.4),
            aged_70_older: Some(1.9),
            gdp_per_capita: Some(4227.6),
            diabetes_prevalence: Some(5.0),
            cardiovasc_death_rate: Some(298.2),
            life_expectancy: 64.1,
            human_development_index: Some(0.6),
        },
    );

    alpha3.insert(
        "Greece".to_string(),
        CountyStatistic {
            iso_code: "GRC".to_string(),
            country_name: "Greece".to_string(),
            continent: "Europe".to_string(),
            population: 10423056,
            population_density: Some(83.5),
            median_age: Some(45.3),
            aged_65_older: Some(20.4),
            aged_70_older: Some(14.5),
            gdp_per_capita: Some(24574.4),
            diabetes_prevalence: Some(4.5),
            cardiovasc_death_rate: Some(175.7),
            life_expectancy: 82.2,
            human_development_index: Some(0.9),
        },
    );

    alpha3.insert(
        "Grenada".to_string(),
        CountyStatistic {
            iso_code: "GRD".to_string(),
            country_name: "Grenada".to_string(),
            continent: "North America".to_string(),
            population: 112519,
            population_density: Some(317.1),
            median_age: Some(29.4),
            aged_65_older: Some(7.3),
            aged_70_older: Some(5.0),
            gdp_per_capita: Some(13593.9),
            diabetes_prevalence: Some(10.7),
            cardiovasc_death_rate: Some(244.0),
            life_expectancy: 72.4,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Guatemala".to_string(),
        CountyStatistic {
            iso_code: "GTM".to_string(),
            country_name: "Guatemala".to_string(),
            continent: "North America".to_string(),
            population: 17915567,
            population_density: Some(157.8),
            median_age: Some(22.9),
            aged_65_older: Some(4.7),
            aged_70_older: Some(3.0),
            gdp_per_capita: Some(7423.8),
            diabetes_prevalence: Some(10.2),
            cardiovasc_death_rate: Some(155.9),
            life_expectancy: 74.3,
            human_development_index: Some(0.7),
        },
    );

    alpha3.insert(
        "Guinea".to_string(),
        CountyStatistic {
            iso_code: "GIN".to_string(),
            country_name: "Guinea".to_string(),
            continent: "Africa".to_string(),
            population: 13132792,
            population_density: Some(51.8),
            median_age: Some(19.0),
            aged_65_older: Some(3.1),
            aged_70_older: Some(1.7),
            gdp_per_capita: Some(1998.9),
            diabetes_prevalence: Some(2.4),
            cardiovasc_death_rate: Some(336.7),
            life_expectancy: 61.6,
            human_development_index: Some(0.5),
        },
    );

    alpha3.insert(
        "Guinea-Bissau".to_string(),
        CountyStatistic {
            iso_code: "GNB".to_string(),
            country_name: "Guinea-Bissau".to_string(),
            continent: "Africa".to_string(),
            population: 1967998,
            population_density: Some(66.2),
            median_age: Some(19.4),
            aged_65_older: Some(3.0),
            aged_70_older: Some(1.6),
            gdp_per_capita: Some(1548.7),
            diabetes_prevalence: Some(2.4),
            cardiovasc_death_rate: Some(382.5),
            life_expectancy: 58.3,
            human_development_index: Some(0.5),
        },
    );

    alpha3.insert(
        "Guyana".to_string(),
        CountyStatistic {
            iso_code: "GUY".to_string(),
            country_name: "Guyana".to_string(),
            continent: "South America".to_string(),
            population: 786559,
            population_density: Some(4.0),
            median_age: Some(26.3),
            aged_65_older: Some(5.3),
            aged_70_older: Some(2.8),
            gdp_per_capita: Some(7435.0),
            diabetes_prevalence: Some(11.6),
            cardiovasc_death_rate: Some(373.2),
            life_expectancy: 69.9,
            human_development_index: Some(0.7),
        },
    );

    alpha3.insert(
        "Haiti".to_string(),
        CountyStatistic {
            iso_code: "HTI".to_string(),
            country_name: "Haiti".to_string(),
            continent: "North America".to_string(),
            population: 11402533,
            population_density: Some(398.4),
            median_age: Some(24.3),
            aged_65_older: Some(4.8),
            aged_70_older: Some(3.0),
            gdp_per_capita: Some(1653.2),
            diabetes_prevalence: Some(6.7),
            cardiovasc_death_rate: Some(430.5),
            life_expectancy: 64.0,
            human_development_index: Some(0.5),
        },
    );

    alpha3.insert(
        "Holy See".to_string(),
        CountyStatistic {
            iso_code: "VAT".to_string(),
            country_name: "Holy See".to_string(),
            continent: "Europe".to_string(),
            population: 809,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 75.1,
            human_development_index: None,
        },
    );

    alpha3.insert(
        "Honduras".to_string(),
        CountyStatistic {
            iso_code: "HND".to_string(),
            country_name: "Honduras".to_string(),
            continent: "North America".to_string(),
            population: 9904608,
            population_density: Some(82.8),
            median_age: Some(24.9),
            aged_65_older: Some(4.7),
            aged_70_older: Some(2.9),
            gdp_per_capita: Some(4541.8),
            diabetes_prevalence: Some(7.2),
            cardiovasc_death_rate: Some(240.2),
            life_expectancy: 75.3,
            human_development_index: Some(0.6),
        },
    );

    alpha3.insert(
        "Hong Kong".to_string(),
        CountyStatistic {
            iso_code: "HKG".to_string(),
            country_name: "Hong Kong".to_string(),
            continent: "Asia".to_string(),
            population: 7496988,
            population_density: Some(7039.7),
            median_age: Some(44.8),
            aged_65_older: Some(16.3),
            aged_70_older: Some(10.2),
            gdp_per_capita: Some(56054.9),
            diabetes_prevalence: Some(8.3),
            cardiovasc_death_rate: None,
            life_expectancy: 84.9,
            human_development_index: Some(0.9),
        },
    );

    alpha3.insert(
        "Hungary".to_string(),
        CountyStatistic {
            iso_code: "HUN".to_string(),
            country_name: "Hungary".to_string(),
            continent: "Europe".to_string(),
            population: 9660350,
            population_density: Some(108.0),
            median_age: Some(43.4),
            aged_65_older: Some(18.6),
            aged_70_older: Some(12.0),
            gdp_per_capita: Some(26777.6),
            diabetes_prevalence: Some(7.5),
            cardiovasc_death_rate: Some(278.3),
            life_expectancy: 76.9,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Iceland".to_string(),
        CountyStatistic {
            iso_code: "ISL".to_string(),
            country_name: "Iceland".to_string(),
            continent: "Europe".to_string(),
            population: 341250,
            population_density: Some(3.4),
            median_age: Some(37.3),
            aged_65_older: Some(14.4),
            aged_70_older: Some(9.2),
            gdp_per_capita: Some(46483.0),
            diabetes_prevalence: Some(5.3),
            cardiovasc_death_rate: Some(118.0),
            life_expectancy: 83.0,
            human_development_index: Some(0.9),
        },
    );

    alpha3.insert(
        "India".to_string(),
        CountyStatistic {
            iso_code: "IND".to_string(),
            country_name: "India".to_string(),
            continent: "Asia".to_string(),
            population: 1380004385,
            population_density: Some(450.4),
            median_age: Some(28.2),
            aged_65_older: Some(6.0),
            aged_70_older: Some(3.4),
            gdp_per_capita: Some(6426.7),
            diabetes_prevalence: Some(10.4),
            cardiovasc_death_rate: Some(282.3),
            life_expectancy: 69.7,
            human_development_index: Some(0.6),
        },
    );

    alpha3.insert(
        "Indonesia".to_string(),
        CountyStatistic {
            iso_code: "IDN".to_string(),
            country_name: "Indonesia".to_string(),
            continent: "Asia".to_string(),
            population: 273523621,
            population_density: Some(145.7),
            median_age: Some(29.3),
            aged_65_older: Some(5.3),
            aged_70_older: Some(3.1),
            gdp_per_capita: Some(11188.7),
            diabetes_prevalence: Some(6.3),
            cardiovasc_death_rate: Some(342.9),
            life_expectancy: 71.7,
            human_development_index: Some(0.7),
        },
    );

    alpha3.insert(
        "Iran".to_string(),
        CountyStatistic {
            iso_code: "IRN".to_string(),
            country_name: "Iran".to_string(),
            continent: "Asia".to_string(),
            population: 83992953,
            population_density: Some(49.8),
            median_age: Some(32.4),
            aged_65_older: Some(5.4),
            aged_70_older: Some(3.2),
            gdp_per_capita: Some(19082.6),
            diabetes_prevalence: Some(9.6),
            cardiovasc_death_rate: Some(270.3),
            life_expectancy: 76.7,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Iraq".to_string(),
        CountyStatistic {
            iso_code: "IRQ".to_string(),
            country_name: "Iraq".to_string(),
            continent: "Asia".to_string(),
            population: 40222503,
            population_density: Some(88.1),
            median_age: Some(20.0),
            aged_65_older: Some(3.2),
            aged_70_older: Some(2.0),
            gdp_per_capita: Some(15664.0),
            diabetes_prevalence: Some(8.8),
            cardiovasc_death_rate: Some(218.6),
            life_expectancy: 70.6,
            human_development_index: Some(0.7),
        },
    );

    alpha3.insert(
        "Ireland".to_string(),
        CountyStatistic {
            iso_code: "IRL".to_string(),
            country_name: "Ireland".to_string(),
            continent: "Europe".to_string(),
            population: 4937796,
            population_density: Some(69.9),
            median_age: Some(38.7),
            aged_65_older: Some(13.9),
            aged_70_older: Some(8.7),
            gdp_per_capita: Some(67335.3),
            diabetes_prevalence: Some(3.3),
            cardiovasc_death_rate: Some(126.5),
            life_expectancy: 82.3,
            human_development_index: Some(0.9),
        },
    );

    alpha3.insert(
        "Israel".to_string(),
        CountyStatistic {
            iso_code: "ISR".to_string(),
            country_name: "Israel".to_string(),
            continent: "Asia".to_string(),
            population: 8655541,
            population_density: Some(402.6),
            median_age: Some(30.6),
            aged_65_older: Some(11.7),
            aged_70_older: Some(7.4),
            gdp_per_capita: Some(33132.3),
            diabetes_prevalence: Some(6.7),
            cardiovasc_death_rate: Some(93.3),
            life_expectancy: 83.0,
            human_development_index: Some(0.9),
        },
    );

    alpha3.insert(
        "Italy".to_string(),
        CountyStatistic {
            iso_code: "ITA".to_string(),
            country_name: "Italy".to_string(),
            continent: "Europe".to_string(),
            population: 60461828,
            population_density: Some(205.9),
            median_age: Some(47.9),
            aged_65_older: Some(23.0),
            aged_70_older: Some(16.2),
            gdp_per_capita: Some(35220.1),
            diabetes_prevalence: Some(4.8),
            cardiovasc_death_rate: Some(113.2),
            life_expectancy: 83.5,
            human_development_index: Some(0.9),
        },
    );

    alpha3.insert(
        "Jamaica".to_string(),
        CountyStatistic {
            iso_code: "JAM".to_string(),
            country_name: "Jamaica".to_string(),
            continent: "North America".to_string(),
            population: 2961161,
            population_density: Some(266.9),
            median_age: Some(31.4),
            aged_65_older: Some(9.7),
            aged_70_older: Some(6.4),
            gdp_per_capita: Some(8193.6),
            diabetes_prevalence: Some(11.3),
            cardiovasc_death_rate: Some(206.5),
            life_expectancy: 74.5,
            human_development_index: Some(0.7),
        },
    );

    alpha3.insert(
        "Japan".to_string(),
        CountyStatistic {
            iso_code: "JPN".to_string(),
            country_name: "Japan".to_string(),
            continent: "Asia".to_string(),
            population: 126476458,
            population_density: Some(347.8),
            median_age: Some(48.2),
            aged_65_older: Some(27.0),
            aged_70_older: Some(18.5),
            gdp_per_capita: Some(39002.2),
            diabetes_prevalence: Some(5.7),
            cardiovasc_death_rate: Some(79.4),
            life_expectancy: 84.6,
            human_development_index: Some(0.9),
        },
    );

    alpha3.insert(
        "Jordan".to_string(),
        CountyStatistic {
            iso_code: "JOR".to_string(),
            country_name: "Jordan".to_string(),
            continent: "Asia".to_string(),
            population: 10203140,
            population_density: Some(109.3),
            median_age: Some(23.2),
            aged_65_older: Some(3.8),
            aged_70_older: Some(2.4),
            gdp_per_capita: Some(8337.5),
            diabetes_prevalence: Some(11.8),
            cardiovasc_death_rate: Some(208.3),
            life_expectancy: 74.5,
            human_development_index: Some(0.7),
        },
    );

    alpha3.insert(
        "Kazakhstan".to_string(),
        CountyStatistic {
            iso_code: "KAZ".to_string(),
            country_name: "Kazakhstan".to_string(),
            continent: "Asia".to_string(),
            population: 18776707,
            population_density: Some(6.7),
            median_age: Some(30.6),
            aged_65_older: Some(7.0),
            aged_70_older: Some(4.6),
            gdp_per_capita: Some(24055.6),
            diabetes_prevalence: Some(7.1),
            cardiovasc_death_rate: Some(466.8),
            life_expectancy: 73.6,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Kenya".to_string(),
        CountyStatistic {
            iso_code: "KEN".to_string(),
            country_name: "Kenya".to_string(),
            continent: "Africa".to_string(),
            population: 53771300,
            population_density: Some(87.3),
            median_age: Some(20.0),
            aged_65_older: Some(2.7),
            aged_70_older: Some(1.5),
            gdp_per_capita: Some(2993.0),
            diabetes_prevalence: Some(2.9),
            cardiovasc_death_rate: Some(218.6),
            life_expectancy: 66.7,
            human_development_index: Some(0.6),
        },
    );

    alpha3.insert(
        "South Korea".to_string(),
        CountyStatistic {
            iso_code: "KOR".to_string(),
            country_name: "South Korea".to_string(),
            continent: "Asia".to_string(),
            population: 51269183,
            population_density: Some(528.0),
            median_age: Some(43.4),
            aged_65_older: Some(13.9),
            aged_70_older: Some(8.6),
            gdp_per_capita: Some(35938.4),
            diabetes_prevalence: Some(6.8),
            cardiovasc_death_rate: Some(86.0),
            life_expectancy: 83.0,
            human_development_index: Some(0.9),
        },
    );

    alpha3.insert(
        "Kuwait".to_string(),
        CountyStatistic {
            iso_code: "KWT".to_string(),
            country_name: "Kuwait".to_string(),
            continent: "Asia".to_string(),
            population: 4270563,
            population_density: Some(232.1),
            median_age: Some(33.7),
            aged_65_older: Some(2.3),
            aged_70_older: Some(1.1),
            gdp_per_capita: Some(65530.5),
            diabetes_prevalence: Some(15.8),
            cardiovasc_death_rate: Some(132.2),
            life_expectancy: 75.5,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Kyrgyzstan".to_string(),
        CountyStatistic {
            iso_code: "KGZ".to_string(),
            country_name: "Kyrgyzstan".to_string(),
            continent: "Asia".to_string(),
            population: 6524191,
            population_density: Some(32.3),
            median_age: Some(26.3),
            aged_65_older: Some(4.5),
            aged_70_older: Some(2.9),
            gdp_per_capita: Some(3393.5),
            diabetes_prevalence: Some(7.1),
            cardiovasc_death_rate: Some(436.4),
            life_expectancy: 71.5,
            human_development_index: Some(0.7),
        },
    );

    alpha3.insert(
        "Laos".to_string(),
        CountyStatistic {
            iso_code: "LAO".to_string(),
            country_name: "Laos".to_string(),
            continent: "Asia".to_string(),
            population: 7275556,
            population_density: Some(29.7),
            median_age: Some(24.4),
            aged_65_older: Some(4.0),
            aged_70_older: Some(2.3),
            gdp_per_capita: Some(6397.4),
            diabetes_prevalence: Some(4.0),
            cardiovasc_death_rate: Some(368.1),
            life_expectancy: 67.9,
            human_development_index: Some(0.6),
        },
    );

    alpha3.insert(
        "Latvia".to_string(),
        CountyStatistic {
            iso_code: "LVA".to_string(),
            country_name: "Latvia".to_string(),
            continent: "Europe".to_string(),
            population: 1886202,
            population_density: Some(31.2),
            median_age: Some(43.9),
            aged_65_older: Some(19.8),
            aged_70_older: Some(14.1),
            gdp_per_capita: Some(25063.8),
            diabetes_prevalence: Some(4.9),
            cardiovasc_death_rate: Some(350.1),
            life_expectancy: 75.3,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Lebanon".to_string(),
        CountyStatistic {
            iso_code: "LBN".to_string(),
            country_name: "Lebanon".to_string(),
            continent: "Asia".to_string(),
            population: 6825442,
            population_density: Some(594.6),
            median_age: Some(31.1),
            aged_65_older: Some(8.5),
            aged_70_older: Some(5.4),
            gdp_per_capita: Some(13367.6),
            diabetes_prevalence: Some(12.7),
            cardiovasc_death_rate: Some(266.6),
            life_expectancy: 78.9,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Lesotho".to_string(),
        CountyStatistic {
            iso_code: "LSO".to_string(),
            country_name: "Lesotho".to_string(),
            continent: "Africa".to_string(),
            population: 2142252,
            population_density: Some(73.6),
            median_age: Some(22.2),
            aged_65_older: Some(4.5),
            aged_70_older: Some(2.6),
            gdp_per_capita: Some(2851.2),
            diabetes_prevalence: Some(3.9),
            cardiovasc_death_rate: Some(405.1),
            life_expectancy: 54.3,
            human_development_index: Some(0.5),
        },
    );

    alpha3.insert(
        "Liberia".to_string(),
        CountyStatistic {
            iso_code: "LBR".to_string(),
            country_name: "Liberia".to_string(),
            continent: "Africa".to_string(),
            population: 5057677,
            population_density: Some(49.1),
            median_age: Some(19.2),
            aged_65_older: Some(3.1),
            aged_70_older: Some(1.8),
            gdp_per_capita: Some(752.8),
            diabetes_prevalence: Some(2.4),
            cardiovasc_death_rate: Some(272.5),
            life_expectancy: 64.1,
            human_development_index: Some(0.4),
        },
    );

    alpha3.insert(
        "Libya".to_string(),
        CountyStatistic {
            iso_code: "LBY".to_string(),
            country_name: "Libya".to_string(),
            continent: "Africa".to_string(),
            population: 6871287,
            population_density: Some(3.6),
            median_age: Some(29.0),
            aged_65_older: Some(4.4),
            aged_70_older: Some(2.8),
            gdp_per_capita: Some(17881.5),
            diabetes_prevalence: Some(10.4),
            cardiovasc_death_rate: Some(341.9),
            life_expectancy: 72.9,
            human_development_index: Some(0.7),
        },
    );

    alpha3.insert(
        "Liechtenstein".to_string(),
        CountyStatistic {
            iso_code: "LIE".to_string(),
            country_name: "Liechtenstein".to_string(),
            continent: "Europe".to_string(),
            population: 38137,
            population_density: Some(237.0),
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: Some(7.8),
            cardiovasc_death_rate: None,
            life_expectancy: 82.5,
            human_development_index: Some(0.9),
        },
    );

    alpha3.insert(
        "Lithuania".to_string(),
        CountyStatistic {
            iso_code: "LTU".to_string(),
            country_name: "Lithuania".to_string(),
            continent: "Europe".to_string(),
            population: 2722291,
            population_density: Some(45.1),
            median_age: Some(43.5),
            aged_65_older: Some(19.0),
            aged_70_older: Some(13.8),
            gdp_per_capita: Some(29524.3),
            diabetes_prevalence: Some(3.7),
            cardiovasc_death_rate: Some(343.0),
            life_expectancy: 75.9,
            human_development_index: Some(0.9),
        },
    );

    alpha3.insert(
        "Luxembourg".to_string(),
        CountyStatistic {
            iso_code: "LUX".to_string(),
            country_name: "Luxembourg".to_string(),
            continent: "Europe".to_string(),
            population: 625976,
            population_density: Some(231.4),
            median_age: Some(39.7),
            aged_65_older: Some(14.3),
            aged_70_older: Some(9.8),
            gdp_per_capita: Some(94278.0),
            diabetes_prevalence: Some(4.4),
            cardiovasc_death_rate: Some(128.3),
            life_expectancy: 82.3,
            human_development_index: Some(0.9),
        },
    );

    alpha3.insert(
        "Madagascar".to_string(),
        CountyStatistic {
            iso_code: "MDG".to_string(),
            country_name: "Madagascar".to_string(),
            continent: "Africa".to_string(),
            population: 27691019,
            population_density: Some(44.0),
            median_age: Some(19.6),
            aged_65_older: Some(2.9),
            aged_70_older: Some(1.7),
            gdp_per_capita: Some(1416.4),
            diabetes_prevalence: Some(3.9),
            cardiovasc_death_rate: Some(406.0),
            life_expectancy: 67.0,
            human_development_index: Some(0.5),
        },
    );

    alpha3.insert(
        "Malawi".to_string(),
        CountyStatistic {
            iso_code: "MWI".to_string(),
            country_name: "Malawi".to_string(),
            continent: "Africa".to_string(),
            population: 19129955,
            population_density: Some(197.5),
            median_age: Some(18.1),
            aged_65_older: Some(3.0),
            aged_70_older: Some(1.8),
            gdp_per_capita: Some(1095.0),
            diabetes_prevalence: Some(3.9),
            cardiovasc_death_rate: Some(227.3),
            life_expectancy: 64.3,
            human_development_index: Some(0.5),
        },
    );

    alpha3.insert(
        "Malaysia".to_string(),
        CountyStatistic {
            iso_code: "MYS".to_string(),
            country_name: "Malaysia".to_string(),
            continent: "Asia".to_string(),
            population: 32365998,
            population_density: Some(96.3),
            median_age: Some(29.9),
            aged_65_older: Some(6.3),
            aged_70_older: Some(3.4),
            gdp_per_capita: Some(26808.2),
            diabetes_prevalence: Some(16.7),
            cardiovasc_death_rate: Some(260.9),
            life_expectancy: 76.2,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Maldives".to_string(),
        CountyStatistic {
            iso_code: "MDV".to_string(),
            country_name: "Maldives".to_string(),
            continent: "Asia".to_string(),
            population: 540542,
            population_density: Some(1454.4),
            median_age: Some(30.6),
            aged_65_older: Some(4.1),
            aged_70_older: Some(2.9),
            gdp_per_capita: Some(15183.6),
            diabetes_prevalence: Some(9.2),
            cardiovasc_death_rate: Some(164.9),
            life_expectancy: 78.9,
            human_development_index: Some(0.7),
        },
    );

    alpha3.insert(
        "Mali".to_string(),
        CountyStatistic {
            iso_code: "MLI".to_string(),
            country_name: "Mali".to_string(),
            continent: "Africa".to_string(),
            population: 20250834,
            population_density: Some(15.2),
            median_age: Some(16.4),
            aged_65_older: Some(2.5),
            aged_70_older: Some(1.5),
            gdp_per_capita: Some(2014.3),
            diabetes_prevalence: Some(2.4),
            cardiovasc_death_rate: Some(268.0),
            life_expectancy: 59.3,
            human_development_index: Some(0.4),
        },
    );

    alpha3.insert(
        "Malta".to_string(),
        CountyStatistic {
            iso_code: "MLT".to_string(),
            country_name: "Malta".to_string(),
            continent: "Europe".to_string(),
            population: 441539,
            population_density: Some(1454.0),
            median_age: Some(42.4),
            aged_65_older: Some(19.4),
            aged_70_older: Some(11.3),
            gdp_per_capita: Some(36513.3),
            diabetes_prevalence: Some(8.8),
            cardiovasc_death_rate: Some(168.7),
            life_expectancy: 82.5,
            human_development_index: Some(0.9),
        },
    );

    alpha3.insert(
        "Marshall Islands".to_string(),
        CountyStatistic {
            iso_code: "MHL".to_string(),
            country_name: "Marshall Islands".to_string(),
            continent: "Oceania".to_string(),
            population: 59194,
            population_density: Some(295.1),
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: Some(3819.2),
            diabetes_prevalence: Some(30.5),
            cardiovasc_death_rate: Some(557.8),
            life_expectancy: 73.7,
            human_development_index: Some(0.7),
        },
    );

    alpha3.insert(
        "Mauritania".to_string(),
        CountyStatistic {
            iso_code: "MRT".to_string(),
            country_name: "Mauritania".to_string(),
            continent: "Africa".to_string(),
            population: 4649660,
            population_density: Some(4.3),
            median_age: Some(20.3),
            aged_65_older: Some(3.1),
            aged_70_older: Some(1.8),
            gdp_per_capita: Some(3597.6),
            diabetes_prevalence: Some(2.4),
            cardiovasc_death_rate: Some(232.3),
            life_expectancy: 64.9,
            human_development_index: Some(0.5),
        },
    );

    alpha3.insert(
        "Mauritius".to_string(),
        CountyStatistic {
            iso_code: "MUS".to_string(),
            country_name: "Mauritius".to_string(),
            continent: "Africa".to_string(),
            population: 1271767,
            population_density: Some(623.0),
            median_age: Some(37.4),
            aged_65_older: Some(10.9),
            aged_70_older: Some(5.9),
            gdp_per_capita: Some(20292.7),
            diabetes_prevalence: Some(22.0),
            cardiovasc_death_rate: Some(224.6),
            life_expectancy: 75.0,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Mexico".to_string(),
        CountyStatistic {
            iso_code: "MEX".to_string(),
            country_name: "Mexico".to_string(),
            continent: "North America".to_string(),
            population: 128932753,
            population_density: Some(66.4),
            median_age: Some(29.3),
            aged_65_older: Some(6.9),
            aged_70_older: Some(4.3),
            gdp_per_capita: Some(17336.5),
            diabetes_prevalence: Some(13.1),
            cardiovasc_death_rate: Some(152.8),
            life_expectancy: 75.0,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Micronesia".to_string(),
        CountyStatistic {
            iso_code: "FSM".to_string(),
            country_name: "Micronesia".to_string(),
            continent: "Oceania".to_string(),
            population: 115021,
            population_density: Some(150.8),
            median_age: Some(23.0),
            aged_65_older: Some(4.8),
            aged_70_older: Some(2.4),
            gdp_per_capita: Some(3299.5),
            diabetes_prevalence: Some(12.0),
            cardiovasc_death_rate: Some(454.3),
            life_expectancy: 67.9,
            human_development_index: Some(0.6),
        },
    );

    alpha3.insert(
        "Moldova".to_string(),
        CountyStatistic {
            iso_code: "MDA".to_string(),
            country_name: "Moldova".to_string(),
            continent: "Europe".to_string(),
            population: 4033963,
            population_density: Some(123.7),
            median_age: Some(37.6),
            aged_65_older: Some(10.9),
            aged_70_older: Some(7.0),
            gdp_per_capita: Some(5190.0),
            diabetes_prevalence: Some(5.7),
            cardiovasc_death_rate: Some(408.5),
            life_expectancy: 71.9,
            human_development_index: Some(0.7),
        },
    );

    alpha3.insert(
        "Monaco".to_string(),
        CountyStatistic {
            iso_code: "MCO".to_string(),
            country_name: "Monaco".to_string(),
            continent: "Europe".to_string(),
            population: 39244,
            population_density: Some(19347.5),
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: Some(5.5),
            cardiovasc_death_rate: None,
            life_expectancy: 86.8,
            human_development_index: None,
        },
    );

    alpha3.insert(
        "Mongolia".to_string(),
        CountyStatistic {
            iso_code: "MNG".to_string(),
            country_name: "Mongolia".to_string(),
            continent: "Asia".to_string(),
            population: 3278292,
            population_density: Some(2.0),
            median_age: Some(28.6),
            aged_65_older: Some(4.0),
            aged_70_older: Some(2.4),
            gdp_per_capita: Some(11840.8),
            diabetes_prevalence: Some(4.8),
            cardiovasc_death_rate: Some(460.0),
            life_expectancy: 69.9,
            human_development_index: Some(0.7),
        },
    );

    alpha3.insert(
        "Montenegro".to_string(),
        CountyStatistic {
            iso_code: "MNE".to_string(),
            country_name: "Montenegro".to_string(),
            continent: "Europe".to_string(),
            population: 628062,
            population_density: Some(46.3),
            median_age: Some(39.1),
            aged_65_older: Some(14.8),
            aged_70_older: Some(9.4),
            gdp_per_capita: Some(16409.3),
            diabetes_prevalence: Some(10.1),
            cardiovasc_death_rate: Some(387.3),
            life_expectancy: 76.9,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Morocco".to_string(),
        CountyStatistic {
            iso_code: "MAR".to_string(),
            country_name: "Morocco".to_string(),
            continent: "Africa".to_string(),
            population: 36910558,
            population_density: Some(80.1),
            median_age: Some(29.6),
            aged_65_older: Some(6.8),
            aged_70_older: Some(4.2),
            gdp_per_capita: Some(7485.0),
            diabetes_prevalence: Some(7.1),
            cardiovasc_death_rate: Some(419.1),
            life_expectancy: 76.7,
            human_development_index: Some(0.7),
        },
    );

    alpha3.insert(
        "Mozambique".to_string(),
        CountyStatistic {
            iso_code: "MOZ".to_string(),
            country_name: "Mozambique".to_string(),
            continent: "Africa".to_string(),
            population: 31255435,
            population_density: Some(37.7),
            median_age: Some(17.7),
            aged_65_older: Some(3.2),
            aged_70_older: Some(1.9),
            gdp_per_capita: Some(1136.1),
            diabetes_prevalence: Some(3.3),
            cardiovasc_death_rate: Some(329.9),
            life_expectancy: 60.9,
            human_development_index: Some(0.4),
        },
    );

    alpha3.insert(
        "Myanmar".to_string(),
        CountyStatistic {
            iso_code: "MMR".to_string(),
            country_name: "Myanmar".to_string(),
            continent: "Asia".to_string(),
            population: 54409794,
            population_density: Some(81.7),
            median_age: Some(29.1),
            aged_65_older: Some(5.7),
            aged_70_older: Some(3.1),
            gdp_per_capita: Some(5591.6),
            diabetes_prevalence: Some(4.6),
            cardiovasc_death_rate: Some(202.1),
            life_expectancy: 67.1,
            human_development_index: Some(0.6),
        },
    );

    alpha3.insert(
        "Namibia".to_string(),
        CountyStatistic {
            iso_code: "NAM".to_string(),
            country_name: "Namibia".to_string(),
            continent: "Africa".to_string(),
            population: 2540916,
            population_density: Some(3.1),
            median_age: Some(22.0),
            aged_65_older: Some(3.6),
            aged_70_older: Some(2.1),
            gdp_per_capita: Some(9541.8),
            diabetes_prevalence: Some(3.9),
            cardiovasc_death_rate: Some(243.8),
            life_expectancy: 63.7,
            human_development_index: Some(0.6),
        },
    );

    alpha3.insert(
        "Nepal".to_string(),
        CountyStatistic {
            iso_code: "NPL".to_string(),
            country_name: "Nepal".to_string(),
            continent: "Asia".to_string(),
            population: 29136808,
            population_density: Some(204.4),
            median_age: Some(25.0),
            aged_65_older: Some(5.8),
            aged_70_older: Some(3.2),
            gdp_per_capita: Some(2442.8),
            diabetes_prevalence: Some(7.3),
            cardiovasc_death_rate: Some(260.8),
            life_expectancy: 70.8,
            human_development_index: Some(0.6),
        },
    );

    alpha3.insert(
        "Netherlands".to_string(),
        CountyStatistic {
            iso_code: "NLD".to_string(),
            country_name: "Netherlands".to_string(),
            continent: "Europe".to_string(),
            population: 17134873,
            population_density: Some(508.5),
            median_age: Some(43.2),
            aged_65_older: Some(18.8),
            aged_70_older: Some(11.9),
            gdp_per_capita: Some(48472.5),
            diabetes_prevalence: Some(5.3),
            cardiovasc_death_rate: Some(109.4),
            life_expectancy: 82.3,
            human_development_index: Some(0.9),
        },
    );

    alpha3.insert(
        "New Zealand".to_string(),
        CountyStatistic {
            iso_code: "NZL".to_string(),
            country_name: "New Zealand".to_string(),
            continent: "Oceania".to_string(),
            population: 4822233,
            population_density: Some(18.2),
            median_age: Some(37.9),
            aged_65_older: Some(15.3),
            aged_70_older: Some(9.7),
            gdp_per_capita: Some(36085.8),
            diabetes_prevalence: Some(8.1),
            cardiovasc_death_rate: Some(128.8),
            life_expectancy: 82.3,
            human_development_index: Some(0.9),
        },
    );

    alpha3.insert(
        "Nicaragua".to_string(),
        CountyStatistic {
            iso_code: "NIC".to_string(),
            country_name: "Nicaragua".to_string(),
            continent: "North America".to_string(),
            population: 6624554,
            population_density: Some(51.7),
            median_age: Some(27.3),
            aged_65_older: Some(5.4),
            aged_70_older: Some(3.5),
            gdp_per_capita: Some(5321.4),
            diabetes_prevalence: Some(11.5),
            cardiovasc_death_rate: Some(137.0),
            life_expectancy: 74.5,
            human_development_index: Some(0.7),
        },
    );

    alpha3.insert(
        "Niger".to_string(),
        CountyStatistic {
            iso_code: "NER".to_string(),
            country_name: "Niger".to_string(),
            continent: "Africa".to_string(),
            population: 24206636,
            population_density: Some(17.0),
            median_age: Some(15.1),
            aged_65_older: Some(2.6),
            aged_70_older: Some(1.4),
            gdp_per_capita: Some(926.0),
            diabetes_prevalence: Some(2.4),
            cardiovasc_death_rate: Some(238.3),
            life_expectancy: 62.4,
            human_development_index: Some(0.4),
        },
    );

    alpha3.insert(
        "Nigeria".to_string(),
        CountyStatistic {
            iso_code: "NGA".to_string(),
            country_name: "Nigeria".to_string(),
            continent: "Africa".to_string(),
            population: 206139587,
            population_density: Some(209.6),
            median_age: Some(18.1),
            aged_65_older: Some(2.8),
            aged_70_older: Some(1.4),
            gdp_per_capita: Some(5338.5),
            diabetes_prevalence: Some(2.4),
            cardiovasc_death_rate: Some(181.0),
            life_expectancy: 54.7,
            human_development_index: Some(0.5),
        },
    );

    alpha3.insert(
        "Norway".to_string(),
        CountyStatistic {
            iso_code: "NOR".to_string(),
            country_name: "Norway".to_string(),
            continent: "Europe".to_string(),
            population: 5421242,
            population_density: Some(14.5),
            median_age: Some(39.7),
            aged_65_older: Some(16.8),
            aged_70_older: Some(10.8),
            gdp_per_capita: Some(64800.1),
            diabetes_prevalence: Some(5.3),
            cardiovasc_death_rate: Some(114.3),
            life_expectancy: 82.4,
            human_development_index: Some(1.0),
        },
    );

    alpha3.insert(
        "Oman".to_string(),
        CountyStatistic {
            iso_code: "OMN".to_string(),
            country_name: "Oman".to_string(),
            continent: "Asia".to_string(),
            population: 5106622,
            population_density: Some(15.0),
            median_age: Some(30.7),
            aged_65_older: Some(2.4),
            aged_70_older: Some(1.5),
            gdp_per_capita: Some(37960.7),
            diabetes_prevalence: Some(12.6),
            cardiovasc_death_rate: Some(266.3),
            life_expectancy: 77.9,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Pakistan".to_string(),
        CountyStatistic {
            iso_code: "PAK".to_string(),
            country_name: "Pakistan".to_string(),
            continent: "Asia".to_string(),
            population: 220892331,
            population_density: Some(255.6),
            median_age: Some(23.5),
            aged_65_older: Some(4.5),
            aged_70_older: Some(2.8),
            gdp_per_capita: Some(5034.7),
            diabetes_prevalence: Some(8.3),
            cardiovasc_death_rate: Some(423.0),
            life_expectancy: 67.3,
            human_development_index: Some(0.6),
        },
    );

    alpha3.insert(
        "Palestine".to_string(),
        CountyStatistic {
            iso_code: "PSE".to_string(),
            country_name: "Palestine".to_string(),
            continent: "Asia".to_string(),
            population: 5101416,
            population_density: Some(778.2),
            median_age: Some(20.4),
            aged_65_older: Some(3.0),
            aged_70_older: Some(1.7),
            gdp_per_capita: Some(4449.9),
            diabetes_prevalence: Some(10.6),
            cardiovasc_death_rate: Some(265.9),
            life_expectancy: 74.0,
            human_development_index: Some(0.7),
        },
    );

    alpha3.insert(
        "Panama".to_string(),
        CountyStatistic {
            iso_code: "PAN".to_string(),
            country_name: "Panama".to_string(),
            continent: "North America".to_string(),
            population: 4314768,
            population_density: Some(55.1),
            median_age: Some(29.7),
            aged_65_older: Some(7.9),
            aged_70_older: Some(5.0),
            gdp_per_capita: Some(22267.0),
            diabetes_prevalence: Some(8.3),
            cardiovasc_death_rate: Some(128.3),
            life_expectancy: 78.5,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Papua New Guinea".to_string(),
        CountyStatistic {
            iso_code: "PNG".to_string(),
            country_name: "Papua New Guinea".to_string(),
            continent: "Oceania".to_string(),
            population: 8947027,
            population_density: Some(18.2),
            median_age: Some(22.6),
            aged_65_older: Some(3.8),
            aged_70_older: Some(2.1),
            gdp_per_capita: Some(3823.2),
            diabetes_prevalence: Some(17.6),
            cardiovasc_death_rate: Some(561.5),
            life_expectancy: 64.5,
            human_development_index: Some(0.5),
        },
    );

    alpha3.insert(
        "Paraguay".to_string(),
        CountyStatistic {
            iso_code: "PRY".to_string(),
            country_name: "Paraguay".to_string(),
            continent: "South America".to_string(),
            population: 7132530,
            population_density: Some(17.1),
            median_age: Some(26.5),
            aged_65_older: Some(6.4),
            aged_70_older: Some(3.8),
            gdp_per_capita: Some(8827.0),
            diabetes_prevalence: Some(8.3),
            cardiovasc_death_rate: Some(199.1),
            life_expectancy: 74.3,
            human_development_index: Some(0.7),
        },
    );

    alpha3.insert(
        "Peru".to_string(),
        CountyStatistic {
            iso_code: "PER".to_string(),
            country_name: "Peru".to_string(),
            continent: "South America".to_string(),
            population: 32971846,
            population_density: Some(25.1),
            median_age: Some(29.1),
            aged_65_older: Some(7.2),
            aged_70_older: Some(4.5),
            gdp_per_capita: Some(12236.7),
            diabetes_prevalence: Some(6.0),
            cardiovasc_death_rate: Some(85.8),
            life_expectancy: 76.7,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Philippines".to_string(),
        CountyStatistic {
            iso_code: "PHL".to_string(),
            country_name: "Philippines".to_string(),
            continent: "Asia".to_string(),
            population: 109581085,
            population_density: Some(351.9),
            median_age: Some(25.2),
            aged_65_older: Some(4.8),
            aged_70_older: Some(2.7),
            gdp_per_capita: Some(7599.2),
            diabetes_prevalence: Some(7.1),
            cardiovasc_death_rate: Some(370.4),
            life_expectancy: 71.2,
            human_development_index: Some(0.7),
        },
    );

    alpha3.insert(
        "Poland".to_string(),
        CountyStatistic {
            iso_code: "POL".to_string(),
            country_name: "Poland".to_string(),
            continent: "Europe".to_string(),
            population: 37846605,
            population_density: Some(124.0),
            median_age: Some(41.8),
            aged_65_older: Some(16.8),
            aged_70_older: Some(10.2),
            gdp_per_capita: Some(27216.4),
            diabetes_prevalence: Some(5.9),
            cardiovasc_death_rate: Some(227.3),
            life_expectancy: 78.7,
            human_development_index: Some(0.9),
        },
    );

    alpha3.insert(
        "Portugal".to_string(),
        CountyStatistic {
            iso_code: "PRT".to_string(),
            country_name: "Portugal".to_string(),
            continent: "Europe".to_string(),
            population: 10196707,
            population_density: Some(112.4),
            median_age: Some(46.2),
            aged_65_older: Some(21.5),
            aged_70_older: Some(14.9),
            gdp_per_capita: Some(27936.9),
            diabetes_prevalence: Some(9.8),
            cardiovasc_death_rate: Some(127.8),
            life_expectancy: 82.0,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Qatar".to_string(),
        CountyStatistic {
            iso_code: "QAT".to_string(),
            country_name: "Qatar".to_string(),
            continent: "Asia".to_string(),
            population: 2881060,
            population_density: Some(227.3),
            median_age: Some(31.9),
            aged_65_older: Some(1.3),
            aged_70_older: Some(0.6),
            gdp_per_capita: Some(116935.6),
            diabetes_prevalence: Some(16.5),
            cardiovasc_death_rate: Some(176.7),
            life_expectancy: 80.2,
            human_development_index: Some(0.9),
        },
    );

    alpha3.insert(
        "North Macedonia".to_string(),
        CountyStatistic {
            iso_code: "MKD".to_string(),
            country_name: "North Macedonia".to_string(),
            continent: "Europe".to_string(),
            population: 2083380,
            population_density: Some(82.6),
            median_age: Some(39.1),
            aged_65_older: Some(13.3),
            aged_70_older: Some(8.2),
            gdp_per_capita: Some(13111.2),
            diabetes_prevalence: Some(10.1),
            cardiovasc_death_rate: Some(322.7),
            life_expectancy: 75.8,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Romania".to_string(),
        CountyStatistic {
            iso_code: "ROU".to_string(),
            country_name: "Romania".to_string(),
            continent: "Europe".to_string(),
            population: 19237682,
            population_density: Some(85.1),
            median_age: Some(43.0),
            aged_65_older: Some(17.9),
            aged_70_older: Some(11.7),
            gdp_per_capita: Some(23313.2),
            diabetes_prevalence: Some(9.7),
            cardiovasc_death_rate: Some(370.9),
            life_expectancy: 76.0,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Russia".to_string(),
        CountyStatistic {
            iso_code: "RUS".to_string(),
            country_name: "Russia".to_string(),
            continent: "Europe".to_string(),
            population: 145934460,
            population_density: Some(8.8),
            median_age: Some(39.6),
            aged_65_older: Some(14.2),
            aged_70_older: Some(9.4),
            gdp_per_capita: Some(24766.0),
            diabetes_prevalence: Some(6.2),
            cardiovasc_death_rate: Some(431.3),
            life_expectancy: 72.6,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Rwanda".to_string(),
        CountyStatistic {
            iso_code: "RWA".to_string(),
            country_name: "Rwanda".to_string(),
            continent: "Africa".to_string(),
            population: 12952209,
            population_density: Some(494.9),
            median_age: Some(20.3),
            aged_65_older: Some(3.0),
            aged_70_older: Some(1.6),
            gdp_per_capita: Some(1854.2),
            diabetes_prevalence: Some(4.3),
            cardiovasc_death_rate: Some(191.4),
            life_expectancy: 69.0,
            human_development_index: Some(0.5),
        },
    );

    alpha3.insert(
        "Saint Kitts and Nevis".to_string(),
        CountyStatistic {
            iso_code: "KNA".to_string(),
            country_name: "Saint Kitts and Nevis".to_string(),
            continent: "North America".to_string(),
            population: 53192,
            population_density: Some(212.9),
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: Some(24654.4),
            diabetes_prevalence: Some(12.8),
            cardiovasc_death_rate: None,
            life_expectancy: 76.2,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Saint Lucia".to_string(),
        CountyStatistic {
            iso_code: "LCA".to_string(),
            country_name: "Saint Lucia".to_string(),
            continent: "North America".to_string(),
            population: 183629,
            population_density: Some(293.2),
            median_age: Some(34.9),
            aged_65_older: Some(9.7),
            aged_70_older: Some(6.4),
            gdp_per_capita: Some(12951.8),
            diabetes_prevalence: Some(11.6),
            cardiovasc_death_rate: Some(204.6),
            life_expectancy: 76.2,
            human_development_index: Some(0.7),
        },
    );

    alpha3.insert(
        "Saint Vincent and the Grenadines".to_string(),
        CountyStatistic {
            iso_code: "VCT".to_string(),
            country_name: "Saint Vincent and the Grenadines".to_string(),
            continent: "North America".to_string(),
            population: 110947,
            population_density: Some(281.8),
            median_age: Some(31.8),
            aged_65_older: Some(7.7),
            aged_70_older: Some(4.8),
            gdp_per_capita: Some(10727.1),
            diabetes_prevalence: Some(11.6),
            cardiovasc_death_rate: Some(252.7),
            life_expectancy: 72.5,
            human_development_index: Some(0.7),
        },
    );

    alpha3.insert(
        "Samoa".to_string(),
        CountyStatistic {
            iso_code: "WSM".to_string(),
            country_name: "Samoa".to_string(),
            continent: "Oceania".to_string(),
            population: 198410,
            population_density: Some(69.4),
            median_age: Some(22.0),
            aged_65_older: Some(5.6),
            aged_70_older: Some(3.6),
            gdp_per_capita: Some(6021.6),
            diabetes_prevalence: Some(9.2),
            cardiovasc_death_rate: Some(349.0),
            life_expectancy: 73.3,
            human_development_index: Some(0.7),
        },
    );

    alpha3.insert(
        "San Marino".to_string(),
        CountyStatistic {
            iso_code: "SMR".to_string(),
            country_name: "San Marino".to_string(),
            continent: "Europe".to_string(),
            population: 33938,
            population_density: Some(556.7),
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: Some(56861.5),
            diabetes_prevalence: Some(5.6),
            cardiovasc_death_rate: None,
            life_expectancy: 85.0,
            human_development_index: None,
        },
    );

    alpha3.insert(
        "Sao Tome and Principe".to_string(),
        CountyStatistic {
            iso_code: "STP".to_string(),
            country_name: "Sao Tome and Principe".to_string(),
            continent: "Africa".to_string(),
            population: 219161,
            population_density: Some(212.8),
            median_age: Some(18.7),
            aged_65_older: Some(2.9),
            aged_70_older: Some(2.2),
            gdp_per_capita: Some(3052.7),
            diabetes_prevalence: Some(2.4),
            cardiovasc_death_rate: Some(270.1),
            life_expectancy: 70.4,
            human_development_index: Some(0.6),
        },
    );

    alpha3.insert(
        "Saudi Arabia".to_string(),
        CountyStatistic {
            iso_code: "SAU".to_string(),
            country_name: "Saudi Arabia".to_string(),
            continent: "Asia".to_string(),
            population: 34813867,
            population_density: Some(15.3),
            median_age: Some(31.9),
            aged_65_older: Some(3.3),
            aged_70_older: Some(1.8),
            gdp_per_capita: Some(49045.4),
            diabetes_prevalence: Some(17.7),
            cardiovasc_death_rate: Some(259.5),
            life_expectancy: 75.1,
            human_development_index: Some(0.9),
        },
    );

    alpha3.insert(
        "Senegal".to_string(),
        CountyStatistic {
            iso_code: "SEN".to_string(),
            country_name: "Senegal".to_string(),
            continent: "Africa".to_string(),
            population: 16743930,
            population_density: Some(82.3),
            median_age: Some(18.7),
            aged_65_older: Some(3.0),
            aged_70_older: Some(1.8),
            gdp_per_capita: Some(2470.6),
            diabetes_prevalence: Some(2.4),
            cardiovasc_death_rate: Some(241.2),
            life_expectancy: 67.9,
            human_development_index: Some(0.5),
        },
    );

    alpha3.insert(
        "Serbia".to_string(),
        CountyStatistic {
            iso_code: "SRB".to_string(),
            country_name: "Serbia".to_string(),
            continent: "Europe".to_string(),
            population: 6804596,
            population_density: Some(80.3),
            median_age: Some(41.2),
            aged_65_older: Some(17.4),
            aged_70_older: None,
            gdp_per_capita: Some(14048.9),
            diabetes_prevalence: Some(10.1),
            cardiovasc_death_rate: Some(439.4),
            life_expectancy: 76.0,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Seychelles".to_string(),
        CountyStatistic {
            iso_code: "SYC".to_string(),
            country_name: "Seychelles".to_string(),
            continent: "Africa".to_string(),
            population: 98340,
            population_density: Some(208.4),
            median_age: Some(36.2),
            aged_65_older: Some(8.6),
            aged_70_older: Some(5.6),
            gdp_per_capita: Some(26382.3),
            diabetes_prevalence: Some(10.6),
            cardiovasc_death_rate: Some(242.6),
            life_expectancy: 73.4,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Sierra Leone".to_string(),
        CountyStatistic {
            iso_code: "SLE".to_string(),
            country_name: "Sierra Leone".to_string(),
            continent: "Africa".to_string(),
            population: 7976985,
            population_density: Some(104.7),
            median_age: Some(19.1),
            aged_65_older: Some(2.5),
            aged_70_older: Some(1.3),
            gdp_per_capita: Some(1390.3),
            diabetes_prevalence: Some(2.4),
            cardiovasc_death_rate: Some(325.7),
            life_expectancy: 54.7,
            human_development_index: Some(0.4),
        },
    );

    alpha3.insert(
        "Singapore".to_string(),
        CountyStatistic {
            iso_code: "SGP".to_string(),
            country_name: "Singapore".to_string(),
            continent: "Asia".to_string(),
            population: 5850343,
            population_density: Some(7915.7),
            median_age: Some(42.4),
            aged_65_older: Some(12.9),
            aged_70_older: Some(7.0),
            gdp_per_capita: Some(85535.4),
            diabetes_prevalence: Some(11.0),
            cardiovasc_death_rate: Some(92.2),
            life_expectancy: 83.6,
            human_development_index: Some(0.9),
        },
    );

    alpha3.insert(
        "Slovakia".to_string(),
        CountyStatistic {
            iso_code: "SVK".to_string(),
            country_name: "Slovakia".to_string(),
            continent: "Europe".to_string(),
            population: 5459643,
            population_density: Some(113.1),
            median_age: Some(41.2),
            aged_65_older: Some(15.1),
            aged_70_older: Some(9.2),
            gdp_per_capita: Some(30155.2),
            diabetes_prevalence: Some(7.3),
            cardiovasc_death_rate: Some(288.0),
            life_expectancy: 77.5,
            human_development_index: Some(0.9),
        },
    );

    alpha3.insert(
        "Slovenia".to_string(),
        CountyStatistic {
            iso_code: "SVN".to_string(),
            country_name: "Slovenia".to_string(),
            continent: "Europe".to_string(),
            population: 2078932,
            population_density: Some(102.6),
            median_age: Some(44.5),
            aged_65_older: Some(19.1),
            aged_70_older: Some(12.9),
            gdp_per_capita: Some(31400.8),
            diabetes_prevalence: Some(7.3),
            cardiovasc_death_rate: Some(153.5),
            life_expectancy: 81.3,
            human_development_index: Some(0.9),
        },
    );

    alpha3.insert(
        "Solomon Islands".to_string(),
        CountyStatistic {
            iso_code: "SLB".to_string(),
            country_name: "Solomon Islands".to_string(),
            continent: "Oceania".to_string(),
            population: 686878,
            population_density: Some(21.8),
            median_age: Some(20.8),
            aged_65_older: Some(3.5),
            aged_70_older: Some(2.0),
            gdp_per_capita: Some(2205.9),
            diabetes_prevalence: Some(18.7),
            cardiovasc_death_rate: Some(459.8),
            life_expectancy: 73.0,
            human_development_index: Some(0.5),
        },
    );

    alpha3.insert(
        "Somalia".to_string(),
        CountyStatistic {
            iso_code: "SOM".to_string(),
            country_name: "Somalia".to_string(),
            continent: "Africa".to_string(),
            population: 15893219,
            population_density: Some(23.5),
            median_age: Some(16.8),
            aged_65_older: Some(2.7),
            aged_70_older: Some(1.5),
            gdp_per_capita: None,
            diabetes_prevalence: Some(6.0),
            cardiovasc_death_rate: Some(365.8),
            life_expectancy: 57.4,
            human_development_index: None,
        },
    );

    alpha3.insert(
        "South Africa".to_string(),
        CountyStatistic {
            iso_code: "ZAF".to_string(),
            country_name: "South Africa".to_string(),
            continent: "Africa".to_string(),
            population: 59308690,
            population_density: Some(46.8),
            median_age: Some(27.3),
            aged_65_older: Some(5.3),
            aged_70_older: Some(3.1),
            gdp_per_capita: Some(12294.9),
            diabetes_prevalence: Some(5.5),
            cardiovasc_death_rate: Some(200.4),
            life_expectancy: 64.1,
            human_development_index: Some(0.7),
        },
    );

    alpha3.insert(
        "South Sudan".to_string(),
        CountyStatistic {
            iso_code: "SSD".to_string(),
            country_name: "South Sudan".to_string(),
            continent: "Africa".to_string(),
            population: 11193729,
            population_density: None,
            median_age: Some(19.2),
            aged_65_older: Some(3.4),
            aged_70_older: Some(2.0),
            gdp_per_capita: Some(1569.9),
            diabetes_prevalence: Some(10.4),
            cardiovasc_death_rate: Some(280.8),
            life_expectancy: 57.9,
            human_development_index: Some(0.4),
        },
    );

    alpha3.insert(
        "Spain".to_string(),
        CountyStatistic {
            iso_code: "ESP".to_string(),
            country_name: "Spain".to_string(),
            continent: "Europe".to_string(),
            population: 46754783,
            population_density: Some(93.1),
            median_age: Some(45.5),
            aged_65_older: Some(19.4),
            aged_70_older: Some(13.8),
            gdp_per_capita: Some(34272.4),
            diabetes_prevalence: Some(7.2),
            cardiovasc_death_rate: Some(99.4),
            life_expectancy: 83.6,
            human_development_index: Some(0.9),
        },
    );

    alpha3.insert(
        "Sri Lanka".to_string(),
        CountyStatistic {
            iso_code: "LKA".to_string(),
            country_name: "Sri Lanka".to_string(),
            continent: "Asia".to_string(),
            population: 21413250,
            population_density: Some(342.0),
            median_age: Some(34.1),
            aged_65_older: Some(10.1),
            aged_70_older: Some(5.3),
            gdp_per_capita: Some(11669.1),
            diabetes_prevalence: Some(10.7),
            cardiovasc_death_rate: Some(197.1),
            life_expectancy: 77.0,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Sudan".to_string(),
        CountyStatistic {
            iso_code: "SDN".to_string(),
            country_name: "Sudan".to_string(),
            continent: "Africa".to_string(),
            population: 43849269,
            population_density: Some(23.3),
            median_age: Some(19.7),
            aged_65_older: Some(3.5),
            aged_70_older: Some(2.0),
            gdp_per_capita: Some(4466.5),
            diabetes_prevalence: Some(15.7),
            cardiovasc_death_rate: Some(431.4),
            life_expectancy: 65.3,
            human_development_index: Some(0.5),
        },
    );

    alpha3.insert(
        "Suriname".to_string(),
        CountyStatistic {
            iso_code: "SUR".to_string(),
            country_name: "Suriname".to_string(),
            continent: "South America".to_string(),
            population: 586634,
            population_density: Some(3.6),
            median_age: Some(29.6),
            aged_65_older: Some(6.9),
            aged_70_older: Some(4.2),
            gdp_per_capita: Some(13767.1),
            diabetes_prevalence: Some(12.5),
            cardiovasc_death_rate: Some(258.3),
            life_expectancy: 71.7,
            human_development_index: Some(0.7),
        },
    );

    alpha3.insert(
        "Sweden".to_string(),
        CountyStatistic {
            iso_code: "SWE".to_string(),
            country_name: "Sweden".to_string(),
            continent: "Europe".to_string(),
            population: 10099270,
            population_density: Some(24.7),
            median_age: Some(41.0),
            aged_65_older: Some(20.0),
            aged_70_older: Some(13.4),
            gdp_per_capita: Some(46949.3),
            diabetes_prevalence: Some(4.8),
            cardiovasc_death_rate: Some(134.0),
            life_expectancy: 82.8,
            human_development_index: Some(0.9),
        },
    );

    alpha3.insert(
        "Switzerland".to_string(),
        CountyStatistic {
            iso_code: "CHE".to_string(),
            country_name: "Switzerland".to_string(),
            continent: "Europe".to_string(),
            population: 8654618,
            population_density: Some(214.2),
            median_age: Some(43.1),
            aged_65_older: Some(18.4),
            aged_70_older: Some(12.6),
            gdp_per_capita: Some(57410.2),
            diabetes_prevalence: Some(5.6),
            cardiovasc_death_rate: Some(99.7),
            life_expectancy: 83.8,
            human_development_index: Some(0.9),
        },
    );

    alpha3.insert(
        "Syria".to_string(),
        CountyStatistic {
            iso_code: "SYR".to_string(),
            country_name: "Syria".to_string(),
            continent: "Asia".to_string(),
            population: 17500657,
            population_density: None,
            median_age: Some(21.7),
            aged_65_older: None,
            aged_70_older: Some(2.6),
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: Some(376.3),
            life_expectancy: 72.7,
            human_development_index: Some(0.5),
        },
    );

    alpha3.insert(
        "Taiwan".to_string(),
        CountyStatistic {
            iso_code: "TWN".to_string(),
            country_name: "Taiwan".to_string(),
            continent: "Asia".to_string(),
            population: 23816775,
            population_density: None,
            median_age: Some(42.2),
            aged_65_older: None,
            aged_70_older: Some(8.4),
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: Some(104.0),
            life_expectancy: 80.5,
            human_development_index: None,
        },
    );

    alpha3.insert(
        "Tajikistan".to_string(),
        CountyStatistic {
            iso_code: "TJK".to_string(),
            country_name: "Tajikistan".to_string(),
            continent: "Asia".to_string(),
            population: 9537642,
            population_density: Some(64.3),
            median_age: Some(23.3),
            aged_65_older: Some(3.5),
            aged_70_older: Some(2.2),
            gdp_per_capita: Some(2896.9),
            diabetes_prevalence: Some(7.1),
            cardiovasc_death_rate: Some(427.7),
            life_expectancy: 71.1,
            human_development_index: Some(0.7),
        },
    );

    alpha3.insert(
        "Tanzania".to_string(),
        CountyStatistic {
            iso_code: "TZA".to_string(),
            country_name: "Tanzania".to_string(),
            continent: "Africa".to_string(),
            population: 59734213,
            population_density: Some(64.7),
            median_age: Some(17.7),
            aged_65_older: Some(3.1),
            aged_70_older: Some(1.9),
            gdp_per_capita: Some(2683.3),
            diabetes_prevalence: Some(5.8),
            cardiovasc_death_rate: Some(217.3),
            life_expectancy: 65.5,
            human_development_index: Some(0.5),
        },
    );

    alpha3.insert(
        "Thailand".to_string(),
        CountyStatistic {
            iso_code: "THA".to_string(),
            country_name: "Thailand".to_string(),
            continent: "Asia".to_string(),
            population: 69799978,
            population_density: Some(135.1),
            median_age: Some(40.1),
            aged_65_older: Some(11.4),
            aged_70_older: Some(6.9),
            gdp_per_capita: Some(16277.7),
            diabetes_prevalence: Some(7.0),
            cardiovasc_death_rate: Some(109.9),
            life_expectancy: 77.2,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Timor-Leste".to_string(),
        CountyStatistic {
            iso_code: "TLS".to_string(),
            country_name: "Timor-Leste".to_string(),
            continent: "Asia".to_string(),
            population: 1318442,
            population_density: Some(87.2),
            median_age: Some(18.0),
            aged_65_older: Some(3.6),
            aged_70_older: Some(1.9),
            gdp_per_capita: Some(6570.1),
            diabetes_prevalence: Some(6.9),
            cardiovasc_death_rate: Some(335.3),
            life_expectancy: 69.5,
            human_development_index: Some(0.6),
        },
    );

    alpha3.insert(
        "Togo".to_string(),
        CountyStatistic {
            iso_code: "TGO".to_string(),
            country_name: "Togo".to_string(),
            continent: "Africa".to_string(),
            population: 8278737,
            population_density: Some(143.4),
            median_age: Some(19.4),
            aged_65_older: Some(2.8),
            aged_70_older: Some(1.5),
            gdp_per_capita: Some(1429.8),
            diabetes_prevalence: Some(6.2),
            cardiovasc_death_rate: Some(280.0),
            life_expectancy: 61.0,
            human_development_index: Some(0.5),
        },
    );

    alpha3.insert(
        "Trinidad and Tobago".to_string(),
        CountyStatistic {
            iso_code: "TTO".to_string(),
            country_name: "Trinidad and Tobago".to_string(),
            continent: "North America".to_string(),
            population: 1399491,
            population_density: Some(266.9),
            median_age: Some(36.2),
            aged_65_older: Some(10.0),
            aged_70_older: Some(5.8),
            gdp_per_capita: Some(28763.1),
            diabetes_prevalence: Some(11.0),
            cardiovasc_death_rate: Some(228.5),
            life_expectancy: 73.5,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Tunisia".to_string(),
        CountyStatistic {
            iso_code: "TUN".to_string(),
            country_name: "Tunisia".to_string(),
            continent: "Africa".to_string(),
            population: 11818618,
            population_density: Some(74.2),
            median_age: Some(32.7),
            aged_65_older: Some(8.0),
            aged_70_older: Some(5.1),
            gdp_per_capita: Some(10849.3),
            diabetes_prevalence: Some(8.5),
            cardiovasc_death_rate: Some(319.0),
            life_expectancy: 76.7,
            human_development_index: Some(0.7),
        },
    );

    alpha3.insert(
        "Turkey".to_string(),
        CountyStatistic {
            iso_code: "TUR".to_string(),
            country_name: "Turkey".to_string(),
            continent: "Asia".to_string(),
            population: 84339067,
            population_density: Some(104.9),
            median_age: Some(31.6),
            aged_65_older: Some(8.2),
            aged_70_older: Some(5.1),
            gdp_per_capita: Some(25129.3),
            diabetes_prevalence: Some(12.1),
            cardiovasc_death_rate: Some(171.3),
            life_expectancy: 77.7,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Uganda".to_string(),
        CountyStatistic {
            iso_code: "UGA".to_string(),
            country_name: "Uganda".to_string(),
            continent: "Africa".to_string(),
            population: 45741000,
            population_density: Some(213.8),
            median_age: Some(16.4),
            aged_65_older: Some(2.2),
            aged_70_older: Some(1.3),
            gdp_per_capita: Some(1697.7),
            diabetes_prevalence: Some(2.5),
            cardiovasc_death_rate: Some(213.3),
            life_expectancy: 63.4,
            human_development_index: Some(0.5),
        },
    );

    alpha3.insert(
        "Ukraine".to_string(),
        CountyStatistic {
            iso_code: "UKR".to_string(),
            country_name: "Ukraine".to_string(),
            continent: "Europe".to_string(),
            population: 43733759,
            population_density: Some(77.4),
            median_age: Some(41.4),
            aged_65_older: Some(16.5),
            aged_70_older: Some(11.1),
            gdp_per_capita: Some(7894.4),
            diabetes_prevalence: Some(7.1),
            cardiovasc_death_rate: Some(539.8),
            life_expectancy: 72.1,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "United Arab Emirates".to_string(),
        CountyStatistic {
            iso_code: "ARE".to_string(),
            country_name: "United Arab Emirates".to_string(),
            continent: "Asia".to_string(),
            population: 9890400,
            population_density: Some(112.4),
            median_age: Some(34.0),
            aged_65_older: Some(1.1),
            aged_70_older: Some(0.5),
            gdp_per_capita: Some(67293.5),
            diabetes_prevalence: Some(17.3),
            cardiovasc_death_rate: Some(317.8),
            life_expectancy: 78.0,
            human_development_index: Some(0.9),
        },
    );

    alpha3.insert(
        "United Kingdom".to_string(),
        CountyStatistic {
            iso_code: "GBR".to_string(),
            country_name: "United Kingdom".to_string(),
            continent: "Europe".to_string(),
            population: 67886004,
            population_density: Some(272.9),
            median_age: Some(40.8),
            aged_65_older: Some(18.5),
            aged_70_older: Some(12.5),
            gdp_per_capita: Some(39753.2),
            diabetes_prevalence: Some(4.3),
            cardiovasc_death_rate: Some(122.1),
            life_expectancy: 81.3,
            human_development_index: Some(0.9),
        },
    );

    alpha3.insert(
        "United States".to_string(),
        CountyStatistic {
            iso_code: "USA".to_string(),
            country_name: "United States".to_string(),
            continent: "North America".to_string(),
            population: 331002647,
            population_density: Some(35.6),
            median_age: Some(38.3),
            aged_65_older: Some(15.4),
            aged_70_older: Some(9.7),
            gdp_per_capita: Some(54225.4),
            diabetes_prevalence: Some(10.8),
            cardiovasc_death_rate: Some(151.1),
            life_expectancy: 78.9,
            human_development_index: Some(0.9),
        },
    );

    alpha3.insert(
        "Uruguay".to_string(),
        CountyStatistic {
            iso_code: "URY".to_string(),
            country_name: "Uruguay".to_string(),
            continent: "South America".to_string(),
            population: 3473727,
            population_density: Some(19.8),
            median_age: Some(35.6),
            aged_65_older: Some(14.7),
            aged_70_older: Some(10.4),
            gdp_per_capita: Some(20551.4),
            diabetes_prevalence: Some(6.9),
            cardiovasc_death_rate: Some(160.7),
            life_expectancy: 77.9,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Uzbekistan".to_string(),
        CountyStatistic {
            iso_code: "UZB".to_string(),
            country_name: "Uzbekistan".to_string(),
            continent: "Asia".to_string(),
            population: 33469199,
            population_density: Some(76.1),
            median_age: Some(28.2),
            aged_65_older: Some(4.5),
            aged_70_older: Some(2.9),
            gdp_per_capita: Some(6253.1),
            diabetes_prevalence: Some(7.6),
            cardiovasc_death_rate: Some(724.4),
            life_expectancy: 71.7,
            human_development_index: Some(0.7),
        },
    );

    alpha3.insert(
        "Vanuatu".to_string(),
        CountyStatistic {
            iso_code: "VUT".to_string(),
            country_name: "Vanuatu".to_string(),
            continent: "Oceania".to_string(),
            population: 307150,
            population_density: Some(22.7),
            median_age: Some(23.1),
            aged_65_older: Some(4.4),
            aged_70_older: Some(2.6),
            gdp_per_capita: Some(2921.9),
            diabetes_prevalence: Some(12.0),
            cardiovasc_death_rate: Some(546.3),
            life_expectancy: 70.5,
            human_development_index: Some(0.6),
        },
    );

    alpha3.insert(
        "Venezuela".to_string(),
        CountyStatistic {
            iso_code: "VEN".to_string(),
            country_name: "Venezuela".to_string(),
            continent: "South America".to_string(),
            population: 28435943,
            population_density: Some(36.3),
            median_age: Some(29.0),
            aged_65_older: Some(6.6),
            aged_70_older: Some(3.9),
            gdp_per_capita: Some(16745.0),
            diabetes_prevalence: Some(6.5),
            cardiovasc_death_rate: Some(204.8),
            life_expectancy: 72.1,
            human_development_index: Some(0.8),
        },
    );

    alpha3.insert(
        "Vietnam".to_string(),
        CountyStatistic {
            iso_code: "VNM".to_string(),
            country_name: "Vietnam".to_string(),
            continent: "Asia".to_string(),
            population: 97338583,
            population_density: Some(308.1),
            median_age: Some(32.6),
            aged_65_older: Some(7.2),
            aged_70_older: Some(4.7),
            gdp_per_capita: Some(6171.9),
            diabetes_prevalence: Some(6.0),
            cardiovasc_death_rate: Some(245.5),
            life_expectancy: 75.4,
            human_development_index: Some(0.7),
        },
    );

    alpha3.insert(
        "Yemen".to_string(),
        CountyStatistic {
            iso_code: "YEM".to_string(),
            country_name: "Yemen".to_string(),
            continent: "Asia".to_string(),
            population: 29825968,
            population_density: Some(53.5),
            median_age: Some(20.3),
            aged_65_older: Some(2.9),
            aged_70_older: Some(1.6),
            gdp_per_capita: Some(1479.1),
            diabetes_prevalence: Some(5.3),
            cardiovasc_death_rate: Some(495.0),
            life_expectancy: 66.1,
            human_development_index: Some(0.5),
        },
    );

    alpha3.insert(
        "Zambia".to_string(),
        CountyStatistic {
            iso_code: "ZMB".to_string(),
            country_name: "Zambia".to_string(),
            continent: "Africa".to_string(),
            population: 18383956,
            population_density: Some(23.0),
            median_age: Some(17.7),
            aged_65_older: Some(2.5),
            aged_70_older: Some(1.5),
            gdp_per_capita: Some(3689.3),
            diabetes_prevalence: Some(3.9),
            cardiovasc_death_rate: Some(234.5),
            life_expectancy: 63.9,
            human_development_index: Some(0.6),
        },
    );

    alpha3.insert(
        "Zimbabwe".to_string(),
        CountyStatistic {
            iso_code: "ZWE".to_string(),
            country_name: "Zimbabwe".to_string(),
            continent: "Africa".to_string(),
            population: 14862927,
            population_density: Some(42.7),
            median_age: Some(19.6),
            aged_65_older: Some(2.8),
            aged_70_older: Some(1.9),
            gdp_per_capita: Some(1899.8),
            diabetes_prevalence: Some(1.8),
            cardiovasc_death_rate: Some(307.8),
            life_expectancy: 61.5,
            human_development_index: Some(0.5),
        },
    );

    // Not yet part of GIS data

    alpha3.insert(
        "American Samoa".to_string(),
        CountyStatistic {
            iso_code: "ASM".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Anguilla".to_string(),
        CountyStatistic {
            iso_code: "AIA".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Antarctica".to_string(),
        CountyStatistic {
            iso_code: "ATA".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Aruba".to_string(),
        CountyStatistic {
            iso_code: "ABW".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Bermuda".to_string(),
        CountyStatistic {
            iso_code: "BMU".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Bonaire, Sint Eustatius and Saba".to_string(),
        CountyStatistic {
            iso_code: "BES".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Bouvet Island".to_string(),
        CountyStatistic {
            iso_code: "BVT".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "British Indian Ocean Territory".to_string(),
        CountyStatistic {
            iso_code: "IOT".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Cayman Islands".to_string(),
        CountyStatistic {
            iso_code: "CYM".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Christmas Island".to_string(),
        CountyStatistic {
            iso_code: "CXR".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Cocos (Keeling) Islands".to_string(),
        CountyStatistic {
            iso_code: "CCK".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Cook Islands".to_string(),
        CountyStatistic {
            iso_code: "COK".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Curaçao".to_string(),
        CountyStatistic {
            iso_code: "CUW".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Falkland Islands (Malvinas)".to_string(),
        CountyStatistic {
            iso_code: "FLK".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Faroe Islands".to_string(),
        CountyStatistic {
            iso_code: "FRO".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "French Guiana".to_string(),
        CountyStatistic {
            iso_code: "GUF".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "French Polynesia".to_string(),
        CountyStatistic {
            iso_code: "PYF".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "French Southern Territories".to_string(),
        CountyStatistic {
            iso_code: "ATF".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Gibraltar".to_string(),
        CountyStatistic {
            iso_code: "GIB".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Greenland".to_string(),
        CountyStatistic {
            iso_code: "GRL".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Guadeloupe".to_string(),
        CountyStatistic {
            iso_code: "GLP".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Guam".to_string(),
        CountyStatistic {
            iso_code: "GUM".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Guernsey".to_string(),
        CountyStatistic {
            iso_code: "GGY".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Heard Island and McDonald Islands".to_string(),
        CountyStatistic {
            iso_code: "HMD".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Isle of Man".to_string(),
        CountyStatistic {
            iso_code: "IMN".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Jersey".to_string(),
        CountyStatistic {
            iso_code: "JEY".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Kiribati".to_string(),
        CountyStatistic {
            iso_code: "KIR".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Korea (the Democratic People's Republic of)".to_string(),
        CountyStatistic {
            iso_code: "PRK".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Kosovo".to_string(),
        CountyStatistic {
            iso_code: "KOS".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Macao".to_string(),
        CountyStatistic {
            iso_code: "MAC".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Martinique".to_string(),
        CountyStatistic {
            iso_code: "MTQ".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Mayotte".to_string(),
        CountyStatistic {
            iso_code: "MYT".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Montserrat".to_string(),
        CountyStatistic {
            iso_code: "MSR".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Nauru".to_string(),
        CountyStatistic {
            iso_code: "NRU".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "New Caledonia".to_string(),
        CountyStatistic {
            iso_code: "NCL".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Niue".to_string(),
        CountyStatistic {
            iso_code: "NIU".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Norfolk Island".to_string(),
        CountyStatistic {
            iso_code: "NFK".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Northern Mariana Islands".to_string(),
        CountyStatistic {
            iso_code: "MNP".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Palau".to_string(),
        CountyStatistic {
            iso_code: "PLW".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Pitcairn".to_string(),
        CountyStatistic {
            iso_code: "PCN".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Puerto Rico".to_string(),
        CountyStatistic {
            iso_code: "PRI".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Réunion".to_string(),
        CountyStatistic {
            iso_code: "REU".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Saint Barthélemy".to_string(),
        CountyStatistic {
            iso_code: "BLM".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Saint Helena, Ascension and Tristan da Cunha".to_string(),
        CountyStatistic {
            iso_code: "SHN".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Saint Martin (French part)".to_string(),
        CountyStatistic {
            iso_code: "MAF".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Saint Pierre and Miquelon".to_string(),
        CountyStatistic {
            iso_code: "SPM".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Sint Maarten (Dutch part)".to_string(),
        CountyStatistic {
            iso_code: "SXM".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "South Georgia and the South Sandwich Islands".to_string(),
        CountyStatistic {
            iso_code: "SGS".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Svalbard and Jan Mayen".to_string(),
        CountyStatistic {
            iso_code: "SJM".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Tokelau".to_string(),
        CountyStatistic {
            iso_code: "TKL".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Tonga".to_string(),
        CountyStatistic {
            iso_code: "TON".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Turkmenistan".to_string(),
        CountyStatistic {
            iso_code: "TKM".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Turks and Caicos Islands".to_string(),
        CountyStatistic {
            iso_code: "TCA".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Tuvalu".to_string(),
        CountyStatistic {
            iso_code: "TUV".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "United States Minor Outlying Islands".to_string(),
        CountyStatistic {
            iso_code: "UMI".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Virgin Islands (British)".to_string(),
        CountyStatistic {
            iso_code: "VGB".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Virgin Islands (U.S.)".to_string(),
        CountyStatistic {
            iso_code: "VIR".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Wallis and Futuna".to_string(),
        CountyStatistic {
            iso_code: "WLF".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Western Sahara".to_string(),
        CountyStatistic {
            iso_code: "ESH".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );
    alpha3.insert(
        "Åland Islands".to_string(),
        CountyStatistic {
            iso_code: "ALA".to_string(),
            country_name: "".to_string(),
            continent: "".to_string(),
            population: 0,
            population_density: None,
            median_age: None,
            aged_65_older: None,
            aged_70_older: None,
            gdp_per_capita: None,
            diabetes_prevalence: None,
            cardiovasc_death_rate: None,
            life_expectancy: 0.0,
            human_development_index: None,
        },
    );

    alpha3
}

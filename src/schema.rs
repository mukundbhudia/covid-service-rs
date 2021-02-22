use mongodb::bson;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq)]
pub enum Region {
    Global,
    US,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct GlobalCaseByLocation {
    pub confirmed: i64,
    pub active: i64,
    pub recovered: i64,
    pub deaths: i64,
    pub confirmedCasesToday: i64,
    pub deathsToday: i64,
    pub confirmedPerCapita: f64,
    pub deathsPerCapita: f64,
    pub timeSeriesTotalCasesByDate: Vec<TimeSeriesCase>,
    pub globalCasesByDate: Vec<GlobalDayCase>,
    pub dateOfFirstCase: Option<String>,
    pub dateOfFirstDeath: Option<String>,
    pub highestDailyConfirmed: HighestCase,
    pub highestDailyDeaths: HighestCase,
    pub globalPopulation: i64,
    pub populationDensity: Option<f64>,
    pub medianAge: Option<f64>,
    pub aged65older: Option<f64>,
    pub aged70older: Option<f64>,
    pub gdpPerCapita: Option<f64>,
    pub diabetesPrevalence: Option<f64>,
    pub cardiovascDeathRate: Option<f64>,
    pub lifeExpectancy: Option<f64>,
    pub humanDevelopmentIndex: Option<f64>,
    pub totalTests: Option<i64>,
    pub totalTestsPerThousand: Option<f64>,
    pub totalVaccinations: Option<i64>,
    pub peopleVaccinated: Option<i64>,
    pub peopleFullyVaccinated: Option<i64>,
    pub totalVaccinationsPerHundred: Option<f64>,
    pub peopleVaccinatedPerHundred: Option<f64>,
    pub peopleFullyVaccinatedPerHundred: Option<f64>,
    pub reproductionRate: Option<f64>,
    pub positiveRate: Option<f64>,
    pub extremePoverty: Option<f64>,
    pub timeStamp: bson::DateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct CaseByLocation {
    pub idKey: String,
    pub country: String,
    pub province: Option<String>,
    pub countryCode: String,
    pub population: Option<i64>,
    pub confirmed: i64,
    pub deaths: i64,
    pub recovered: i64,
    pub active: i64,
    pub confirmedCasesToday: i64,
    pub deathsToday: i64,
    pub confirmedPerCapita: Option<f64>,
    pub deathsPerCapita: Option<f64>,
    pub lastUpdate: i64,
    pub latitude: f64,
    pub longitude: f64,
    pub dateOfFirstCase: Option<String>,
    pub dateOfFirstDeath: Option<String>,
    pub highestDailyConfirmed: HighestCase,
    pub highestDailyDeaths: HighestCase,
    pub casesByDate: Vec<TimeSeriesCase>,
    pub provincesList: Vec<Province>,
    pub hasProvince: bool,
    pub continent: Option<String>,
    pub populationDensity: Option<f64>,
    pub medianAge: Option<f64>,
    pub aged65older: Option<f64>,
    pub aged70older: Option<f64>,
    pub gdpPerCapita: Option<f64>,
    pub diabetesPrevalence: Option<f64>,
    pub cardiovascDeathRate: Option<f64>,
    pub lifeExpectancy: Option<f64>,
    pub humanDevelopmentIndex: Option<f64>,
    pub totalTests: Option<i64>,
    pub totalTestsPerThousand: Option<f64>,
    pub totalVaccinations: Option<i64>,
    pub peopleVaccinated: Option<i64>,
    pub peopleFullyVaccinated: Option<i64>,
    pub totalVaccinationsPerHundred: Option<f64>,
    pub peopleVaccinatedPerHundred: Option<f64>,
    pub peopleFullyVaccinatedPerHundred: Option<f64>,
    pub reproductionRate: Option<f64>,
    pub positiveRate: Option<f64>,
    pub extremePoverty: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct GlobalDayCase {
    pub day: String,
    pub casesOfTheDay: Vec<GlobalCaseByDate>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct GlobalCaseByDate {
    pub idKey: String,
    pub country: String,
    pub countryCode: String,
    pub confirmed: i64,
    pub deaths: i64,
    pub recovered: i64,
    pub active: i64,
    pub confirmedCasesToday: i64,
    pub deathsToday: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct Province {
    pub idKey: String,
    pub province: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct CountyStatistic {
    pub iso_code: String,
    pub country_name: String,
    pub continent: String,
    pub population: i64,
    pub population_density: Option<f64>,
    pub median_age: Option<f64>,
    pub aged_65_older: Option<f64>,
    pub aged_70_older: Option<f64>,
    pub gdp_per_capita: Option<f64>,
    pub diabetes_prevalence: Option<f64>,
    pub cardiovasc_death_rate: Option<f64>,
    pub life_expectancy: f64,
    pub human_development_index: Option<f64>,
    pub total_tests: Option<i64>,
    pub total_tests_per_thousand: Option<f64>,
    pub total_vaccinations: Option<i64>,
    pub people_vaccinated: Option<i64>,
    pub people_fully_vaccinated: Option<i64>,
    pub total_vaccinations_per_hundred: Option<f64>,
    pub people_vaccinated_per_hundred: Option<f64>,
    pub people_fully_vaccinated_per_hundred: Option<f64>,
    pub reproduction_rate: Option<f64>,
    pub positive_rate: Option<f64>,
    pub extreme_poverty: Option<f64>,
}

#[allow(non_snake_case)]
impl Default for CountyStatistic {
    fn default() -> Self {
        CountyStatistic {
            iso_code: String::new(),
            country_name: String::new(),
            continent: String::new(),
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
            total_tests: None,
            total_tests_per_thousand: None,
            total_vaccinations: None,
            people_vaccinated: None,
            people_fully_vaccinated: None,
            total_vaccinations_per_hundred: None,
            people_vaccinated_per_hundred: None,
            people_fully_vaccinated_per_hundred: None,
            reproduction_rate: None,
            positive_rate: None,
            extreme_poverty: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[allow(non_snake_case)]
pub struct TimeSeriesCase {
    pub confirmed: i64,
    pub deaths: i64,
    pub confirmedCasesToday: i64,
    pub deathsToday: i64,
    pub day: String,
}

#[allow(non_snake_case)]
impl TimeSeriesCase {
    pub fn new(
        confirmed: i64,
        deaths: i64,
        confirmedCasesToday: i64,
        deathsToday: i64,
        day: String,
    ) -> TimeSeriesCase {
        TimeSeriesCase {
            confirmed,
            deaths,
            confirmedCasesToday,
            deathsToday,
            day,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct Case {
    pub Province_State: Option<String>,
    pub Country_Region: String,
    pub Last_Update: Option<i64>,
    pub Lat: Option<f64>,
    pub Long_: Option<f64>,
    pub Confirmed: i64,
    pub Recovered: i64,
    pub Deaths: i64,
    pub Active: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[allow(non_snake_case)]
pub struct CsvCase {
    pub Province_State: Option<String>,
    pub Country_Region: String,
    pub Lat: f64,
    pub Long_: f64,
    pub cases: Vec<TimeSeriesCase>,
    pub dateOfFirstCase: Option<String>,
    pub dateOfFirstDeath: Option<String>,
    pub highestDailyConfirmed: HighestCase,
    pub highestDailyDeaths: HighestCase,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[allow(non_snake_case)]
pub struct HighestCase {
    pub count: i64,
    pub date: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct TotalsValue {
    pub value: i64,
}

#[derive(Deserialize, Debug)]
pub struct TotalsAttributes {
    pub attributes: TotalsValue,
}

#[derive(Deserialize, Debug)]
pub struct Total {
    pub features: Vec<TotalsAttributes>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CasesAttributes {
    pub attributes: Case,
}

#[derive(Deserialize, Debug)]
pub struct CasesByCountry {
    pub features: Vec<CasesAttributes>,
}

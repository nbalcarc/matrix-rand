use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;


#[derive(Debug)]
pub struct DiseaseRow {
    pub ints: Vec<i32>,
    pub string: String,
}
#[derive(Debug)]
pub struct DiseaseCSV {
    pub rows: Vec<DiseaseRow>,
}

pub fn read_disease_train() -> Result<DiseaseCSV, Box<dyn Error>> {
    // Path to your CSV file
    let csv_file_path = "/home/terrior/Downloads/kaggle_disease/Training.csv";

    // Create a CSV reader with headers enabled
    let file = File::open(csv_file_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    let mut all_rows = Vec::with_capacity(4920);

    // Iterate over CSV records
    for result in rdr.records() {
        let record = result?;

        // Parse each field from the CSV record
        let mut index = 0;
        let mut int_columns = vec![];
        while index < 132 {
            let int_value: i32 = record[index].parse()?;
            int_columns.push(int_value);
            index += 1;
        }
        
        // Parse the string field at the end
        let str_col: String = record[132].to_string();

        // Create a new Record instance with parsed values
        let parsed_record = DiseaseRow {
            ints: int_columns,
            string: str_col
        };
        all_rows.push(parsed_record);

        // Process the parsed record (e.g., store in a vector, print, etc.)
        //println!("{:?}", parsed_record);
    }

    Ok(DiseaseCSV {rows: all_rows})
}


pub fn read_disease_test() -> Result<DiseaseCSV, Box<dyn Error>> {
    // Path to your CSV file
    let csv_file_path = "/home/terrior/Downloads/kaggle_disease/Testing.csv";

    // Create a CSV reader with headers enabled
    let file = File::open(csv_file_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    let mut all_rows = Vec::with_capacity(42);

    // Iterate over CSV records
    for result in rdr.records() {
        let record = result?;

        // Parse each field from the CSV record
        let mut index = 0;
        let mut int_columns = vec![];
        while index < 132 {
            let int_value: i32 = record[index].parse()?;
            int_columns.push(int_value);
            index += 1;
        }
        
        // Parse the string field at the end
        let str_col: String = record[132].to_string();

        // Create a new Record instance with parsed values
        let parsed_record = DiseaseRow {
            ints: int_columns,
            string: str_col
        };
        all_rows.push(parsed_record);

        // Process the parsed record (e.g., store in a vector, print, etc.)
        //println!("{:?}", parsed_record);
    }

    Ok(DiseaseCSV {rows: all_rows})
}



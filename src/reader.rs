use crate::error::Csv;
use crate::transaction;

pub fn read_file_from_path(path: &str) -> Result<Vec<transaction::Transaction>, Csv> {
    csv::Reader::from_path(path)
        .map_err(|e| Csv::FileReadError(format!("Error reading file: {:?}", e)))
        .and_then(|mut reader| {
            let mut results = vec![];
            let mut errors = vec![];

            reader.deserialize().for_each(|result| match result {
                Ok(result) => results.push(result),
                Err(e) => errors.push(e),
            });

            if errors.len() > 0 {
                Err(Csv::ParseError(errors.iter().map(|x| format!("{:?}", x)).collect::<Vec<String>>()))
            } else {
                Ok(results)
            }
        })
}

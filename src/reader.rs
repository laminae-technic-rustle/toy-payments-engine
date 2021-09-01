use crate::error::Csv;
use crate::transaction;

/*
 * I chose to disgard the CSV entirely if there are elements unparse-able.
 * One might choose to return a tuple and push back the errors in the console,
 * for further / later manual processing
 * */
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
                Err(Csv::ParseError(
                    errors
                        .iter()
                        .map(|x| format!("{:?}", x))
                        .collect::<Vec<String>>(),
                ))
            } else {
                Ok(results)
            }
        })
}

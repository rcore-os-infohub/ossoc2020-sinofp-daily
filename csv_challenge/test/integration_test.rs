#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use csv_challenge::{
        Opt,
        {load_csv, write_csv},
        replace_column,
    };

    #[test]
    fn test_cfg_challenge() {
        let filename = PathBuf::from("./input/challenge.csv");
        let csv_data = load_csv(filename);
        assert!(csv_data.is_ok());

        let modified_data = replace_column(csv_data, "City", "Beijing").unwrap();
        assert!(modified_data.is_ok());

        let output_file = write_csv(
            &modified_data, "output/test.csv"
        );
        assert!(output_file.is_ok())
    }
}
use iocore::Path;
use unfolder::{fold_file, unfold_file, Result};

fn fixtures_path() -> Path {
    let path = Path::new(file!())
        .parent()
        .unwrap()
        .join("fixtures");
    assert!(path.is_dir(), "{path} should exist");
    path
}
fn output_file_path(name: &str) -> Path {
    Path::new(file!())
        .parent()
        .unwrap()
        .join("output")
        .join(name)
        .mkdir_parents()
        .unwrap()
}
fn output_folder_path(name: &str) -> Path {
    output_file_path(name).mkdir().unwrap()
}

fn fixture_path(name: &str) -> Path {
    fixtures_path().join(name)
}

#[test]
fn test_unfold_file() -> Result<()> {
    let mut messages = Vec::<String>::new();
    let input_path = fixture_path("policies-google-com_privacy.png");
    let flattened_path = unfold_file(
        &input_path,
        &output_folder_path("policies-google-com_privacy").delete()?,
        |progress| {
            messages.push(format!("{progress}"));
        },
    )?;
    let unflattened_path =
        output_file_path("policies-google-com_privacy.png.unflattened")
            .delete()?;
    fold_file(&flattened_path, &unflattened_path, |progress| {
        messages.push(format!("{progress}"));
    })?;
    assert_eq!(
        input_path.read_bytes()?,
        unflattened_path.read_bytes()?,
        "input data should match unflattened data"
    );
    assert_eq!(
        messages,
        vec![
            "Write start",
            "Write chunk 1/45",
            "Write chunk 2/45",
            "Write chunk 3/45",
            "Write chunk 4/45",
            "Write chunk 5/45",
            "Write chunk 6/45",
            "Write chunk 7/45",
            "Write chunk 8/45",
            "Write chunk 9/45",
            "Write chunk 10/45",
            "Write chunk 11/45",
            "Write chunk 12/45",
            "Write chunk 13/45",
            "Write chunk 14/45",
            "Write chunk 15/45",
            "Write chunk 16/45",
            "Write chunk 17/45",
            "Write chunk 18/45",
            "Write chunk 19/45",
            "Write chunk 20/45",
            "Write chunk 21/45",
            "Write chunk 22/45",
            "Write chunk 23/45",
            "Write chunk 24/45",
            "Write chunk 25/45",
            "Write chunk 26/45",
            "Write chunk 27/45",
            "Write chunk 28/45",
            "Write chunk 29/45",
            "Write chunk 30/45",
            "Write chunk 31/45",
            "Write chunk 32/45",
            "Write chunk 33/45",
            "Write chunk 34/45",
            "Write chunk 35/45",
            "Write chunk 36/45",
            "Write chunk 37/45",
            "Write chunk 38/45",
            "Write chunk 39/45",
            "Write chunk 40/45",
            "Write chunk 41/45",
            "Write chunk 42/45",
            "Write chunk 43/45",
            "Write chunk 44/45",
            "Write chunk 45/45",
            "Write end",
            "Read start",
            "Read chunk 1/45",
            "Read chunk 2/45",
            "Read chunk 3/45",
            "Read chunk 4/45",
            "Read chunk 5/45",
            "Read chunk 6/45",
            "Read chunk 7/45",
            "Read chunk 8/45",
            "Read chunk 9/45",
            "Read chunk 10/45",
            "Read chunk 11/45",
            "Read chunk 12/45",
            "Read chunk 13/45",
            "Read chunk 14/45",
            "Read chunk 15/45",
            "Read chunk 16/45",
            "Read chunk 17/45",
            "Read chunk 18/45",
            "Read chunk 19/45",
            "Read chunk 20/45",
            "Read chunk 21/45",
            "Read chunk 22/45",
            "Read chunk 23/45",
            "Read chunk 24/45",
            "Read chunk 25/45",
            "Read chunk 26/45",
            "Read chunk 27/45",
            "Read chunk 28/45",
            "Read chunk 29/45",
            "Read chunk 30/45",
            "Read chunk 31/45",
            "Read chunk 32/45",
            "Read chunk 33/45",
            "Read chunk 34/45",
            "Read chunk 35/45",
            "Read chunk 36/45",
            "Read chunk 37/45",
            "Read chunk 38/45",
            "Read chunk 39/45",
            "Read chunk 40/45",
            "Read chunk 41/45",
            "Read chunk 42/45",
            "Read chunk 43/45",
            "Read chunk 44/45",
            "Read chunk 45/45",
            "Read end"
        ]
    );
    Ok(())
}

#[test]
fn test_unfold_all_fixtures() -> Result<()> {
    eprintln!();
    for input_path in fixtures_path()
        .list()?
        .into_iter()
        .filter(|path| path.is_file())
    {
        let output_file =
            output_file_path(&format!("{}_folded", input_path.name()))
                .delete()?;
        let output_folder =
            output_folder_path(&format!("{}_unfolded", input_path.name()))
                .delete()?;
        let flattened_path =
            unfold_file(&input_path, &output_folder, |progress| {
                eprintln!("{input_path} => {output_folder} => {progress}");
            })?;
        fold_file(&flattened_path, &output_file, |progress| {
            eprintln!("{flattened_path} => {output_file} => {progress}");
        })?;
        assert_eq!(
            input_path.read_bytes()?,
            output_file.read_bytes()?,
            "input data should match unflattened data"
        );
    }
    Ok(())
}

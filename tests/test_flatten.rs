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
            "Unfold start",
            "Unfold chunk 1/45",
            "Unfold chunk 2/45",
            "Unfold chunk 3/45",
            "Unfold chunk 4/45",
            "Unfold chunk 5/45",
            "Unfold chunk 6/45",
            "Unfold chunk 7/45",
            "Unfold chunk 8/45",
            "Unfold chunk 9/45",
            "Unfold chunk 10/45",
            "Unfold chunk 11/45",
            "Unfold chunk 12/45",
            "Unfold chunk 13/45",
            "Unfold chunk 14/45",
            "Unfold chunk 15/45",
            "Unfold chunk 16/45",
            "Unfold chunk 17/45",
            "Unfold chunk 18/45",
            "Unfold chunk 19/45",
            "Unfold chunk 20/45",
            "Unfold chunk 21/45",
            "Unfold chunk 22/45",
            "Unfold chunk 23/45",
            "Unfold chunk 24/45",
            "Unfold chunk 25/45",
            "Unfold chunk 26/45",
            "Unfold chunk 27/45",
            "Unfold chunk 28/45",
            "Unfold chunk 29/45",
            "Unfold chunk 30/45",
            "Unfold chunk 31/45",
            "Unfold chunk 32/45",
            "Unfold chunk 33/45",
            "Unfold chunk 34/45",
            "Unfold chunk 35/45",
            "Unfold chunk 36/45",
            "Unfold chunk 37/45",
            "Unfold chunk 38/45",
            "Unfold chunk 39/45",
            "Unfold chunk 40/45",
            "Unfold chunk 41/45",
            "Unfold chunk 42/45",
            "Unfold chunk 43/45",
            "Unfold chunk 44/45",
            "Unfold chunk 45/45",
            "Unfold end",
            "Fold start",
            "Fold chunk 1/45",
            "Fold chunk 2/45",
            "Fold chunk 3/45",
            "Fold chunk 4/45",
            "Fold chunk 5/45",
            "Fold chunk 6/45",
            "Fold chunk 7/45",
            "Fold chunk 8/45",
            "Fold chunk 9/45",
            "Fold chunk 10/45",
            "Fold chunk 11/45",
            "Fold chunk 12/45",
            "Fold chunk 13/45",
            "Fold chunk 14/45",
            "Fold chunk 15/45",
            "Fold chunk 16/45",
            "Fold chunk 17/45",
            "Fold chunk 18/45",
            "Fold chunk 19/45",
            "Fold chunk 20/45",
            "Fold chunk 21/45",
            "Fold chunk 22/45",
            "Fold chunk 23/45",
            "Fold chunk 24/45",
            "Fold chunk 25/45",
            "Fold chunk 26/45",
            "Fold chunk 27/45",
            "Fold chunk 28/45",
            "Fold chunk 29/45",
            "Fold chunk 30/45",
            "Fold chunk 31/45",
            "Fold chunk 32/45",
            "Fold chunk 33/45",
            "Fold chunk 34/45",
            "Fold chunk 35/45",
            "Fold chunk 36/45",
            "Fold chunk 37/45",
            "Fold chunk 38/45",
            "Fold chunk 39/45",
            "Fold chunk 40/45",
            "Fold chunk 41/45",
            "Fold chunk 42/45",
            "Fold chunk 43/45",
            "Fold chunk 44/45",
            "Fold chunk 45/45",
            "Fold end"
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

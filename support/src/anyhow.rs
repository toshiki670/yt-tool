use anyhow::*;

pub fn collect_results<T>(results: Vec<Result<T, anyhow::Error>>) -> Result<Vec<T>> {
    let (oks, errs) = split_results(results);

    if errs.is_empty() {
        Ok(oks)
    } else {
        Err(combined_err(errs))
    }
}

fn split_results<T>(results: Vec<Result<T, anyhow::Error>>) -> (Vec<T>, Vec<anyhow::Error>) {
    let (oks, errs): (Vec<_>, Vec<_>) = results.into_iter().partition(Result::is_ok);
    let oks = oks.into_iter().filter_map(Result::ok).collect();
    let errs = errs.into_iter().filter_map(Result::err).collect();

    (oks, errs)
}

fn combined_err(errs: Vec<anyhow::Error>) -> anyhow::Error {
    let len = errs.len();
    let combined_err = errs
        .into_iter()
        .map(|e| format!("{:#}", e))
        .fold(anyhow::anyhow!("{len} error(s) occurred"), |acc, e| {
            acc.context(e)
        });

    combined_err
}

#[test]
fn test_collect_results() {
    let results = vec![
        Ok(1),
        Err(anyhow::anyhow!("error 1")),
        Ok(2),
        Err(anyhow::anyhow!("error 2")),
    ];
    let result = collect_results(results);
    assert!(result.is_err());

    let formatted_error = format!("{:#}", result.unwrap_err());
    assert_eq!(formatted_error, "error 2: error 1: 2 error(s) occurred");
}

use anyhow::*;

pub(crate) fn collect_results<T>(results: Vec<Result<T, anyhow::Error>>) -> Result<Vec<T>> {
    let (oks, errs): (Vec<_>, Vec<_>) = results.into_iter().partition(Result::is_ok);
    let oks = oks.into_iter().map(Result::unwrap).collect();
    let errs: Vec<anyhow::Error> = errs.into_iter().filter_map(Result::err).collect();

    if errs.is_empty() {
        Ok(oks)
    } else {
        let len = errs.len();
        let combined_err = errs
            .into_iter()
            .map(|e| format!("{:#}", e))
            .fold(anyhow::anyhow!("{len} error(s) occurred"), |acc, e| {
                acc.context(e)
            });

        Err(combined_err)
    }
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
    assert_eq!(
        result.unwrap_err().to_string(),
        "2 error(s) occurred: error 1\nerror 2"
    );
}

use std::path::PathBuf;

thread_local! {
    static ABSOLUTE_CUR_DIR: PathBuf = std::fs::canonicalize(std::env::current_dir().expect("")).expect("");
}

pub fn filtered_backtrace(skip_last_frame: bool) -> backtrace::Backtrace {
    let backtrace = backtrace::Backtrace::new();
    let frames = backtrace
        .frames()
        .iter()
        .filter(|frame| {
            frame.symbols().iter().any(|symbol| {
                symbol
                    .filename()
                    .and_then(|filename| filename.to_str())
                    .map(|f| {
                        let path_buf = std::fs::canonicalize(PathBuf::from(f));
                        let path_buf = match path_buf {
                            Ok(p) => p,
                            Err(_) => return false,
                        };
                        path_buf.starts_with(ABSOLUTE_CUR_DIR.with(|p| p.clone()))
                    })
                    .unwrap_or(false)
            })
        })
        .skip(if skip_last_frame { 1 } else { 0 })
        .cloned()
        .collect::<Vec<_>>();

    backtrace::Backtrace::from(frames)
}

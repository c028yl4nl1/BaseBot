pub fn set_max_open_files(limit: u64) -> Result<(), std::io::Error> {
    use libc::{rlimit, setrlimit, RLIMIT_NOFILE};
    use std::io::{Error, ErrorKind};
    let rlim = rlimit {
        rlim_cur: limit,
        rlim_max: limit,
    };
    let result = unsafe { setrlimit(RLIMIT_NOFILE, &rlim) };
    if result != 0 {
        return Err(Error::new(ErrorKind::Other, "Failed to set file limit"));
    }
    Ok(())
}

#[cfg(unix)]
mod unix_tests {
    use rlimit::{getrlimit, setrlimit, Resource, Rlim};
    use std::io::ErrorKind;

    const SOFT: Rlim = Rlim::from_raw(4 * 1024 * 1024);
    const HARD: Rlim = Rlim::from_raw(8 * 1024 * 1024);

    #[test]
    fn resource_set_get() {
        assert!(Resource::FSIZE.set(SOFT - 1, HARD).is_ok());

        assert!(setrlimit(Resource::FSIZE, SOFT, HARD).is_ok());

        assert_eq!(Resource::FSIZE.get().unwrap(), (SOFT, HARD));

        assert_eq!(
            Resource::FSIZE.set(HARD, SOFT).unwrap_err().kind(),
            ErrorKind::InvalidInput
        );
        assert_eq!(
            Resource::FSIZE.set(HARD, HARD + 1).unwrap_err().kind(),
            ErrorKind::PermissionDenied
        );
    }

    #[test]
    fn resource_get() {
        assert_eq!(
            getrlimit(Resource::CPU).unwrap(),
            (Rlim::INFINITY, Rlim::INFINITY)
        );
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn linux_prlimit() {
        use rlimit::prlimit;
        let res = Resource::CORE;

        assert!(prlimit(0, res, Some((SOFT, HARD)), None).is_ok());

        let mut soft = Rlim::default();
        let mut hard = Rlim::default();

        assert!(prlimit(0, res, None, Some((&mut soft, &mut hard))).is_ok());
        assert_eq!((soft, hard), (SOFT, HARD));

        assert_eq!(
            prlimit(0, res, Some((HARD, SOFT)), None)
                .unwrap_err()
                .kind(),
            ErrorKind::InvalidInput
        );

        assert_eq!(
            prlimit(0, res, Some((HARD, HARD + 1)), None)
                .unwrap_err()
                .kind(),
            ErrorKind::PermissionDenied
        );
    }
}

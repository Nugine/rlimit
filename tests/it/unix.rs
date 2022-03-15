use std::io::ErrorKind;

use rlimit::{getrlimit, setrlimit, Resource};

use super::{expect_err, expect_ok};

#[test]
fn resource_set_get() {
    const SOFT: u64 = 4 * 1024 * 1024;
    const HARD: u64 = 8 * 1024 * 1024;

    expect_ok(Resource::FSIZE.set(SOFT - 1, HARD));

    expect_ok(setrlimit(Resource::FSIZE, SOFT, HARD));

    assert_eq!(Resource::FSIZE.get().unwrap(), (SOFT, HARD));

    // FIXME: why does this line succeed on freebsd?
    #[cfg(not(target_os = "freebsd"))]
    {
        expect_err(Resource::FSIZE.set(HARD, SOFT), ErrorKind::InvalidInput);
    }

    expect_err(
        Resource::FSIZE.set(HARD, HARD + 1),
        ErrorKind::PermissionDenied,
    );
}

#[test]
fn resource_infinity() {
    assert_eq!(
        getrlimit(Resource::CPU).unwrap(),
        (rlimit::INFINITY, rlimit::INFINITY)
    );
}

#[test]
fn rlim_value() {
    use std::fs;
    use std::process::{Command, Stdio};

    fn exec(cmd: &[&str]) -> String {
        let mut child = Command::new(cmd[0]);
        child.args(&cmd[1..]);
        child.stdout(Stdio::piped());
        let output = child.spawn().unwrap().wait_with_output().unwrap();
        String::from_utf8(output.stdout).unwrap()
    }

    const CPP_CODE: &str = r#"
        #include<iostream>
        #include<cstdint>
        #include<sys/resource.h>
        using namespace std;
    
        int main(){
            cout<<RLIM_INFINITY<<'\n';
            cout<<RLIM_SAVED_CUR<<'\n';
            cout<<RLIM_SAVED_MAX<<'\n';
            return 0;
        }
    "#;

    let cpp_path = "/tmp/__rlim_value_test.cpp";
    let exe_path = "/tmp/__rlim_value_test";
    fs::write(cpp_path, CPP_CODE).unwrap();

    exec(&["g++", cpp_path, "-std=c++11", "-o", exe_path]);

    let c_output = exec(&[exe_path]);

    let libc_output = format!(
        "{}\n{}\n{}\n",
        libc::RLIM_INFINITY,
        libc::RLIM_SAVED_CUR,
        libc::RLIM_SAVED_MAX,
    );

    let rlimit_output = format!(
        "{}\n{}\n{}\n",
        rlimit::INFINITY,
        rlimit::SAVED_CUR,
        rlimit::SAVED_MAX
    );

    assert_eq!(c_output, libc_output);
    assert_eq!(c_output, rlimit_output);

    fs::remove_file(cpp_path).ok();
    fs::remove_file(exe_path).ok();
}

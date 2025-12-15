//! Example demonstrating the use of the asm feature for direct syscalls.
//! This example should work with the asm feature enabled.

fn main() {
    #[cfg(unix)]
    {
        use rlimit::{getrlimit, setrlimit, Resource};

        // Get current NOFILE limit
        match getrlimit(Resource::NOFILE) {
            Ok((soft, hard)) => {
                println!("Current NOFILE limits: soft={}, hard={}", soft, hard);
            }
            Err(e) => {
                eprintln!("Error getting NOFILE limit: {}", e);
                std::process::exit(1);
            }
        }

        // Try to set a new limit
        let new_soft = 1024;
        let new_hard = 4096;
        match setrlimit(Resource::NOFILE, new_soft, new_hard) {
            Ok(()) => {
                println!(
                    "Successfully set NOFILE limits: soft={}, hard={}",
                    new_soft, new_hard
                );
            }
            Err(e) => {
                println!("Note: Could not set NOFILE limit: {} (this is expected if not running as root)", e);
            }
        }

        // Verify the new limit
        match getrlimit(Resource::NOFILE) {
            Ok((soft, hard)) => {
                println!("New NOFILE limits: soft={}, hard={}", soft, hard);
            }
            Err(e) => {
                eprintln!("Error getting NOFILE limit after setting: {}", e);
                std::process::exit(1);
            }
        }

        #[cfg(target_os = "linux")]
        {
            use rlimit::prlimit;

            // Test prlimit for the current process
            let mut old_soft = 0;
            let mut old_hard = 0;
            match prlimit(
                0,
                Resource::NOFILE,
                None,
                Some((&mut old_soft, &mut old_hard)),
            ) {
                Ok(()) => {
                    println!(
                        "prlimit get for current process: soft={}, hard={}",
                        old_soft, old_hard
                    );
                }
                Err(e) => {
                    eprintln!("Error with prlimit: {}", e);
                }
            }
        }

        println!("\n✓ asm feature test completed successfully!");
    }

    #[cfg(not(unix))]
    {
        println!("This example only works on Unix systems.");
    }
}

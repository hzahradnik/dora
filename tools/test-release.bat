cargo build --release && cargo test --release && ruby tools\tester.rb --release %* && cargo run --release -- test --test-boots --boots=dora-boots --gc-verify

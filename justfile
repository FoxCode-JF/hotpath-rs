# Just configuration for hotpath-rs

# Default recipe
default:
    @just --list

# Run all tests
test_all:
    cargo test --features hotpath --test functions -- --nocapture --test-threads=1
    cargo test --features hotpath --test streams -- --nocapture --test-threads=1
    cargo test --features hotpath --test channels_crossbeam -- --nocapture --test-threads=1
    cargo test --features hotpath --test channels_ftc -- --nocapture --test-threads=1
    cargo test --features hotpath --test channels_std -- --nocapture --test-threads=1
    cargo test --features hotpath --test channels_tokio -- --nocapture --test-threads=1
    cargo test --features hotpath --test threads -- --nocapture --test-threads=1
    cargo test --features hotpath --test futures -- --nocapture --test-threads=1
    cargo test --features hotpath --test debug -- --nocapture --test-threads=1

# Start the dev server
server: docs
    cd crates/hotpath-backend && source .envrc && cargo run --bin server

# Build mdbook docs and clean .html links
docs:
    cd crates/hotpath-backend/html_src && mdbook build
    cargo run -p hotpath-backend --bin clean-html-links crates/hotpath-backend/html

# Deploy to remote server
deploy: docs
    cd crates/hotpath-backend && ./deploy.sh

# Deploy and restart server
release: deploy
    cd crates/hotpath-backend && ./remote/restart.sh
    echo "Release deployed and server restarted"

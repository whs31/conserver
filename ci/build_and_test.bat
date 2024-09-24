:: Print version information
rustc -Vv || exit /b 1
cargo -V || exit /b 1

:: Build and test main crate
if "%CFG_RELEASE_CHANNEL%"=="nightly" (
    cargo build --all-features || exit /b 1
) else (
    cargo build || exit /b 1
)
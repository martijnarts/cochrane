set dotenv-load
set dotenv-filename := ".just-env"

default:
    just --list

[no-exit-message]
setup:
    @echo 'Run `just infra/setup [GITHUB_OWNER] [GITHUB_NAME]`'
    @false

test:
    cargo test

ghi:
  pre-commit install

ghu:
  pre-commit autoupdate

ghr:
  pre-commit run --all-files

Rust Libre Storage
----
Free file share platform

## Run development

```
> rustup default nightly
> $env:DATABASE_URL='libre_storage.db'; cargo run
```

## Contrib for Windows

1. Download OpenSSL i386 with ssleay32.dll and unpack it to C:\lib\openssl\x86
1. Append OpenSSL directory to the PATH env
1. Download and place x86 build of sqlite3.dll to C:\lib\sqlite3\x86
1. Generate lib file
```
> cd C:\lib\sqlite3\x86 && lib /DEF:sqlite3.def /OUT:sqlite3.lib
```
1. Append "C:\lib\sqlite3\x86" to LIB env and set SQLITE3_LIB_DIR to the same dir
1. Install diesel_cli globally
```
> cargo install diesel_cli --no-default-features --features "sqlite"
```
1. Set dev env
```
> echo DATABASE_URL=file:libre_storage.db > .env
```
1. Initialize database
```
> diesel migration run --database-url="libre_storage.db"
```

## License

GNU AGPLv3
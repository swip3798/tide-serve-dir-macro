# Tide Serve Directory Macros

This crate offers a few macros to serve a directory of static files in tide without using the `serve_dir` function itself. Instead it loops through the static directory at compile-time and generates direct `serve_file` routes for all the files it found, or directly embeds the files in the application binary using `include_str`.

This approach aims to be more safe than just serving an entire directory, since every single file that is going to be served is known at compile time. When embedding the files in the application binary, it can also drastically improve performance.

## Usage

First add tide-serve-dir-macro to your dependencies:
```toml
[dependencies]
tide-serve-dir-macro = "0.2"
```

There are three macros available for serving a directory:

### `serve_dir!`

Generates `serve_file` endpoints for all files found in the given directory. Files that are not given at compile time won't be served. If a file is missing at runtime, the app will panic, since the `serve_file` function is unwraped.
Has the benefit that files can be modified at runtime. Furthermore it only loads the files when the given route is accessed.
```rust,no_run
let app = tide::new();
serve_dir!(app, "/path-prefix", "path/to/directory");
```

### `include_dir!`
Generates endpoints that directly include the files content in the binary. This doesn't require the static files to be available at runtime and heavily improves response time when accessing the endpoint, but also comes at the cost of a bigger binary size for the application and more memory usage and is therefore not a good idea for large files. Furthermore, the entire app needs to be recompiled when the static files are changed. Since it is not a great idea to serve large files with this macro, you can provide a 4th optional parameter, that sets the maximum file size in bytes that a file should be directly included in the binary. If the file is larger, it's going to be served using `serve_file`;

```rust,no_run
let app = tide::new();
include_dir!(app, "/path-prefix", "path/to/directory");
// Only embed files smaller than 4KiB
include_dir!(app, "/path-prefix", "path/to/directory", 4096);
```

### `auto_serve_dir!`
Helper macro that changes between the use of `serve_dir!` and `include_dir!` based on the build profile. Since the debug build should build as fast as possible and you probably want to change static files at runtime during development, this switches based on the build profile and uses `include_dir` only during a release build.

```rust,no_run
let app = tide::new();
auto_serve_dir!(app, "/path-prefix", "path/to/directory");
// Uses the same parameter set as include_dir!, although the maximum file size is ignored in the debug build
auto_serve_dir!(app, "/path-prefix", "path/to/directory", 4096);
```

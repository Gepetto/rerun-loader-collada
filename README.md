# Rerun external DataLoader for collada

## Installation
Install in your $PATH
```bash
cargo install --path . -f
```

## Usage
Open a collada file with Rerun :
```bash
rerun file.dae
```
Or use [RecordingStream::log_file_from_path()](https://docs.rs/re_data_loader/0.19.0/re_data_loader/trait.DataLoader.html):


## More informations
The Rerun Viewer will always pass at least these two pieces of information:
1. The path to be loaded, as a positional arg.
2. A shared recording ID, via the `--recording-id` flag.

It is up to you whether you make use of that shared recording ID or not.
If you use it, the data will end up in the same recording as all other plugins interested in
that file, otherwise you can just create a dedicated recording for it. Or both.

Check out `re_data_source::DataLoaderSettings` documentation for an exhaustive listing of
the available CLI parameters.

This is an example executable data-loader plugin for the Rerun Viewer. \
Any executable on your `$PATH` with a name that starts with [`rerun-loader-`] will be treated as an external data-loader.

This particular one will log Rust source code files as markdown documents, and return a special exit code to indicate that it doesn't support anything else.

See: https://rerun.io/docs/reference/data-loaders/overview

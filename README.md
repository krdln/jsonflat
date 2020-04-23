# Jsonflat

Jsonflat is a utility to convert JSONs into flatter, greppable form. Given the following json:

```
{
    "foos": [
        {
            "name": "a",
            "frobnicity": 50
        },
        {
            "name": "b",
            "frobnicity": 50
        }
    ]
}
```

it spits out:

```
.foos[0](name=a).frobnicity: 50
.foos[0](name=a).name: "a"
.foos[1](name=b).frobnicity: 50
.foos[1](name=b).name: "b"
```

Note that `name` key is special and is included in paths. This is to make paths
more meaningful if your data representation is not a dict, but rather list of
named objects.

It also works when jsons are embedded in plaintext:

```
[2020-02-02 20:02] Got response: {
    "code": 404,
    "message": "nothing there"
}. Ignoring.
```

```
[2020-02-02 20:02] Got response: .code: 404
[2020-02-02 20:02] Got response: .message: "nothing there"
[2020-02-02 20:02] Got response: {…}. Ignoring.
```

## Usage

Reads from stdin, writes to stdout.

### Example

```sh
command that returns json | jsonflat | grep "thing"
```

There's also a `stripcommonprefix` command included that should help with long paths:

```sh
command that returns json | jsonflat | grep "thing" | stripcommonprefix
```

## Installation

### Linux

Check out the releases.

### Manual

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. ```sh
   cargo install --git https://github.com/krdln/jsonflat
   ```

## License

TODO

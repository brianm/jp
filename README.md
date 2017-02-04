# jp

Extract values from JSON (or a stream of JSON values) via [json pointers](https://tools.ietf.org/html/rfc6901):

```
$ cat sample.json
{
    "name": "Brian",
    "pets":["dog"],
    "diet":{
        "drink": "coffee",
        "eat": "bacon"
    }
}
{
    "name": "Keith",
    "pets": ["cat", "dog"],
    "diet": {
        "drink": "tea",
        "eat": "granola"
    }
}
$ cat sample.json | jp /name /diet/drink /pets/0 /pets/1
Brian   coffee  dog
Keith   tea     cat   dog
$
```

# Install

On OS X, via Homebrew:

```
$ brew install brianm/tools/jp
```

Otherwise, build from source: install rust >= 1.15, checkout this repo, 
then `cargo build --release` and get binary at `target/release/jp`

# Usage

```
jp 0.1.0
Extract JSON elements from a stream

USAGE:
    jp [OPTIONS] <POINTER>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <INPUT>    input file to use if not receiving on stdin

ARGS:
    <POINTER>...    JSON Pointer expressions to match on input
```

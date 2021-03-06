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
jp 
Extract JSON elements from a stream via JSON Pointers

USAGE:
    jp [OPTIONS] <POINTER>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --delimiter <DELIMITER>    delimiter between output values, default is tab
    -i, --input <INPUT>            input file to use if not receiving on stdin

ARGS:
    <POINTER>...    JSON Pointer expressions to match on input
```

# Inspired by jq

[jq](https://stedolan.github.io/jq/) is awesome, and much more powerful than `jp`. However, I can
never remember its query language, and mostly want to convert json into tab delimited UNIXy stuff,
so this is easier for my 80% use cases :-) jq is way more powerful and awesome, though.

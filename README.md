# jp

Extract values from JSON (or a stream of JSON values) via [json pointers](https://tools.ietf.org/html/rfc6901):

```
$ echo '{"name":"Brian", "pets":["dog"]} {"name":"Keith", "pets":["dog", "cat"]}' | jp /name /pets/0 /pets/1
Brian	dog
Keith	dog	cat
```

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

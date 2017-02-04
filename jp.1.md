% JP(1)
% Brian McCallister <brianm@skife.org>

# NAME

jp - Extract values from JSON (or a stream of JSON values) via JSON Pointers

# SYNOPSIS

**jp** [*OPTIONS*] <*POINTER*>...

# FLAGS

**-h**, **--help**
:    Prints help information

**-V**, **--version**
:    Prints version information

# OPTIONS

**-d**, **--delimiter** <*DELIMITER*>
:    delimiter between output values, default is tab

**-i**, **--input** <*INPUT*>
:    input file to use if not receiving on stdin

# DESCRIPTION

Accepts a stream of JSON values, applying the JSON Pointers passed on the command line to 
each value, and printing out matches, one line per JSON value. Matched values will seperated
by the configured delimiter, which is tab by default. 

# RESOURCES

Source
:    <https://github.com/brianm/jp/>

# COPYING

Copyright 2017 Brian McCallister

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
   

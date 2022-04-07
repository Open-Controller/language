# OpenController Language Compiler

A compiler for OpenController house specifications.

## [Example Usage](test/example)

## [Documentation](https://open-controller.github.io/language/)

## Development

1. Clone the repository

        git clone https://github.com/Open-Controller/language.git
        cd ./language

2. Run with an ocdef file

        cargo run ./test.ocdef ./result.ocbin

3. Run tests

        cargo test

## Installation

1. Clone the repository

        git clone https://github.com/Open-Controller/language.git
        cd ./language

2. Install

        cargo install

## Usage

    language [OPTIONS] <input> <output>

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    OPTIONS:
        -v <verbosity>        Sets the level of verbosity [default: INFO]

    ARGS:
        <input>     Sets the input file to use
        <output>    Sets the output file to use

## License

        Copyright (C) 2022 PJTSearch

        Licensed under the Apache License, Version 2.0 (the "License");
        you may not use this file except in compliance with the License.
        You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

        Unless required by applicable law or agreed to in writing, software
        distributed under the License is distributed on an "AS IS" BASIS,
        WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
        See the License for the specific language governing permissions and
        limitations under the License.

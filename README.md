# Count Vowels Web Application in Rust

## Overview

The intent of this web application is to use [Rust] with [Hyper] and [Handlebars] to display a single web page that can be used to submit a string to the [Hyper] server.  The [Hyper] server counts the number of vowels in the text and it reports a sum of each vowel found.

## Description

Enter a string and the application counts the sum of each vowel found and the total number of vowels in the text.

This could be written completly in JavaScript but we’re testing interacting with the server and server side code.  So we will use little or no JavaScript when possible.

## Design

This application will have a single URL, with a GET request that offers a blank form.  A POST request with valid query string parameters returns the results otherwise it returns the same blank form as a GET request.

The intent is to be very strict.  Most web sites are very forgiving as far as case sensitivity, http methods, and query string parameters.  This application looks for an exact match.

## Documentation

http://rusticcoder.github.io/rust-count_vowels/count_vowels/index.html

## Project Status

[Project status of what's done, what's left](//github.com/RusticCoder/rust-count_vowels/blob/master/TODO_development.md#readme)

## Building from Source (Based on Linux Mint 17.2 Live CD)

1. Install dependencies
* Install packages
  ```sh
  $ sudo apt-get install build-essential curl git libssl-dev
  ```
* [Install Nightly Rust](//doc.rust-lang.org/book/nightly-rust.html)
  ```sh
  $ curl -s https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly
  ```

2. Edit the hosts file
  The application listens for requests to count-vowels.localhost.com
  ```sh
  $ sudo vi /etc/hosts
  ```
  There will be a line starting with "127.0.0.1" append "count-vowels.localhost.com" to the end of the line.  The resulting line will look something like the following.
> 127.0.0.1 localhost count-vowels.localhost.com

3. Clone the [source](//github.com/RusticCoder/rust-count_vowels)
  ```sh
  $ cd ~
  $ git clone https://github.com/RusticCoder/rust-count_vowels.git
  ```

4. Build
  ```sh
  $ cd ~/rust-count_vowels
  $ cargo run
  ```

5. Run
  Using your favorite browser, browse to http://count-vowels.localhost.com:1337

## Building the Documentation

  ```sh
  $ cd ~/rust-count_vowels
  $ cargo doc --no-deps --open
  ```

The generated documentation will appear in `~/rust-count_vowels/target/doc/count_vowels`.

## Notes

Tested on:

| Platform \ Architecture        | x86 | x86_64 |
|--------------------------------|-----|--------|
| Windows (7, 8, Server 2008 R2) |     |        |
| Linux (Mint 17.2)              |     |    ✓   |
| OSX (10.7 Lion or later)       |     |        |

You may find that other platforms work, but this is the officially supported build environment that is most likely to work.

## Getting Help

Submit an issue and I'll get back to you.

* [Count Vowels Issues] - Tasks, enhancements, and bugs.
* [General Feedback] - Feedback reguarding the blog, setup, or other projects in general that fit a broader subject then just this project.

## Contributor Code of Conduct

[Code of Conduct](//github.com/RusticCoder/rust-count_vowels/blob/master/code_of_conduct.md#readme)

## Copyright License

[LICENSE](//github.com/RusticCoder/rust-count_vowels/blob/master/LICENSE.md#readme)

Copyright (c) 2016 Rustic Coder

[Rust]: //www.rust-lang.org
[Handlebars]: //github.com/sunng87/handlebars-rust/blob/master/README.md#readme
[Hyper]: //github.com/hyperium/hyper/blob/master/README.md#readme
[Count Vowels Issues]: //github.com/RusticCoder/rust-count_vowels/issues/new
[General Feedback]: //github.com/RusticCoder/feedback/issues/new

# Count Vowels Web Application in Rust

## Overview

The intent of this web application is to use [Rust] with [Hyper] and [Handlebars] to display a single web page that can be used to submit a string to the [Hyper] server.  The [Hyper] server counts the number of vowels in the text and it reports a sum of each vowel found.

## Description

Enter a string and the program counts the sum of each vowel found and the total number of vowels in the text.

This could be written completly in JavaScript but we’re testing interacting with the server and server side code.  So we will use little or no JavaScript when possible.

## Design

This application will have a single URL, with a GET request that offers a blank form.  A POST request with valid query string parameters returns the results otherwise it returns the same blank form as a GET request.

The intent is to be very strict.  Most web sites are very forgiving as far as case sensitivity, http methods, and query string parameters.  This application looks for an exact match.

## Building from Source (Based on Linux Mint 17.2 Live CD)

1. Install dependencies
* Install packages
  ```sh
  $ sudo apt-get install curl git
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
> 127.0.0.1       localhost count-vowels.localhost.com

3. Clone the [source](//github.com/RusticCoder/rust-count_vowels)
  ```sh
  $ cd ~
  $ git clone https://github.com/RusticCoder/rust-count_vowels.git
  ```

4. Build
  ```sh
  $ cd ~/count_vowels
  $ cargo run
  ```

5. Run
  Using your favorite browser, browse to http://count-vowels.localhost.com:1337

## Building the Documentation

  ```sh
  $ cd ~/count_vowels
  $ cargo doc
  ```

The generated documentation will appear in a top-level `doc` directory.

## Notes

Tested on:

| Platform \ Architecture        | x86 | x86_64 |
|--------------------------------|-----|--------|
| Windows (7, 8, Server 2008 R2) |     |        |
| Linux (Mint 17.2)              |     |   ✓    |
| OSX (10.7 Lion or later)       |     |        |

You may find that other platforms work, but this is my officially supported build environment that is most likely to work.

## Getting Help

Submit an issue and I'll get back to you.

* [Count Vowels Issues] - Tasks, enhancements, and bugs.
* [General Feedback] - Feedback reguarding my blog, setup, or my projects in general that fit a broader subject then just this project.

## Contributor Code of Conduct

[Code of Conduct](//github.com/RusticCoder/rust-count_vowels/blob/master/code_of_conduct.md)

## Copyright License

[LICENSE.md](//github.com/RusticCoder/rust-count_vowels/blob/master/LICENSE.md)

Copyright (c) 2016 Rustic Coder

[Rust]: //www.rust-lang.org
[Handlebars]: //sunng.info/handlebars-rust/handlebars/index.html
[Hyper]: //hyper.rs/hyper/hyper/index.html
[Count Vowels Issues]: //github.com/RusticCoder/rust-count_vowels/issues/new
[General Feedback]: //github.com/RusticCoder/Feedback/issues/new

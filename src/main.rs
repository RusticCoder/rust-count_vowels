#![feature(convert, braced_empty_structs)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://rusticcoder.github.io/images/photo.jpg",
       html_favicon_url = "https://rusticcoder.github.io/favicon.ico",
       html_root_url = "https://rusticcoder.github.io/")]

/*
   Copyright 2015-2016 Rustic Coder. See the COPYRIGHT file at
   https://github.com/RusticCoder/rust-count_vowels/blob/master/COPYRIGHT

   Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
   http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
   or http://opensource.org/licenses/MIT>, at your option. This file may not
   be copied, modified, or distributed except according to those terms.
*/

//! A web server that will display a single web page that can be used to submit
//! a string to the server.  The server counts the number of vowels in the text
//! and it reports a sum of each vowel found.
//! 
//! # Usage
//! 
//! See: https://github.com/RusticCoder/rust-count_vowels/blob/master/README.md#readme

extern crate handlebars;
#[macro_use]
extern crate hyper;
#[macro_use]
extern crate log;
extern crate rustc_serialize as serialize;
extern crate serde_json;
extern crate url;

pub mod count_vowels;

use hyper::{header, server};
use serialize::json;
use std::{fs, path};
use std::io::{Read, BufReader};

/**
   Start the server based on the values in the Json formatted configuration 
   file.
*/
fn main() {
   let self_name: &str = "main.main()";

   let config: json::Json =
      {
         let file_name: &str = "./server.cfg";
         match fs::canonicalize(&path::Path::new(file_name)) {
            Err(why) => {
               error!("{}\tfs::canonicalize(&path::Path::new(\"{}\"))\t{}", self_name, file_name, why);
               json::Json::Null
            }
            Ok(abs_path) => {
               match fs::File::open(&abs_path.as_path()) {
                  Err(why) => {
                     error!("{}\tfs::File::open(\"{}\")\t{}", self_name,
                        match abs_path.to_str() {
                           None => {
                              file_name
                           }
                           Some(path) => {
                              path
                           }
                        }, why);

                     json::Json::Null
                  }
                  Ok(file) => {
                     match json::Json::from_reader(BufReader::new(file).by_ref()) {
                        Err(why) => {
                           error!("{}\tjson::Json::from_reader(BufReader::new(\"{}\").by_ref())\t{}", self_name,
                              match abs_path.to_str() {
                                 None => {
                                    file_name
                                 }
                                 Some(path) => {
                                    path
                                 }
                              }, why);

                           json::Json::Null
                        }
                        Ok(config) => {
                           config
                        }
                     }
                  }
               }
            }
         }
      };

   struct SimpleLogger;
   impl log::Log for SimpleLogger {
      fn enabled(&self, metadata: &log::LogMetadata) -> bool {
         metadata.level() <= log::LogLevel::Info
      }

      fn log(&self, record: &log::LogRecord) {
         if self.enabled(record.metadata()) {
            println!("{}\t{}", record.level(), record.args());
         }
      }
   }

   let logger_result: Result<(), log::SetLoggerError> =
      log::set_logger(
         |max_log_level| {
            max_log_level.set(
               match config.find_path(&["Logger", "Level"]) {
                  None => {
                     println!("INFO\t{}\tconfig.find_path(&[\"Logger\", \"Level\"]", self_name);
                     log::LogLevelFilter::Info
                  }
                  Some(logger_level_json) => {
                     match logger_level_json.as_string() {
                        None => {
                           println!("ERROR\t{}\tlogger_level_json.as_string()", self_name);
                           log::LogLevelFilter::Info
                        }
                        Some(logger_level) => {
                           match logger_level {
                              "Info" => {
                                 log::LogLevelFilter::Info
                              }
                              "Warn" => {
                                 log::LogLevelFilter::Warn
                              }
                              "Error" => {
                                 log::LogLevelFilter::Error
                              }
                              _ => {
                                 println!("ERROR\t{}\tmatch \"{}\"", self_name, logger_level);
                                 log::LogLevelFilter::Info
                              }
                           }
                        }
                     }
                  }
               }
            );
            Box::new(SimpleLogger)
         }
      );

   match logger_result {
      Err(why) => {
         println!("ERROR\t{}\tLogger initialize error\t{}", self_name, why);
      }
      Ok(logger) => {
         let _ = logger;

         match config.find_path(&["ListenOn", "IP"]) {
            None => {
               error!("{}\tconfig.find_path(&[\"ListenOn\", \"IP\"]", self_name);
            }
            Some(listen_on_ip_json) => {
               match listen_on_ip_json.as_string() {
                  None => {
                     error!("{}\tlisten_on_ip_json.as_string()", self_name);
                  }
                  Some(listen_on_ip) => {
                     match config.find_path(&["ListenOn", "Port"]) {
                        None => {
                           error!("{}\tconfig.find_path(&[\"ListenOn\", \"Port\"])", self_name);
                        }
                        Some(listen_on_port_json) => {
                           match listen_on_port_json.as_string() {
                              None => {
                                 error!("{}\tlisten_on_port_json.as_string()", self_name);
                              }
                              Some(listen_on_port) => {
                                 match server::Server::http(format!("{}:{}", listen_on_ip, listen_on_port).as_str()) {
                                    Err(why) => {
                                       error!("{}\tserver::Server::http(\"{}:{}\")\t{}", self_name, listen_on_ip, listen_on_port, why);
                                    }
                                    Ok(http_listener) => {
                                       match http_listener.handle(get_response) {
                                          Err(why) => {
                                             error!("{}\thttp_listener.handle(get_response)\t{}", self_name, why);
                                          }
                                          Ok(listening) => {
                                             let _ = listening;
                                             info!("{}\tListening on {}:{}", self_name, listen_on_ip, listen_on_port);
                                             info!("{}\tAccepting requests for http://count-vowels.localhost.com:{}", self_name, listen_on_port);
                                          }
                                       }
                                    }
                                 }
                              }
                           }
                        }
                     }
                  }
               }
            }
         }
      }
   }
}

/**
   Parse the request, if it is targeted at count-vowels.localhost.com pass the
   request along.
*/
fn get_response(req: server::Request, mut res: server::Response) {
   let self_name: &str = "main.get_response(req, res)";

   let fn_to_call: Result<fn(server::Request, server::Response), String> =
      match req.headers.get::<header::Host>() {
         None => {
            Err("req.headers.get::<header::Host>() FAILED".to_string())
         }
         Some(host) => {
            match host.hostname.as_ref() {
               "count-vowels.localhost.com" => {
                  Ok(count_vowels::get_response)
               }
               _ => {
                  Err(format!("Exception: host.hostname '{}'", host.hostname))
               }
            }
         }
      };

   match fn_to_call {
      Err(message) => {
         *res.status_mut() = hyper::NotFound;

         match res.send(message.as_bytes()) {
            Err(why) => {
               error!("{}\tres.send(\"{}\")\t{}", self_name, message, why);
            }
            Ok(res_value) => {
               res_value
            }
         }
      }
      Ok(fn_call) => {
         fn_call(req, res);
      }
   }
}

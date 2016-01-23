/*
   Copyright 2015-2016 Rustic Coder. See the COPYRIGHT file at
   https://github.com/RusticCoder/rust-count_vowels/blob/master/COPYRIGHT

   Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
   http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
   or http://opensource.org/licenses/MIT>, at your option. This file may not
   be copied, modified, or distributed except according to those terms.
*/

//! A function that will convert a request into a response that can be used to
//! submit a request to the server.  This function counts the number of vowels
//! in the text and it reports a sum of each vowel found.
//! 
//! # Notes
//! 
//! This could be written completly in JavaScript but we’re testing
//! interacting with the server and server side code.  So we will use
//! little or no JavaScript when possible.

use handlebars;
use hyper::{header, method, server, status, uri};
use serde_json;
use serialize::json;
use std::{fs, collections, path, str};
use std::io::Read;
use url;

header! { 
   /// Avoid Clickjacking attacks, by ensuring that their content is not
   /// embedded into other sites.
   (XFrameOptions, "X-Frame-Options") => [String]
}

header! {
   /// Force/Disable XSS protection
   (XXssProtection, "X-XSS-Protection") => [String]
}

/**
   Accept a HTTP request and return the appropriate HTTP response.

   # Examples

   ```rust
   extern crate hyper;
   extern crate env_logger;

   use hyper::server::{Request, Response};

   fn main() {
      hyper::Server::http("127.0.0.1:1337").unwrap().handle(get_response);
   }
   ```
*/
pub fn get_response(mut req: server::Request, res: server::Response) {
   // Convert the request into a URL object.
   let parsed_url: Result<url::Url, url::ParseError> =
      match req.headers.get::<header::Host>() {
         None => {
            // If there's no host there's no point in trying to determine
            // the URL.
            Err(url::ParseError::EmptyHost)
         }
         Some(host) => {
            match req.uri {
               uri::RequestUri::AbsolutePath(ref uri) => {
                  // Determine which port the request came in on, default
                  // to port 80.
                  let port: u16 =
                     match host.port {
                        None => {
                           80
                        }
                        Some(host_port) => {
                           host_port
                        }
                     };

                  // Return the URL object as http because we can not
                  // determine if it is htttp or https.
                  url::Url::parse(format!("http://{}:{}{}", host.hostname, port, uri).as_str())
               }
               _ => {
                  Err(url::ParseError::NonUrlCodePoint)
               }
            }
         }
      };

   // Use the parsed URL to determine what action to take.
   match parsed_url {
      Err(why) => {
         send_res(res, status::StatusCode::InternalServerError, format!("URL Parse Error\t{}", why));
      }
      Ok(url_parse) => {
         match url_parse.serialize_path() {
            None => {
               send_res(res, status::StatusCode::NotFound, format!("{}.serialize_path()", url_parse));
            }
            Some(path) => {
               // Initialize the two handlebars templates we are using.
               // TODO: In a future project find a more elegant way to handle
               // the templates.

               let mut handlebars: handlebars::Handlebars = handlebars::Handlebars::new();

               match get_template_string("./template/count-vowels/get.hbs") {
                  Err(why) => {
                     send_res(res, status::StatusCode::InternalServerError, why);
                     return;
                  }
                  Ok(template_string) => {
                     match handlebars.register_template_string("Method_Get", template_string.to_string()) {
                        Err(why) => {
                           send_res(res, status::StatusCode::InternalServerError, format!("handlebars.register_template_string(\"template\", \"{}\")\t{}", template_string, why));
                           return;
                        }
                        Ok(ok) => {
                           let _ = ok;
                        }
                     }
                  }
               }

               match get_template_string("./template/count-vowels/post.hbs") {
                  Err(why) => {
                     send_res(res, status::StatusCode::InternalServerError, why);
                     return;
                  }
                  Ok(template_string) => {
                     match handlebars.register_template_string("Method_Post", template_string.to_string()) {
                        Err(why) => {
                           send_res(res, status::StatusCode::InternalServerError, format!("handlebars.register_template_string(\"template\", \"{}\")\t{}", template_string, why));
                           return;
                        }
                        Ok(ok) => {
                           let _ = ok;
                        }
                     }
                  }
               }

               // A path exists but is it a path, method, and query string
               // combination that we recognize?
               match (&req.method, path.as_str()) {
                  (&method::Method::Get, "/") => {
                     match url_parse.query {
                        Some(query_string) => {
                           // We are not expecting a query string.
                           send_res(res, status::StatusCode::NotFound, format!("match url_parse.query::Some {} {}?{}", &req.method, path.as_str(), query_string));
                        }
                        None => {
                           match handlebars.render("Method_Get", &json::Json::Null) {
                              Err(why) => {
                                 send_res(res, status::StatusCode::InternalServerError, format!("handlebars.render(\"Method_Get\", json::Json::Null)\t{}", why))
                              }
                              Ok(template) => {
                                 send_res(res, status::StatusCode::Ok, template);
                              }
                           }
                        }
                     }
                  }
                  (&method::Method::Post, "/") => {
                     match url_parse.query {
                        Some(query_string) => {
                           // We are not expecting a query string.
                           send_res(res, status::StatusCode::NotFound, format!("match url_parse.query::None {} {}?{}", &req.method, path.as_str(), query_string));
                        }
                        None => {
                           // The most we are expecting is 255 characters.
                           // We'll accept a little more within reason but not
                           // everything the user cares to throw at us.
                           let buffer_max_len: usize = 375;
                           let mut buffer: [u8; 375] = [0; 375];
                           match req.read(&mut buffer) {
                              Err(why) => {
                                 send_res(res, status::StatusCode::InternalServerError, format!("req.read(buffer) {}", why));
                              },
                              Ok(num_bytes) => {
                                 let input_area_max_len: usize = 255;

                                 if num_bytes < 1 {
                                    send_res(res, status::StatusCode::InternalServerError, "num_bytes < 1".to_owned());
                                 } else if (buffer_max_len - 1) < num_bytes {
                                    // The user submitted more then 255 characters.
                                    send_res(res, status::StatusCode::InternalServerError, "(buffer_max_len - 1) < num_bytes".to_owned());
                                 } else {
                                    match str::from_utf8(&buffer) {
                                       Err(why) => {
                                          send_res(res, status::StatusCode::InternalServerError, format!("str::from_utf8(buffer) {}", why));
                                       }
                                       Ok(body_string) => {
                                          // From the query values we are only
                                          // looking for "input_area".  Make
                                          // sure it exists.
                                          match url::form_urlencoded::parse(body_string.as_bytes()).iter().find(|&&(ref x, _)| *x == "input_area") {
                                             None => {
                                                send_res(res, status::StatusCode::InternalServerError, "input_area missing".to_owned());
                                             }
                                             Some(input_area_value) => {
                                                if input_area_value.1.len() < 1 {
                                                   // Nothing submitted, send the blank form.
                                                   match handlebars.render("Method_Get", &json::Json::Null) {
                                                      Err(why) => {
                                                         send_res(res, status::StatusCode::InternalServerError, format!("handlebars.render(\"Method_Get\", json::Json::Null)\t{}", why))
                                                      }
                                                      Ok(template) => {
                                                         send_res(res, status::StatusCode::Ok, template);
                                                      }
                                                   }
                                                } else if input_area_max_len < input_area_value.1.len() {
                                                   // String more then 255 characters.
                                                   send_res(res, status::StatusCode::InternalServerError, "0".to_owned());
                                                } else {
                                                   // Define the object to be
                                                   // sent to Handlebars with
                                                   // the data to be returned
                                                   // to the user.
                                                   struct InputAreaResults {
                                                      input_area: String,
                                                      results_a: u16,
                                                      results_e: u16,
                                                      results_i: u16,
                                                      results_o: u16,
                                                      results_u: u16,
                                                      results_total: u16
                                                   }
                                                   impl json::ToJson for InputAreaResults {
                                                       fn to_json(&self) -> json::Json {
                                                           let mut btm: collections::BTreeMap<String, json::Json> = collections::BTreeMap::new();
                                                           
                                                           btm.insert("input_area".to_string(), self.input_area.to_json());
                                                           btm.insert("results_a".to_string(), self.results_a.to_json());
                                                           btm.insert("results_e".to_string(), self.results_e.to_json());
                                                           btm.insert("results_i".to_string(), self.results_i.to_json());
                                                           btm.insert("results_o".to_string(), self.results_o.to_json());
                                                           btm.insert("results_u".to_string(), self.results_u.to_json());
                                                           btm.insert("results_total".to_string(), self.results_total.to_json());

                                                           btm.to_json()
                                                       }
                                                   }

                                                   let mut input_area_results: InputAreaResults =
                                                      InputAreaResults {
                                                         input_area: input_area_value.1.to_string(),
                                                         results_a: 0,
                                                         results_e: 0,
                                                         results_i: 0,
                                                         results_o: 0,
                                                         results_u: 0,
                                                         results_total: 0
                                                      };

                                                   // Sum up the vowels.
                                                   for c in input_area_results.input_area.chars() {
                                                      match c {
                                                         'a' | 'A' => {
                                                            input_area_results.results_a += 1;
                                                            input_area_results.results_total += 1;
                                                         }
                                                         'e' | 'E' => {
                                                            input_area_results.results_e += 1;
                                                            input_area_results.results_total += 1;
                                                         }
                                                         'i' | 'I' => {
                                                            input_area_results.results_i += 1;
                                                            input_area_results.results_total += 1;
                                                         }
                                                         'o' | 'O' => {
                                                            input_area_results.results_o += 1;
                                                            input_area_results.results_total += 1;
                                                         }
                                                         'u' | 'U' => {
                                                            input_area_results.results_u += 1;
                                                            input_area_results.results_total += 1;
                                                         }
                                                         _ => {
                                                            // Do nothing
                                                         }
                                                      }
                                                   }

                                                   // Return the results to the user.
                                                   match handlebars.render("Method_Post", &input_area_results) {
                                                      Err(why) => {
                                                         send_res(res, status::StatusCode::InternalServerError, format!("handlebars.render(\"Method_Post\", input_area_results)\t{}", why))
                                                      }
                                                      Ok(template) => {
                                                         send_res(res, status::StatusCode::Ok, template);
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
                  // This URL accepts a string and returns the acceptable
                  // number of characters remaining that can be submitted
                  // to the application.
                  (&method::Method::Post, "/characters_remaining") => {
                     match url_parse.query {
                        Some(query_string) => {
                           // We are not expecting a query string.
                           send_res(res, status::StatusCode::NotFound, format!("match url_parse.query::Some {} {}?{}", &req.method, path.as_str(), query_string));
                        }
                        None => {
                           let buffer_max_len: usize = 275;
                           let mut buffer: [u8; 275] = [0; 275];
                           match req.read(&mut buffer) {
                              Err(why) => {
                                 send_res(res, status::StatusCode::InternalServerError, format!("{}", why));
                              },
                              Ok(num_bytes) => {
                                 // We will not accept more then 255 characters
                                 let input_area_max_len: usize = 255;

                                 if num_bytes < 1 {
                                    send_res(res, status::StatusCode::Ok, input_area_max_len.to_string());
                                 } else if (buffer_max_len - 1) < num_bytes {
                                    send_res(res, status::StatusCode::Ok, "0".to_owned());
                                 } else {
                                    match str::from_utf8(&buffer) {
                                       Err(why) => {
                                          send_res(res, status::StatusCode::InternalServerError, format!("{}", why));
                                       }
                                       Ok(body_string) => {
                                          // Posting non-null terminated data
                                          // to Hyper throws an error.  Remove
                                          // the null termination from the
                                          // string and covert it into a Json
                                          // object.
                                          match serde_json::from_str::<serde_json::Value>(body_string.trim_matches('\0')) {
                                             Err(why) => {
                                                send_res(res, status::StatusCode::InternalServerError, format!("{}", why));
                                             }
                                             Ok(body_json_value) => {
                                                // From the query values we are only looking for "input_area".
                                                match body_json_value.find_path(&["input_area"]) {
                                                   None => {
                                                      send_res(res, status::StatusCode::InternalServerError, "input_area missing".to_owned());
                                                   }
                                                   Some(input_area_json) => {
                                                      match input_area_json.as_string() {
                                                         None => {
                                                            send_res(res, status::StatusCode::InternalServerError, "input_area not string".to_owned());
                                                         }
                                                         Some(input_area_text) => {
                                                            if input_area_max_len < input_area_text.len() {
                                                               send_res(res, status::StatusCode::Ok, "0".to_owned());
                                                            } else {
                                                               send_res(res, status::StatusCode::Ok, (input_area_max_len - input_area_text.len()).to_string());
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
                  _ => {
                     send_res(res, status::StatusCode::NotFound, format!("match url_parse.serialize_path()::_ {} {}", &req.method, path));
                  }
               }
            }
         };
      }
   }
}

/**
   Send a response back to the browser in a consistent fashion.

   # Examples

   ```rust
   pub fn get_response(mut req: server::Request, res: server::Response) {
      send_res(res, status::StatusCode::InternalServerError, format!("Request Failed"));
   }
   ```
*/
fn send_res(mut res: server::Response, status_code: status::StatusCode, message: String) {
   let self_name: &str = "count_vowels.send_res(res, status_code, message)";

   *res.status_mut() = status_code;
   // Avoid Clickjacking attacks, by ensuring that their content is not
   // embedded into other sites.
   res.headers_mut().set(XFrameOptions("SAMEORIGIN".to_owned()));
   // Force XSS protection
   res.headers_mut().set(XXssProtection("1; mode=block".to_owned()));

   match res.send(
         if status_code.is_success() {
            message
         } else {
            if status_code.is_server_error() {
               error!("{}\t{}\t{}", self_name, status_code, message);
            } else {
               warn!("{}\t{}\t{}", self_name, status_code, message);
            }
            format!("{}", status_code)
         }.as_bytes()) {
      Err(why) => {
         error!("{}\tres.send(message)\t{}", self_name, why);
      }
      Ok(ok) => {
         let _ = ok;
      }
   }
}

/**
   Accept a relative path and return the contents of the file.

   # Examples

   ```rust
   fn main () {
      match get_template_string("./templates/my_template.hbs") {
         Err(why) => {
            panic!("called `get_template_string("./templates/my_template.hbs")` Err value: {:?}", why);
         }
         Ok(template_string) => {
            match handlebars.register_template_string("get", template_string.to_string()) {
               Err(why) => {
                  panic!("called `handlebars.register_template_string("get", template_string.to_string())` Err value: {:?}", why);
               }
               Ok(ok) => {
                  println!("OK Value: {:?}", ok);
               }
            }
         }
   }
   ```
*/
fn get_template_string(file_name: &str) -> Result<String, String> {
   match fs::canonicalize(&path::Path::new(file_name)) {
      Err(why) => {
         Err(format!("fs::canonicalize(&path::Path::new(\"{}\"))\t{}", file_name, why))
      },
      Ok(abs_path) => {
         match fs::File::open(&abs_path.as_path()) {
            Err(why) => {
               Err(format!("fs::File::open(\"{}\")\t{}",
                  match abs_path.to_str() {
                     None => {
                        file_name
                     }
                     Some(path) => {
                        path
                     }
                  }, why))
            },
            Ok(mut file) => {
               let mut file_contents: String = String::new();
               match file.read_to_string(&mut file_contents) {
                  Err(why) => {
                     Err(format!("\"{}\".read_to_string(file_contents)\t{}",
                        match abs_path.to_str() {
                           None => {
                              file_name
                           }
                           Some(path) => {
                              path
                           }
                        }, why))
                  },
                  Ok(num_bytes) => {
                     if num_bytes < 1 {
                        Err(format!("num_bytes < 1\t{}", num_bytes))
                     } else {
                        Ok(file_contents)
                     }
                  }
               }
            }
         }
      }
   }
}

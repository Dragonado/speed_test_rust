use random_word::Lang;
use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::process::ExitCode;
use std::time::{Duration, Instant};
use tiny_http::{Header, Method, Request, Response, Server, StatusCode};

fn get_random_sentence(n: usize) -> String {
    let mut sentence = String::new();
    for _ in 0..n {
        sentence.push_str(random_word::get_len(3, Lang::En).expect("There are three letter words"));
        sentence.push_str(" ");
    }
    sentence.pop();
    sentence
}

fn print_stats(time_taken: Duration, num_words: usize, sentence: &String) {
    let wpm = 60_f32 * num_words as f32 / time_taken.as_secs_f32();
    let characters_per_second = sentence.len() as f32 / time_taken.as_secs_f32();

    println!("\nSpeed = {} WPM", wpm);
    println!("Speed = {} Characters per Second", characters_per_second);
    println!("Total time taken = {:?}", time_taken);
    println!("Number of words = {}", num_words);
    println!("Number of characters = {}\n", sentence.len());
}

fn speed_test() {
    let num_words = 4;
    let sentence = get_random_sentence(num_words);
    println!("Please type the following sentence:\n");
    println!("{}\n", sentence);

    println!("Press 'y' and then Enter to start.");

    let stdin = io::stdin();
    // Lock stdin for faster I/O and get an iterator over lines.
    let mut lines = stdin.lock().lines();
    let user_choice = lines
        .next()
        .unwrap()
        .expect("Failed to read the first line");

    if user_choice != "y" {
        println!("User exited. Have a good day.");
        return;
    }

    let timer = Instant::now(); // Record the starting time

    let user_speed_test_answer = lines
        .next()
        .unwrap()
        .expect("Failed to read user test input");

    match user_speed_test_answer == sentence {
        true => {
            println!("\nCorrect answer!");
            print_stats(timer.elapsed(), num_words, &sentence);
        }
        false => {
            println!("Wrong answer. You mistyped somewhere.");
        }
    }
}

fn serve_bytes(request: Request, bytes: &[u8], content_type: &str) -> io::Result<()> {
    let content_type_header = Header::from_bytes("Content-Type", content_type)
        .expect("That we didn't put any garbage in the headers");
    request.respond(Response::from_data(bytes).with_header(content_type_header))
}

fn serve_404(request: Request) {
    let _ = request.respond(Response::from_string("404 unknown site.").with_status_code(404));
}

fn entry() -> Result<(), ()> {
    let mut args = env::args();
    let _ = args.next(); // ignore the program path.

    let address = args.next().unwrap_or("127.0.0.1:8000".to_string());
    let server = Server::http(&address).map_err(|err| {
        eprintln!("ERROR: could not start HTTP server at {address}: {err}");
    })?;

    println!("INFO: server started at: http://{address}/");

    for mut request in server.incoming_requests() {
        eprintln!(
            "received request! method: {:?}, url: {:?}",
            request.method(),
            request.url(),
        );

        match (request.method(), request.url()) {
            (Method::Post, "/api/get_random_sentence") => {
                let _ = request
                    .respond(Response::from_string(get_random_sentence(3)))
                    .map_err(|err| {
                        eprintln!("ERROR: could not serve a request {err}");
                    });
            }
            (Method::Post, "/api/submit_user_input") => {
                let mut content = String::new();
                let _ = request
                    .as_reader()
                    .read_to_string(&mut content)
                    .unwrap_or(0);
                // Do something with `content`, e.g., print or compare
                println!("User submitted: {}", content);
                let _ = request.respond(Response::from_string("Input received!"));
            }
            (Method::Get, "/index.js") => {
                let js_content = match fs::read("src/index.js") {
                    Ok(content) => content,
                    Err(err) => {
                        eprintln!("ERROR: could not read index.js: {err}");
                        continue; // dont crash the server.
                    }
                };

                let _ = serve_bytes(request, &js_content, "text/javascript; charset=utf-8");
            }
            (Method::Get, "/") | (Method::Get, "/index.html") => {
                let html_content = match fs::read("src/index.html") {
                    Ok(content) => content,
                    Err(err) => {
                        eprintln!("ERROR: could not read index.html: {err}");
                        continue; // dont crash the server.
                    }
                };

                let _ = serve_bytes(request, &html_content, "text/html; charset=utf-8");
            }
            _ => serve_404(request),
        }
    }

    Ok(())
}

fn main() -> ExitCode {
    match entry() {
        Ok(()) => ExitCode::SUCCESS,
        Err(()) => ExitCode::FAILURE,
    }
}

#![allow(unused)]
use parking_lot::Mutex;
use std::{
    collections::HashMap,
    fs,
    io::{BufReader, BufWriter, Read, Write},
    net::{TcpListener, TcpStream},
    path::{Path, PathBuf},
    sync::Arc,
    thread,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    OPTIONS,
    HEAD,
    PATCH,
}

impl Method {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        match bytes {
            b"GET" => Some(Method::GET),
            b"POST" => Some(Method::POST),
            b"PUT" => Some(Method::PUT),
            b"DELETE" => Some(Method::DELETE),
            b"OPTIONS" => Some(Method::OPTIONS),
            b"HEAD" => Some(Method::HEAD),
            b"PATCH" => Some(Method::PATCH),
            _ => None,
        }
    }
}

pub struct Request {
    pub method: Method,
    pub path: String,
    pub body: String,
    pub headers: HashMap<String, String>,
    pub query: HashMap<String, String>,
}

pub enum Response {
    Json(&'static str),
    JsonOwned(String),
    Text(&'static str),
    TextOwned(String),
    Html(&'static str),
    HtmlOwned(String),
    File(Vec<u8>, &'static str),
    Status(u16, &'static str),
}

impl Response {
    #[inline]
    pub fn json(data: &'static str) -> Self {
        Response::Json(data)
    }

    #[inline]
    pub fn json_owned(data: String) -> Self {
        Response::JsonOwned(data)
    }

    #[inline]
    pub fn text(data: &'static str) -> Self {
        Response::Text(data)
    }

    #[inline]
    pub fn text_owned(data: String) -> Self {
        Response::TextOwned(data)
    }

    #[inline]
    pub fn html(data: &'static str) -> Self {
        Response::Html(data)
    }

    #[inline]
    pub fn html_owned(data: String) -> Self {
        Response::HtmlOwned(data)
    }

    #[inline]
    pub fn status(code: u16, message: &'static str) -> Self {
        Response::Status(code, message)
    }

    #[inline]
    pub fn not_found() -> Self {
        Response::Status(404, "Not Found")
    }

    #[inline]
    pub fn bad_request() -> Self {
        Response::Status(400, "Bad Request")
    }

    #[inline]
    pub fn internal_error() -> Self {
        Response::Status(500, "Internal Server Error")
    }
}

struct StaticConfig {
    root: PathBuf,
    prefix: String,
}

type Handler<T> = Arc<dyn Fn(Request, &Mutex<T>) -> Response + Send + Sync>;

pub struct HttpServer<T> {
    port: u16,
    state: Arc<Mutex<T>>,
    static_routes: Vec<StaticConfig>,
    handler: Option<Handler<T>>,
    cors_enabled: bool,
}

impl<T: Send + Sync + 'static> HttpServer<T> {
    pub fn new(port: u16, state: T) -> Self {
        Self {
            port,
            state: Arc::new(Mutex::new(state)),
            static_routes: Vec::new(),
            handler: None,
            cors_enabled: true,
        }
    }

    pub fn static_files(mut self, root: impl Into<PathBuf>, prefix: impl Into<String>) -> Self {
        self.static_routes.push(StaticConfig {
            root: root.into(),
            prefix: prefix.into(),
        });
        self
    }

    pub fn cors(mut self, enabled: bool) -> Self {
        self.cors_enabled = enabled;
        self
    }

    pub fn on_request<F>(mut self, handler: F) -> Self
    where
        F: Fn(Request, &Mutex<T>) -> Response + Send + Sync + 'static,
    {
        self.handler = Some(Arc::new(handler));
        self
    }

    pub fn listen(self) -> anyhow::Result<()> {
        let addr = format!("127.0.0.1:{}", self.port);
        let listener = TcpListener::bind(&addr)?;
        println!("🚀 Server running at http://{}", addr);
        if !self.static_routes.is_empty() {
            println!("📁 Static routes:");
            for route in &self.static_routes {
                println!("   {}", route.prefix);
            }
        }

        println!(
            "🌐 CORS: {}",
            if self.cors_enabled {
                "enabled"
            } else {
                "disabled"
            }
        );
        println!("\n✓ Ready\n");

        let state = self.state;
        let handler = self.handler;
        let static_routes = Arc::new(self.static_routes);
        let cors = self.cors_enabled;

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let state = Arc::clone(&state);
                    let handler = handler.clone();
                    let static_routes = Arc::clone(&static_routes);

                    thread::spawn(move || {
                        if let Err(e) = handle_client(stream, state, handler, static_routes, cors) {
                            eprintln!("Request error: {}", e);
                        }
                    });
                }
                Err(e) => eprintln!("Connection error: {}", e),
            }
        }

        Ok(())
    }
}

// ===================================
// REQUEST HANDLING
// ===================================

fn handle_client<T>(
    stream: TcpStream,
    state: Arc<Mutex<T>>,
    handler: Option<Handler<T>>,
    static_routes: Arc<Vec<StaticConfig>>,
    cors: bool,
) -> std::io::Result<()>
where
    T: Send + Sync + 'static,
{
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::with_capacity(64 * 1024, &stream);

    let request = parse_request(&mut reader)?;

    // Handle OPTIONS preflight
    if matches!(request.method, Method::OPTIONS) {
        write_response(&mut writer, 204, "No Content", &[], cors)?;
        return Ok(());
    }

    // Try static files first
    if let Some((content, content_type)) = serve_static(&request.path, &static_routes) {
        write_file_response(&mut writer, &content, content_type, cors)?;
        return Ok(());
    }

    // Handle with user-defined handler
    let response = match handler {
        Some(ref h) => h(request, &state),
        None => Response::not_found(),
    };

    write_app_response(&mut writer, response, cors)?;
    Ok(())
}

// ===================================
// PARSING (Optimized)
// ===================================

fn parse_request(reader: &mut BufReader<&TcpStream>) -> std::io::Result<Request> {
    let mut buffer = Vec::with_capacity(4096);
    let mut total_read = 0;

    // Read until we find \r\n\r\n (end of headers)
    loop {
        let mut byte = [0u8; 1];
        reader.read_exact(&mut byte)?;
        buffer.push(byte[0]);
        total_read += 1;

        if total_read >= 4 {
            let end = total_read;
            if buffer[end - 4..end] == [b'\r', b'\n', b'\r', b'\n'] {
                break;
            }
        }

        if total_read > 8192 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Request too large",
            ));
        }
    }

    let header_end = buffer.len() - 4;
    let headers_str = std::str::from_utf8(&buffer[..header_end])
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8"))?;

    let mut lines = headers_str.lines();

    // Parse request line
    let request_line = lines.next().unwrap_or("");
    let parts: Vec<&str> = request_line.split_whitespace().collect();

    let method =
        Method::from_bytes(parts.get(0).unwrap_or(&"GET").as_bytes()).unwrap_or(Method::GET);

    let full_path = parts.get(1).unwrap_or(&"/");
    let (path, query) = parse_path_and_query(full_path);

    // Parse headers
    let mut headers = HashMap::with_capacity(16);
    let mut content_length = 0;

    for line in lines {
        if let Some(colon_pos) = line.find(':') {
            let key = line[..colon_pos].trim().to_lowercase();
            let value = line[colon_pos + 1..].trim();

            if key == "content-length" {
                content_length = value.parse().unwrap_or(0);
            }

            headers.insert(key, value.to_string());
        }
    }

    // Read body if present
    let body = if content_length > 0 {
        let mut body_buf = vec![0u8; content_length.min(1024 * 1024)]; // Max 1MB
        reader.read_exact(&mut body_buf)?;
        String::from_utf8_lossy(&body_buf).into_owned()
    } else {
        String::new()
    };

    Ok(Request {
        method,
        path,
        body,
        headers,
        query,
    })
}

#[inline]
fn parse_path_and_query(full_path: &str) -> (String, HashMap<String, String>) {
    if let Some(idx) = full_path.find('?') {
        let path = full_path[..idx].to_string();
        let query = parse_query(&full_path[idx + 1..]);
        (path, query)
    } else {
        (full_path.to_string(), HashMap::new())
    }
}

#[inline]
fn parse_query(query_str: &str) -> HashMap<String, String> {
    query_str
        .split('&')
        .filter_map(|pair| {
            let mut parts = pair.splitn(2, '=');
            Some((
                url_decode(parts.next()?),
                url_decode(parts.next().unwrap_or("")),
            ))
        })
        .collect()
}

// ===================================
// STATIC FILE SERVING (Optimized)
// ===================================

fn serve_static(path: &str, configs: &[StaticConfig]) -> Option<(Vec<u8>, &'static str)> {
    for config in configs {
        if !path.starts_with(&config.prefix) {
            continue;
        }

        let rel_path = path[config.prefix.len()..].trim_start_matches('/');
        let rel_path = url_decode(rel_path);
        let file_path = config.root.join(&rel_path);

        // Security: prevent path traversal
        let Ok(canonical) = file_path.canonicalize() else {
            continue;
        };
        let Ok(base_canonical) = config.root.canonicalize() else {
            continue;
        };

        if !canonical.starts_with(base_canonical) {
            eprintln!("⚠️  Path traversal attempt: {:?}", canonical);
            continue;
        }

        match fs::read(&canonical) {
            Ok(contents) => {
                let content_type = get_content_type(&canonical);
                return Some((contents, content_type));
            }
            Err(_) => continue,
        }
    }
    None
}

#[inline]
fn get_content_type(path: &Path) -> &'static str {
    match path.extension().and_then(|s| s.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "application/javascript; charset=utf-8",
        Some("json") => "application/json; charset=utf-8",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("svg") => "image/svg+xml",
        Some("webp") => "image/webp",
        Some("ico") => "image/x-icon",
        Some("woff") => "font/woff",
        Some("woff2") => "font/woff2",
        Some("ttf") => "font/ttf",
        Some("mp4") => "video/mp4",
        Some("webm") => "video/webm",
        Some("pdf") => "application/pdf",
        Some("txt") => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}

// ===================================
// RESPONSE WRITING (Optimized)
// ===================================

const CORS_HEADERS: &str = "Access-Control-Allow-Origin: *\r\n\
     Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS, PATCH\r\n\
     Access-Control-Allow-Headers: Content-Type, Authorization\r\n\
     Access-Control-Max-Age: 86400\r\n";

#[inline]
fn write_response(
    writer: &mut BufWriter<&TcpStream>,
    status: u16,
    status_text: &str,
    body: &[u8],
    cors: bool,
) -> std::io::Result<()> {
    write!(
        writer,
        "HTTP/1.1 {} {}\r\n{}\r\n",
        status,
        status_text,
        if cors { CORS_HEADERS } else { "" }
    )?;
    writer.write_all(body)?;
    writer.flush()
}

#[inline]
fn write_file_response(
    writer: &mut BufWriter<&TcpStream>,
    content: &[u8],
    content_type: &str,
    cors: bool,
) -> std::io::Result<()> {
    write!(
        writer,
        "HTTP/1.1 200 OK\r\n\
         {}Content-Type: {}\r\n\
         Content-Length: {}\r\n\
         Cache-Control: public, max-age=3600\r\n\r\n",
        if cors { CORS_HEADERS } else { "" },
        content_type,
        content.len()
    )?;
    writer.write_all(content)?;
    writer.flush()
}

fn write_app_response(
    writer: &mut BufWriter<&TcpStream>,
    response: Response,
    cors: bool,
) -> std::io::Result<()> {
    match response {
        Response::Json(data) => {
            write_typed_response(writer, 200, "application/json", data.as_bytes(), cors)
        }
        Response::JsonOwned(data) => {
            write_typed_response(writer, 200, "application/json", data.as_bytes(), cors)
        }
        Response::Text(data) => {
            write_typed_response(writer, 200, "text/plain", data.as_bytes(), cors)
        }
        Response::TextOwned(data) => {
            write_typed_response(writer, 200, "text/plain", data.as_bytes(), cors)
        }
        Response::Html(data) => {
            write_typed_response(writer, 200, "text/html", data.as_bytes(), cors)
        }
        Response::HtmlOwned(data) => {
            write_typed_response(writer, 200, "text/html", data.as_bytes(), cors)
        }
        Response::File(data, ct) => write_file_response(writer, &data, ct, cors),
        Response::Status(code, msg) => {
            write!(
                writer,
                "HTTP/1.1 {} {}\r\n\
                 {}Content-Type: text/plain\r\n\
                 Content-Length: {}\r\n\r\n",
                code,
                msg,
                if cors { CORS_HEADERS } else { "" },
                msg.len()
            )?;
            writer.write_all(msg.as_bytes())?;
            writer.flush()
        }
    }
}

#[inline]
fn write_typed_response(
    writer: &mut BufWriter<&TcpStream>,
    status: u16,
    content_type: &str,
    body: &[u8],
    cors: bool,
) -> std::io::Result<()> {
    write!(
        writer,
        "HTTP/1.1 {} OK\r\n\
         {}Content-Type: {}\r\n\
         Content-Length: {}\r\n\r\n",
        status,
        if cors { CORS_HEADERS } else { "" },
        content_type,
        body.len()
    )?;
    writer.write_all(body)?;
    writer.flush()
}

// ===================================
// UTILITIES
// ===================================

fn url_decode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars();

    while let Some(c) = chars.next() {
        match c {
            '%' => {
                let hex: String = chars.by_ref().take(2).collect();
                if hex.len() == 2 {
                    if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                        result.push(byte as char);
                        continue;
                    }
                }
                result.push('%');
                result.push_str(&hex);
            }
            '+' => result.push(' '),
            _ => result.push(c),
        }
    }

    result
}

// ===================================
// EXAMPLE USAGE
// ===================================

#[cfg(test)]
mod example {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone)]
    struct Todo {
        id: String,
        content: String,
        status: u8,
    }

    struct AppState {
        todos: Vec<Todo>,
    }

    #[test]
    fn example_server() {
        let state = AppState { todos: vec![] };

        HttpServer::new(8080, state)
            .cors(true)
            .static_files("./public/images", "/images")
            .static_files("./public/assets", "/assets")
            .on_request(|req, state| {
                println!("{:?} {}", req.method, req.path);

                match (req.method, req.path.as_str()) {
                    (Method::GET, "/") => Response::json(r#"{"message":"Hello from Rust!"}"#),

                    (Method::GET, "/todos") => {
                        let state = state.lock();
                        let json = serde_json::to_string(&state.todos)
                            .unwrap_or_else(|_| "[]".to_string());
                        Response::json_owned(json)
                    }

                    (Method::POST, "/todos") => {
                        #[derive(Deserialize)]
                        struct AddTodo {
                            content: String,
                        }

                        match serde_json::from_str::<AddTodo>(&req.body) {
                            Ok(add_req) => {
                                let mut state = state.lock();
                                let id = format!("todo_{}", state.todos.len() + 1);
                                let todo = Todo {
                                    id: id.clone(),
                                    content: add_req.content,
                                    status: 0,
                                };
                                state.todos.push(todo.clone());
                                let json = serde_json::to_string(&todo)
                                    .unwrap_or_else(|_| "{}".to_string());
                                Response::json_owned(json)
                            }
                            Err(_) => Response::bad_request(),
                        }
                    }

                    (Method::GET, path) if path.starts_with("/todos/") => {
                        let id = &path[7..];
                        let state = state.lock();
                        match state.todos.iter().find(|t| t.id == id) {
                            Some(todo) => {
                                let json = serde_json::to_string(todo)
                                    .unwrap_or_else(|_| "{}".to_string());
                                Response::json_owned(json)
                            }
                            None => Response::not_found(),
                        }
                    }

                    (Method::DELETE, path) if path.starts_with("/todos/") => {
                        let id = &path[7..];
                        let mut state = state.lock();
                        match state.todos.iter().position(|t| t.id == id) {
                            Some(pos) => {
                                state.todos.remove(pos);
                                Response::json(r#"{"success":true}"#)
                            }
                            None => Response::not_found(),
                        }
                    }

                    _ => Response::not_found(),
                }
            });
    }
}

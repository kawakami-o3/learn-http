# learn-http

`learn-http` is a basic, experimental HTTP server. 

# Usage


1. Run a server.

```
% cargo run --bin http-server
```

2. Access with a simple client.

```
% cargo run --bin http-client
```

# Features

## HTTP/0.9

HTTP/1.0 contains HTTP/0.9 features, and `learn-http` supports them.

## HTTP/1.0

* Configuration
  * [x] Content root
  * [x] Host's IP address
  * [x] Listen port
  * [x] Extension status code
  * [ ] Allow
  * [ ] Content-Encoding
  * [ ] Expires
  * [ ] Location

* Header
  * [ ] Allow https://tools.ietf.org/html/rfc1945#section-10.1
  * [x] Authorization https://tools.ietf.org/html/rfc1945#section-10.2
  * [ ] Content-Encoding https://tools.ietf.org/html/rfc1945#section-10.3
  * [x] Content-Length
  * [x] Content-Type
  * [x] Date
  * [ ] Expires https://tools.ietf.org/html/rfc1945#section-10.7
  * [x] From
  * [x] If-Modified-Since
  * [x] Last-Modified
  * [ ] Location
  * [x] Pragma
  * [x] Referer
  * [x] Server
  * [x] User-Agent
  * [x] WWW-authenticate https://tools.ietf.org/html/rfc1945#section-10.16
  
  
* Additinal Header
  * [ ] Accept
  * [ ] Accept-Charset
  * [ ] Accept-Encoding
  * [ ] Accept-Language
  * [ ] Link
  * [ ] MIME-Version
  * [ ] Retry-After
  * [ ] Title
  * [ ] URI
  

* Access Authentication https://tools.ietf.org/html/rfc1945#section-11
  * [x] Basic authentication

## HTTP/1.1

* Method

* Header

* Access Authentication https://tools.ietf.org/html/rfc2616#section-11
  * [ ] Digest authentication https://tools.ietf.org/html/rfc2616#ref-43

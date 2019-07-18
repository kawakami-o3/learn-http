# learn-http

`learn-http` is a simple toy HTTP server. 

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

* Configuration
  * [x] Content root
  * [x] Host's IP address
  * [x] Listen port
  * [x] SERVER header
  * [ ] Extension status code
  * [ ] Location response header
  * [ ] Allow response header

* Response Header
  * [ ] WWW-authenticate
  * [ ] Pragma header
  * [ ] Content-Encoding https://tools.ietf.org/html/rfc1945#section-10.3
  * [ ] Expires https://tools.ietf.org/html/rfc1945#section-10.7
  * [x] Last-Modified

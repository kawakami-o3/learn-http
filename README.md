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
  
* [ ] WWW-authenticate
* [ ] Pragma header
* [ ] Content-Encoding

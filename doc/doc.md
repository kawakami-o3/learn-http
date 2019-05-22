# HTTP/0.9

HTTP/1.0 に simple-request / simple-response として内包されている。

例

```
GET /
GET http://hoge.com/
```

```
Simple-Request  = "GET" SP Request-URI CRLF

Request-URI    = absoluteURI | abs_path

absoluteURI    = scheme ":" *( uchar | reserved )

abs_path       = "/" rel_path
rel_path       = [ path ] [ ";" params ] [ "?" query ]

path           = fsegment *( "/" segment )
fsegment       = 1*pchar
segment        = *pchar

params         = param *( ";" param )
param          = *( pchar | "/" )

scheme         = 1*( ALPHA | DIGIT | "+" | "-" | "." )
query          = *( uchar | reserved )
fragment       = *( uchar | reserved )

pchar          = uchar | ":" | "@" | "&" | "=" | "+"
uchar          = unreserved | escape
reserved       = ";" | "/" | "?" | ":" | "@" | "&" | "=" | "+"

ALPHA          = UPALPHA | LOALPHA
DIGIT          = <any US-ASCII digit "0".."9">
CR             = <US-ASCII CR, carriage return (13)>
LF             = <US-ASCII LF, linefeed (10)>
SP             = <US-ASCII SP, space (32)>
```

```
Simple-Response = [ Entity-Body ] 
```



# HTTP/1.0

https://tools.ietf.org/html/rfc1945

# HTTP/1.1

https://tools.ietf.org/html/rfc2068

# HTTP/2.0

https://tools.ietf.org/html/rfc7540

# memo

https://developer.mozilla.org/ja/docs/Web/HTTP/Basics_of_HTTP/Evolution_of_HTTP


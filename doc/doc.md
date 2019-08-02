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
unreserved     = ALPHA | DIGIT | safe | extra | national

escape         = "%" HEX HEX
reserved       = ";" | "/" | "?" | ":" | "@" | "&" | "=" | "+"
extra          = "!" | "*" | "'" | "(" | ")" | ","
safe           = "$" | "-" | "_" | "."
unsafe         = CTL | SP | <"> | "#" | "%" | "<" | ">"
national       = <any OCTET excluding ALPHA, DIGIT, reserved, extra, safe, and unsafe>

HEX            = "A" | "B" | "C" | "D" | "E" | "F"
               | "a" | "b" | "c" | "d" | "e" | "f" | DIGIT

UPALPHA        = <any US-ASCII uppercase letter "A".."Z">
LOALPHA        = <any US-ASCII lowercase letter "a".."z">
ALPHA          = UPALPHA | LOALPHA
DIGIT          = <any US-ASCII digit "0".."9">
CR             = <US-ASCII CR, carriage return (13)>
LF             = <US-ASCII LF, linefeed (10)>
SP             = <US-ASCII SP, space (32)>
```

```
Simple-Response = [ Entity-Body ]

Entity-Body    = *OCTET

OCTET          = <any 8-bit sequence of data>
```



# HTTP/1.0

https://tools.ietf.org/html/rfc1945

https://tools.ietf.org/html/rfc1945#section-4.1

```
Full-Request   = Request-Line             ; Section 5.1
                *( General-Header        ; Section 4.3
                 | Request-Header        ; Section 5.2
                 | Entity-Header )       ; Section 7.1
                CRLF
                [ Entity-Body ]          ; Section 7.2

Request-Line = Method SP Request-URI SP HTTP-Version CRLF
```

```
       LWS            = [CRLF] 1*( SP | HT )

      token          = 1*<any CHAR except CTLs or tspecials>

       tspecials      = "(" | ")" | "<" | ">" | "@"
                      | "," | ";" | ":" | "\" | <">
                      | "/" | "[" | "]" | "?" | "="
                      | "{" | "}" | SP | HT
```


## Response

https://tools.ietf.org/html/rfc1945#section-6


```
       Response        = Simple-Response | Full-Response

       Simple-Response = [ Entity-Body ]

       Full-Response   = Status-Line             ; Section 6.1
                         *( General-Header       ; Section 4.3
                          | Response-Header      ; Section 6.2
                          | Entity-Header )      ; Section 7.1
                         CRLF
                         [ Entity-Body ]         ; Section 7.2
```

```
       Response-Header = Location                ; Section 10.11
                       | Server                  ; Section 10.14
                       | WWW-Authenticate        ; Section 10.16
```


https://tools.ietf.org/html/rfc1945#section-7.1

```
       Entity-Header  = Allow                    ; Section 10.1
                      | Content-Encoding         ; Section 10.3
                      | Content-Length           ; Section 10.4
                      | Content-Type             ; Section 10.5
                      | Expires                  ; Section 10.7
                      | Last-Modified            ; Section 10.10
                      | extension-header

       extension-header = HTTP-header
```



# HTTP/1.1

https://tools.ietf.org/html/rfc2616

# HTTP/2.0

https://tools.ietf.org/html/rfc7540

# memo

https://developer.mozilla.org/ja/docs/Web/HTTP/Basics_of_HTTP/Evolution_of_HTTP


https://yamitzky.hatenablog.com/entry/2016/05/13/204107

https://httpbin.org/ip


https://nullsweep.com/http-security-headers-a-complete-guide/

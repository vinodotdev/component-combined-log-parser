# combined-log-parser

## Building

The Makefile includes the recipes necessary to generate code from the WIDL schemas in ./schemas/.

To codegen, build, and sign your WASM module, run `make`

```shell
$ make
```

## Codegen and clean

```shell
$ make clean && make codegen
```

## Testing

```shell
$ make test
```

```
wasmflow invoke ./build/text_log_parser.signed.wasm  --trace parser -- --log_format='"$remote_addr - $remote_user [$time_local] \"$request\" $status $body_bytes_sent \"$http_referer\" \"$http_user_agent\""' --log_entry='"135.125.244.48 - - [15/Apr/2022:15:15:35 +0000] \"GET /.env HTTP/1.1\" 404 1371 \"-\" \"Crazy User Agent\""' --field_seperator='" "'
```

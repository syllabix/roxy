# roxy
A local development proxy that rox!

## Running

   roxy start -p 8080

  In a separate terminal, you can now use the proxy with curl:

    curl -v -H 'Content-Type: application/json'  localhost:8080 -d '{ "test": "ok" }'

## Development

  Requires OpenSSL installed.

  Mac: 

    brew install openssl

  Ubuntu:

    sudo apt-get install pkg-config libssl-dev

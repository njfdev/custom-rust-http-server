# Custom Rust HTTP Server

This is the repository for a custom HTTP server written in Rust. It is developed with CodeCrafters, but is still my original code.

## Running

Just run this command and a webserver will start on port 4221.

```bash
./your_server.sh
```

## Capabilities

You can go to the index page which returns nothing. You can also go to `/echo/type_whatever_here` and it will return whatever you typed. The `/user-agent` page will return your User-Agent header. If you pass a `--directory <path>` argument, the `/files/<file_name>` path will return any file specified in that directory. Any other page will return a 404 error code.

The server has a very basic multithreading capability, where it will start a new thread for every request.

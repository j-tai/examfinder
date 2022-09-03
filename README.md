# Exam Finder

Find past exams for University of Waterloo courses.

Live: [**https://examfinder.jtai.ca**](https://examfinder.jtai.ca/)

## Stack

* Frontend: TypeScript, Svelte
* Backend: Rust, Axum, Redis

## Building

You need to install Node.js, Rust, and Redis.

Run the Redis server:

```bash
cd server
redis-server redis.conf
```

In another terminal, run the server:

```bash
cd server
cargo run
```

In another terminal, run the client:

```bash
cd client
npm run dev
```

Now you can open the web app at `http://localhost:5173`.

## License

[MIT License](LICENSE)

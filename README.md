# Blogrs

## Description

This is a Blog API written in Rust using the [Axum](https://github.com/tokio-rs/axum) framework. This application is (will serve as) backend for my [portfolio blog](https://izhar.xyz) site. This project is still in development.

## Installation

1. Clone the repository: `git clone https://github.com/zzedddd/blogrs.git`
2. Navigate to the project directory: `cd blogrs`
3. Install dependencies: `cargo build`

## Usage

To run the project, use the command `cargo run`.

## API Routes and Endpoints

This project provides the following API routes:

### GET /api/post

Fetches all posts.

Example usage:

```bash
curl -X GET http://localhost:8000/api/post
```

### GET /api/post/:slug

Fetches the details of a specific post, identified by its slug.

Example usage:

```bash
curl -X GET http://localhost:8000/post/api/my-first-post
```

### POST /api/post/create

Creates a new post. This route is protected and requires authentication.

Example usage:

```bash
curl -X POST -H "Content-Type: application/json" -d '{"title":"My Post", "slug":"my-post", "excerpt":"This is my post", "content":"This is the content of my post"}' http://localhost:8000/api/post/create
```

### PATCH /api/post/update/:slug

Updates a specific post, identified by its slug. This route is protected and requires authentication.

Example usage:

```bash
curl -X PATCH -H "Content-Type: application/json" -d '{"title":"Updated Post", "slug":"updated-post", "excerpt":"This is my updated post", "content":"This is the updated content of my post"}' http://localhost:8000/api/post/update/my-post
```

### DELETE /api/post/delete/:slug

Deletes a specific post, identified by its slug. This route is protected and requires authentication.

Example usage:

```bash
curl -X DELETE http://localhost:8000/api/post/delete/my-post
```

### POST /api/auth/register

Registers a new user.

Example usage:

```bash
curl -X POST -H "Content-Type: application/json" -d '{"email": "admin@gmail.com", "password":"password"}' http://localhost:8000/api/auth/register
```

### POST /api/auth/login

Authenticates a user and returns a JWT token.

Example usage:

```bash
curl -X POST -H "Content-Type: application/json" -d '{"username":"admin", "email": "admin@gmail.com", "password":"password"}' http://localhost:8000/api/auth/login
```

### GET /api/auth/current_user

Fetches the details of the currently authenticated user. This route is protected and requires authentication.

Example usage:

```bash
curl http://localhost:8000/api/auth/current_user
```

### POST /api/auth/logout

Logs out the currently authenticated user. This route is protected and requires authentication.

Example usage:

```bash
curl -X POST http://localhost:8000/api/auth/logout
```

## Environment Variables

This project uses the following environment variables:

-   `DATABASE_URL`: The URL of the database to connect to.
-   `RUST_LOG`: (Optional) The logging level for the application.
-   `JWT_SECRET`: The secret key used to sign JWT tokens.

## Contributing

If you want to contribute to this project, please create a fork, make your changes, and submit a pull request.

## License

This project is licensed under the MIT license. See the [LICENSE](LICENSE) file for more details.

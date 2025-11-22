# About

This repository is an example of how I like to do error handling with
[axum](https://crates.io/crates/axum) with the help of
[enum_convert](https://crates.io/crates/enum_convert) to avoid some repetitive
[From](https://doc.rust-lang.org/std/convert/trait.From.html) implementations.

The code implements a dummy job runner API with those few routes:
- `POST /jobs` to create a job
- `GET /jobs` to get the jobs list
- `GET /job/{job_id}` to get a specific job
- `POST /job/{job_id}/reset` to reset a job state
- `POST /job/{job_id}/cancel` to cancel a job so that it does not get executed

The main idea for error handling is that each route handler has its own error type.  
This allows a very precise description of the possible failure cases of each handler.  
Note that all of those handler error types have an `InternalError` variant which is here
to handle server side errors (mapped to http 500 error code) such as the database being unreachable.
The other variants are user errors (mapped to http 4xx error code).

Because some handlers have some errors in common (for example `JobNotFound`),
a global `ApiError` is used which contains the union of all the variants of all
the handler errors.  
This allows writing the error message and the mapping to http error code only once
for each variant.  
Note that in this example the error is returned as a simple string,
but one can easily use a more advanced format such as
[RFC7807](https://datatracker.ietf.org/doc/html/rfc7807).

To convert between the handler errors to `ApiError`, we implement
`From<HandlerError> for ApiError`. This is where
[enum_convert](https://crates.io/crates/enum_convert) comes in handy, thanks to the
`EnumInto` derive macro, avoiding the repetitive `From` manual implementation.


# Running

Should be as simple as `cargo run`. You should then be able to interact with the
endpoints, for example using [httpie](https://httpie.io/):

```
http POST :3000/jobs description=test
http :3000/jobs
http :3000/jobs/your-job-uuid
http POST :3000/jobs/your-job-uuid/reset
http POST :3000/jobs/your-job-uuid/cancel
```

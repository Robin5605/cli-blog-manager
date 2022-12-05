# cli-blog-manager
A CLI tool to manage my blog posts. Written in Rust 🚀 and powered by [Clap](https://github.com/clap-rs/clap) 👏

## Syntax
The tool can be run in one of 4 modes:
- `create <FILENAME>`
  - Create a local file with a blog template. This is not published yet.
- `publish <PATH_STR>`
  - Publish the local blog `.md` file to AWS S3, so it can reflect on the site
- `delete <filename>`
  - Delete a blog post that is already on S3 (the filename is the one on S3, not local)
- `list`
  - List all currently published blog posts
  
## Setup
See the [instructions on AWS's documentation](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/credentials.html) to configure AWS Rust SDK credentials

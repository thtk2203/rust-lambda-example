# Introduction

arc-example is a Rust project that implements an AWS Lambda function in Rust.

## References

* [完全理解: AWS Lambda with Go and Rust](https://zenn.dev/taiki45/articles/aws-lambda-with-go-and-rust)
* [RustでAWS Lambda functionをいい感じに書く](https://speakerdeck.com/taiki45/rustdeaws-lambda-functionwoiigan-zinishu-ku?slide=10)

## 挙動
* 並列に呼び出しても、シーケンシャルに実行される
  * 下記のように、ともに約`30`秒かかっている
  * `main.rs`で、`Arc::clone()`相当した結果も、参照数は増えていない（`2`のまま）

### Arcの参照数
```rust
let service = shared_service.clone();
println!("service cloned: {}", Arc::strong_count(&service));
```
この出力を含むログ
```log
     Running `target/debug/arc-example`
service cloned: 2
payload: IncomingMessage { command: "sample-command" }
service cloned: 2
payload: IncomingMessage { command: "sample-command" }
service cloned: 2
payload: IncomingMessage { command: "sample-command" }
service cloned: 2
payload: IncomingMessage { command: "sample-command" }
service cloned: 2
payload: IncomingMessage { command: "sample-command" }
service cloned: 2
payload: IncomingMessage { command: "sample-command" }
service cloned: 2
payload: IncomingMessage { command: "sample-command" }
service cloned: 2
payload: IncomingMessage { command: "sample-command" }
service cloned: 2
payload: IncomingMessage { command: "sample-command" }
service cloned: 2
payload: IncomingMessage { command: "sample-command" }
```

### 並列に実行した場合
`-P5`で`5`並列に呼び出し
```shell
$ time seq 10 | xargs -n1 -P5 -I@ ./invoke.sh @
1: Sun Jan 12 19:01:08 JST 2025
2: Sun Jan 12 19:01:08 JST 2025
3: Sun Jan 12 19:01:08 JST 2025
4: Sun Jan 12 19:01:08 JST 2025
5: Sun Jan 12 19:01:08 JST 2025
{"req_id":"5da1bd80-2d8d-4602-94f4-4a16986712e2","msg":"Command sample-command."}
1: Sun Jan 12 19:01:11 JST 2025
6: Sun Jan 12 19:01:11 JST 2025
{"req_id":"18956c44-c7fa-4284-85ba-0d19a81f108b","msg":"Command sample-command."}
2: Sun Jan 12 19:01:14 JST 2025
7: Sun Jan 12 19:01:14 JST 2025
{"req_id":"a7684ba5-2552-49c6-bf76-8db64ecf3aa6","msg":"Command sample-command."}
3: Sun Jan 12 19:01:17 JST 2025
8: Sun Jan 12 19:01:17 JST 2025
{"req_id":"0e5578e4-09f0-4542-a8bc-f3e22e3ae744","msg":"Command sample-command."}
4: Sun Jan 12 19:01:20 JST 2025
9: Sun Jan 12 19:01:20 JST 2025
{"req_id":"a7d0074d-a8ec-4b98-9212-b6c73cc755d2","msg":"Command sample-command."}
5: Sun Jan 12 19:01:23 JST 2025
10: Sun Jan 12 19:01:23 JST 2025
{"req_id":"393cde01-9f42-48c2-946b-e9ff0c5390d8","msg":"Command sample-command."}
6: Sun Jan 12 19:01:26 JST 2025
{"req_id":"ad5fe289-6ef2-4f44-96d2-fe710fb24c99","msg":"Command sample-command."}
7: Sun Jan 12 19:01:29 JST 2025
{"req_id":"b32e3e88-1dba-4933-97ba-ed3529f313ce","msg":"Command sample-command."}
8: Sun Jan 12 19:01:32 JST 2025
{"req_id":"e8f672f1-d662-4793-a7d0-6c184230085d","msg":"Command sample-command."}
9: Sun Jan 12 19:01:35 JST 2025
{"req_id":"4a457000-f132-461f-8b9d-27de21f9ab72","msg":"Command sample-command."}
10: Sun Jan 12 19:01:38 JST 2025

real    0m30.166s
user    0m0.523s
sys     0m0.240s
```

### 直列に実行した場合
`-P1`で`1`並列に呼び出し
```shell
$ time seq 10 | xargs -n1 -P1 -I@ ./invoke.sh @
1: Sun Jan 12 19:01:43 JST 2025
{"req_id":"48bdfa53-fdc5-46ee-b8ae-83d1ff672431","msg":"Command sample-command."}
1: Sun Jan 12 19:01:46 JST 2025
2: Sun Jan 12 19:01:46 JST 2025
{"req_id":"40d0c941-4867-4d33-9abd-b2c0b70100be","msg":"Command sample-command."}
2: Sun Jan 12 19:01:49 JST 2025
3: Sun Jan 12 19:01:49 JST 2025
{"req_id":"ce8ff5a6-f04d-4998-b7d5-2d00a4984a73","msg":"Command sample-command."}
3: Sun Jan 12 19:01:52 JST 2025
4: Sun Jan 12 19:01:52 JST 2025
{"req_id":"1151380c-7f3b-4ecc-adfd-33e1aea3e0ed","msg":"Command sample-command."}
4: Sun Jan 12 19:01:55 JST 2025
5: Sun Jan 12 19:01:55 JST 2025
{"req_id":"dd346134-6ef6-472e-908d-5b95f1be5f08","msg":"Command sample-command."}
5: Sun Jan 12 19:01:58 JST 2025
6: Sun Jan 12 19:01:58 JST 2025
{"req_id":"55c75868-3ef7-4f9a-841b-ef50c8271dc0","msg":"Command sample-command."}
6: Sun Jan 12 19:02:01 JST 2025
7: Sun Jan 12 19:02:01 JST 2025
{"req_id":"47f1c686-1f03-486f-9e20-786bfea62763","msg":"Command sample-command."}
7: Sun Jan 12 19:02:05 JST 2025
8: Sun Jan 12 19:02:05 JST 2025
{"req_id":"245e2d18-7d9d-462a-a0e9-611a9ebeb27c","msg":"Command sample-command."}
8: Sun Jan 12 19:02:08 JST 2025
9: Sun Jan 12 19:02:08 JST 2025
{"req_id":"fbff07b1-6634-4efb-b7a6-26eb42d1cdde","msg":"Command sample-command."}
9: Sun Jan 12 19:02:11 JST 2025
10: Sun Jan 12 19:02:11 JST 2025
{"req_id":"a4152239-51aa-4b90-a9f8-03bf85d27071","msg":"Command sample-command."}
10: Sun Jan 12 19:02:14 JST 2025

real    0m31.105s
user    0m0.420s
sys     0m0.211s
```

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo Lambda](https://www.cargo-lambda.info/guide/installation.html)

## Building

To build the project for production, run `cargo lambda build --release`. Remove the `--release` flag to build for development.

Read more about building your lambda function in [the Cargo Lambda documentation](https://www.cargo-lambda.info/commands/build.html).

## Testing

You can run regular Rust unit tests with `cargo test`.

If you want to run integration tests locally, you can use the `cargo lambda watch` and `cargo lambda invoke` commands to do it.

First, run `cargo lambda watch` to start a local server. When you make changes to the code, the server will automatically restart.

Second, you'll need a way to pass the event data to the lambda function.

You can use the existent [event payloads](https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/lambda-events/src/fixtures) in the Rust Runtime repository if your lambda function is using one of the supported event types.

You can use those examples directly with the `--data-example` flag, where the value is the name of the file in the [lambda-events](https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/lambda-events/src/fixtures) repository without the `example_` prefix and the `.json` extension.

```bash
cargo lambda invoke --data-example apigw-request
```

For generic events, where you define the event data structure, you can create a JSON file with the data you want to test with. For example:

```json
{
    "command": "test"
}
```

Then, run `cargo lambda invoke --data-file ./data.json` to invoke the function with the data in `data.json`.


Read more about running the local server in [the Cargo Lambda documentation for the `watch` command](https://www.cargo-lambda.info/commands/watch.html).
Read more about invoking the function in [the Cargo Lambda documentation for the `invoke` command](https://www.cargo-lambda.info/commands/invoke.html).

## Deploying

To deploy the project, run `cargo lambda deploy`. This will create an IAM role and a Lambda function in your AWS account.

Read more about deploying your lambda function in [the Cargo Lambda documentation](https://www.cargo-lambda.info/commands/deploy.html).

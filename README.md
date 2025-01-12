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
$ time seq 10 | xargs -n1 -P5 -I@ cargo lambda invoke --data-file test/test-payload.json
{"req_id":"17c245c6-3ae4-45b0-b7a5-f1978157c7b1","msg":"Command sample-command."}
{"req_id":"44dfdc6b-219d-4bd9-87d7-620cce57605a","msg":"Command sample-command."}
{"req_id":"58f319cc-4417-4942-a8e7-b1bc29cf6e09","msg":"Command sample-command."}
{"req_id":"9d349f61-b5dc-4e58-abda-c11324036a58","msg":"Command sample-command."}
{"req_id":"52d23d9e-05e7-44ba-8b0e-9a12e2bac92c","msg":"Command sample-command."}
{"req_id":"18665515-6b74-4329-85ad-4c55fcf2e6eb","msg":"Command sample-command."}
{"req_id":"aba83f40-a48e-4cc2-8b58-c062b147dc99","msg":"Command sample-command."}
{"req_id":"644127ae-ed22-4754-a5c1-f89681896d9a","msg":"Command sample-command."}
{"req_id":"a23b5d42-fd03-4fbc-b8bf-a15c1b6226b4","msg":"Command sample-command."}
{"req_id":"9ed065d0-bcf3-4ad8-9a38-4e69a828a649","msg":"Command sample-command."}

real    0m30.140s
user    0m0.328s
sys     0m0.286s
```

### 直列に実行した場合
`-P1`で`1`並列に呼び出し
```shell
$ time seq 10 | xargs -n1 -P1 -I@ cargo lambda invoke --data-file test/test-payload.json
{"req_id":"3ff418d6-d554-4247-9604-36119776f3f7","msg":"Command sample-command."}
{"req_id":"0c515f65-87d0-4051-bcde-8c9b32f89cee","msg":"Command sample-command."}
{"req_id":"a938957d-1118-4d6a-981e-eb5b1d660f8c","msg":"Command sample-command."}
{"req_id":"8dbc98ed-6aa9-4bdd-8fbd-cc22282c366f","msg":"Command sample-command."}
{"req_id":"cac0ee8c-d99f-4c27-b69c-e002ecc17174","msg":"Command sample-command."}
{"req_id":"c8ad148b-2dd1-4bc7-a36a-3c4f5a91c80e","msg":"Command sample-command."}
{"req_id":"99ec1407-0c4d-44e3-b3c8-c9aae59817a8","msg":"Command sample-command."}
{"req_id":"5a3274c9-1c6f-4b52-9506-6ba89df9dcb1","msg":"Command sample-command."}
{"req_id":"85e55556-cad9-4f83-96df-983fa90b43ff","msg":"Command sample-command."}
{"req_id":"faf7b668-99ad-4282-84d1-752a4f48ed33","msg":"Command sample-command."}

real    0m31.450s
user    0m0.537s
sys     0m0.242s
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

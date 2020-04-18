# barbeque_client-rs
[Barbeque](https://github.com/cookpad/barbeque) client for Rust.

# Example usage

```
% cargo run --features mock_server --example mock_server &
[1] 501077
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/examples/mock_server`
% cargo run --example enqueue
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/examples/enqueue`
Enqueued 502fe886-77bf-4295-89b5-ee5da253d9e9
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/examples/job
[502fe886-77bf-4295-89b5-ee5da253d9e9] Executing TestJob with Message { model_id: 42 }
```

# TODO:
- rewrite tests using cycle and array of (ans, input)
- ? "smt" -> `smt`
- write rendering tests



## Later:
- ? create extension macro:

  ```
  extension! {
      fn String.multiply(times: u32) -> String { ... }
  }
  ```
  results into:
  ```
  trait ExtensionMultiply {
      fn multiply(&self, times: u32) -> String;
  }
  impl ExtensionMultiply for String {
      fn multiply(times: u32) -> String { ... }
  }
  ```


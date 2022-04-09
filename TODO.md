# TODO:
- impl `combine`
- write output file to folder same as input file
- add `-o` option for output folder
- ? "smt" -> `smt`
- write rendering tests
- rewrite tests using cycle



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


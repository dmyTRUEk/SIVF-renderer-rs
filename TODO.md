# TODO:
- `"smt"` -> `\`smt\``
- const fn

- add cli option for render variant: cpu, gpu, ...
- create extension macro:
  ```
  extension! {
      fn String.multiply(times: u32) -> String {
          ...
      }
  }
  ```
  results into:
  ```
  trait ExtensionMultiply {
      fn multiply(&self, times: u32) -> String;
  }
  impl ExtensionMultiply for String {
      fn multiply(times: u32) -> String {
          ...
      }
  }
  ```

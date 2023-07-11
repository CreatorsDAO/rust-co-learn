a simple blockchain demo for learning 


# blockchain-rust -


a simple blockchain demo for learning


## usage

- Create wallet:
  ```sh
  cargo run createwallet
  ```
- Create blockchain:
  ```
  cargo run create  <address>
  ```
- send coins (if `-m` is specified, the block will be mined immediately in the same node):
  ```
  cargo run send <from> <to> <amount> -m 
  ```


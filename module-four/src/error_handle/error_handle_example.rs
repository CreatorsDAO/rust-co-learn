//! 使用第三方库自定义错误
//!

/**

```
    // 扩展:使用第三方库thiserror来自定义错误

    use std::io;
    use thiserror::Error;



    #[derive(Error, Debug)]
    pub enum DataStoreError {
        #[error("data store disconnected")]
        Disconnect(#[from] io::Error),
        #[error("the data for key `{0}` is not available")]
        Redaction(String),
        #[error("invalid header (expected {expected:?}, found {found:?})")]
        InvalidHeader { expected: String, found: String },
        #[error("unknown data store error")]
        Unknown,
    }

    fn read_data(key: &str) -> Result<(), DataStoreError> {
        if key == "invalid_key" {
            return Err(DataStoreError::Redaction(format!(
                "The data for key `{}` is not available",
                key
            )));
        }

        // 读取数据的逻辑...

        Ok(())
    }

    match read_data("valid_key") {
        Ok(()) => println!("read success"),
        Err(err) => println!("error: {:?}", err),
    }


```
*/

pub fn error_handle_example() {
    println!("");
}

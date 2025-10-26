# protobuf generated code
- super::Resize  表示上一模块的Resize struct
  ```rust
    pub struct Resize {}
    public struct Spec{
        data : Option<spec::Data>,
    }
    pub mod spec{
        pub enum Data{
            Resize(super::Resize),
            ....
        }
    }

  ```


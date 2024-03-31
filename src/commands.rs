pub trait Command<T> {
    fn execute(args: T) -> Result<String, String>;
}

pub mod generate;

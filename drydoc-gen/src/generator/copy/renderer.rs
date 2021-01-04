use std::collections::HashMap;

macro_rules! map(
  { $($key:expr => $value:expr),+ } => {
      {
          let mut m = ::std::collections::HashMap::new();
          $(
              m.insert($key, $value);
          )+
          m
      }
   };
);

lazy_static! {
  pub static ref RENDERER_MAPPINGS: HashMap<&'static str, &'static str> = map! {
    "md" => "text/markdown",
    "markdown" => "text/markdown"
  };
}

pub fn lookup<'a, E: AsRef<str>>(extension: E) -> Option<&'static str> {
  RENDERER_MAPPINGS.get(extension.as_ref().to_lowercase().as_str()).map(|ty| *ty)
}
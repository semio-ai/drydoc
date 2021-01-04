use crate::actor::{Actor, Addr, Receiver};
// use tokio::sync::oneshot::Sender;

use super::{GeneratorMsg, GenerateError};
use crate::config::Unit;
use crate::page::{Id, Page};
use crate::bundle::{Bundle, Manifest};
use crate::fs::{File};

use std::collections::HashMap;
use std::collections::HashSet;

use std::path::Path;

mod content_type;
mod renderer;

static PARAM_PATH: &'static str = "path";
static NAME_PATH: &'static str = "name";

pub struct CopyGenerator {

}

impl CopyGenerator {
  pub fn new() -> Self {
    Self {}
  }

  async fn generate(unit: Unit, prefix: String) -> Result<Bundle, GenerateError> {
    assert_eq!(unit.rule.name.as_str(), "copy");

    let path = match unit.rule.params.get(&PARAM_PATH.to_string()) {
      Some(path) => Path::new(path),
      None => return Err(GenerateError::MissingParameter(PARAM_PATH.to_string()))
    };

    let name = match unit.rule.params.get(&NAME_PATH.to_string()) {
      Some(name) => name.to_string(),
      None => path.to_str().unwrap().to_string()
    };

    let extension = path.extension().map(|s| s.to_os_string().into_string().unwrap());

    let content_type = extension.clone()
      .and_then(|e| content_type::lookup(e))
      .unwrap_or("application/unknown")
      .to_string();

    let renderer = extension
      .and_then(|e| renderer::lookup(e))
      .unwrap_or("default")
      .to_string();

    let page_id = Id(format!("{}", prefix));
    let mut pages = HashMap::new();
    
    pages.insert(page_id.clone(), Page {
      id: page_id.clone(),
      content_type,
      name,
      renderer,
      metadata: HashMap::new(),
      children: HashSet::new()
    });

    let mut bundle = Bundle::new(Manifest::new(page_id.clone(), pages));
    bundle.insert_entry(format!("{}.page", page_id), File::open(path).await?);
    Ok(bundle)
  }

  async fn run(self, mut rx: Receiver<GeneratorMsg>) {
    while let Some(msg) = rx.recv().await {
      match msg {
        GeneratorMsg::Generate { unit, prefix, sender } => {
          tokio::spawn(async move {
            let _ = sender.send(Self::generate(unit, prefix).await);
          });
        }
      }
    }
  }
}

impl Actor for CopyGenerator {
  type Msg = GeneratorMsg;

  fn spawn(self) -> Addr<Self::Msg> {
    let (addr, rx) = Addr::new();
    tokio::spawn(self.run(rx));
    addr
  }
}
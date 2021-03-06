use std::{collections::HashMap, path::Path};

use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct VirtualFile {
  content: Box<[u8]>
}

impl VirtualFile {
  pub fn open<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
    Ok(Self {
      content: std::fs::read(path)?.into_boxed_slice()
    })
  }

  pub fn content(&self) -> &[u8] {
    &self.content
  }

  pub fn content_mut(&mut self) -> &mut [u8] {
    &mut self.content
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocalFile {
  path: String
}

impl LocalFile {
  pub fn new<P: AsRef<Path>>(path: P) -> Self {
    Self {
      path: path.as_ref().to_string_lossy().to_string()
    }
  }

  pub fn path(&self) -> &Path {
    self.path.as_ref()
  }
}

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct LinkedFileHandle(u32);

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkedFile {
  handle: LinkedFileHandle
}

#[derive(Serialize, Deserialize, Debug)]
pub enum File {
  Virtual(VirtualFile),
  Local(LocalFile),
  Linked(LinkedFile)
}

impl From<VirtualFile> for File {
  fn from(value: VirtualFile) -> Self {
    Self::Virtual(value)
  }
}

impl From<LocalFile> for File {
  fn from(value: LocalFile) -> Self {
    Self::Local(value)
  }
}

impl From<LinkedFile> for File {
  fn from(value: LinkedFile) -> Self {
    Self::Linked(value)
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocalFolder {
  path: String
}

impl LocalFolder {
  pub fn new<P: AsRef<Path>>(path: P) -> Self {
    Self {
      path: path.as_ref().to_string_lossy().to_string()
    }
  }

  pub fn path(&self) -> &Path {
    self.path.as_ref()
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VirtualFolder {
  entries: HashMap<String, Entry>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Folder {
  Virtual(VirtualFolder),
  Local(LocalFolder)
}

impl Folder {
  
}

impl From<VirtualFolder> for Folder {
  fn from(value: VirtualFolder) -> Self {
    Self::Virtual(value)
  }
}

impl From<LocalFolder> for Folder {
  fn from(value: LocalFolder) -> Self {
    Self::Local(value)
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Entry {
  File(File),
  Folder(Folder)
}

impl From<File> for Entry {
  fn from(value: File) -> Self {
    Self::File(value)
  }
}

impl From<Folder> for Entry {
  fn from(value: Folder) -> Self {
    Self::Folder(value)
  }
}
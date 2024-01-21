use serde::{Serialize, Deserialize};

use crate::error::Result;
use std::{path::PathBuf, collections::{HashMap, BTreeMap}, fs::File, io::{Read, Seek, BufReader, Write, BufWriter, SeekFrom, self}};
pub struct KvStore {
    path: PathBuf,
    readers: HashMap<u64, BufReaderWithPos<File>>,
    writer: BufWriterWithPos<File>,
    current_gen: u64,
    index: BTreeMap<String, CommandPos>,
    uncompacted: u64,
}

impl KvStore {

    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {

        Ok(KvStore {
            path: path.into(),
            readers: HashMap::new(),
            writer: BufWriterWithPos::new(),
            current_gen: 0,
            index: BTreeMap::new(),
            uncompacted: 0
        })
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        Ok(Some("aaaa".into()))
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        Ok(())
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        Ok(())
    }
}

struct CommandPos {
    gen: u64,
    pos: u64,
    len: u64,
}

#[derive(Serialize, Deserialize, Debug)]
enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}

impl Command {
    fn set(key: String, value: String) -> Command {
        Command::Set { key, value }
    }

    fn remove(key: String) -> Command {
        Command::Remove { key }
    }
}

struct BufReaderWithPos<R: Read + Seek> {
    reader: BufReader<R>,
    pos: u64
}

impl<R: Read + Seek> BufReaderWithPos<R> {
    pub fn new(mut inner: R) -> Result<Self> {
        let pos = inner.seek(SeekFrom::Current(0))?;
        Ok(BufReaderWithPos {
            reader: BufReader::new(inner),
            pos,
        })
    }
}

impl<R: Read + Seek> Read for BufReaderWithPos<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let len = self.reader.read(buf)?;
        self.pos += len as u64;
        Ok(len)
    }
}

impl<R: Read + Seek> Seek for BufReaderWithPos<R> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.pos = self.reader.seek(pos)?;
        Ok(self.pos)
    }
}

struct BufWriterWithPos<W: Write + Seek> {
    writer: BufWriter<W>,
    pos: u64
}

impl<W: Write + Seek> BufWriterWithPos<W> {
    pub fn new(mut inner: W) -> Result<Self> {
        let pos = inner.seek(SeekFrom::Current(0))?;
        Ok(BufWriterWithPos { 
            writer: BufWriter::new(inner),
            pos
        })
    }
}

impl<W: Write + Seek> Write for BufWriterWithPos<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let delta = self.writer.write(buf)?;
        self.pos += delta as u64;
        Ok(delta)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

impl<W: Write + Seek> Seek for BufWriterWithPos<W> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.pos = self.writer.seek(pos)?;
        Ok(self.pos)
    }
}
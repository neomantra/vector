use crate::FilePosition;
use std::fs;
use std::io::{self, BufRead, Seek};
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use std::time;

/// The `FileWatcher` struct defines the polling based state machine which reads
/// from a file, updating the underlying file descriptor when the file has been
/// rolled over, as is common for logs.
///
/// The `FileWatcher` is expected to live for the lifetime of the file.
/// The `FileServer` is responsible for clearing away `FileWatchers` which no
/// longer exist.
pub struct FileWatcher {
    pub path: PathBuf,
    findable: bool,
    reader: Option<io::BufReader<fs::File>>,
    devno: u64,
    inode: u64,
}

impl FileWatcher {
    /// Create a new `FileWatcher`
    ///
    /// The input path will be used by `FileWatcher` to prime its state
    /// machine. A `FileWatcher` tracks _only one_ file. This function returns
    /// None if the path does not exist or is not readable by cernan.
    pub fn new(
        path: PathBuf,
        file_position: u64,
        ignore_before: Option<time::SystemTime>,
    ) -> Result<FileWatcher, io::Error> {
        let f = fs::File::open(&path)?;
        let metadata = f.metadata()?;
        let mut rdr = io::BufReader::new(f);

        let too_old = if let (Some(ignore_before), Ok(mtime)) = (ignore_before, metadata.modified())
        {
            mtime < ignore_before
        } else {
            false
        };

        if too_old {
            assert!(rdr.seek(io::SeekFrom::End(0)).is_ok());
        } else {
            assert!(rdr.seek(io::SeekFrom::Start(file_position)).is_ok());
        }

        Ok(FileWatcher {
            path: path,
            findable: true,
            reader: Some(rdr),
            devno: metadata.dev(),
            inode: metadata.ino(),
        })
    }

    pub fn update_path(&mut self, path: PathBuf) -> Result<(), io::Error> {
        assert!(self.reader.is_some());
        let metadata = fs::metadata(&path)?;
        let (devno, inode) = (metadata.dev(), metadata.ino());
        if (devno, inode) != (self.devno, self.inode) {
            let old_reader = self.reader.as_mut().unwrap();
            let position = old_reader.seek(io::SeekFrom::Current(0))?;
            let f = fs::File::open(&path)?;
            let mut new_reader = io::BufReader::new(f);
            new_reader.seek(io::SeekFrom::Start(position))?;
            self.reader = Some(new_reader);
            self.devno = devno;
            self.inode = inode;
        }
        self.path = path;
        Ok(())
    }

    pub fn set_file_findable(&mut self, f: bool) {
        self.findable = f;
    }

    pub fn file_findable(&self) -> bool {
        self.findable
    }

    pub fn get_file_position(&mut self) -> FilePosition {
        assert!(self.reader.is_some());
        let reader = self.reader.as_mut().unwrap();
        let position = reader.seek(io::SeekFrom::Current(0));
        assert!(position.is_ok());
        position.unwrap()
    }

    pub fn set_dead(&mut self) {
        self.reader = None;
    }

    pub fn dead(&self) -> bool {
        self.reader.is_none()
    }

    /// Read a single line from the underlying file
    ///
    /// This function will attempt to read a new line from its file, blocking,
    /// up to some maximum but unspecified amount of time. `read_line` will open
    /// a new file handler at need, transparently to the caller.
    pub fn read_line(&mut self, mut buffer: &mut Vec<u8>, max_size: usize) -> io::Result<usize> {
        //ensure buffer is re-initialized
        buffer.clear();
        assert!(self.reader.is_some());
        let reader = self.reader.as_mut().unwrap();
        match read_until_with_max_size(reader, b'\n', &mut buffer, max_size) {
            Ok(sz) => {
                if sz == 0 && !self.file_findable() {
                    self.set_dead();
                }
                Ok(sz)
            }
            Err(e) => {
                if let io::ErrorKind::NotFound = e.kind() {
                    self.set_dead();
                }
                Err(e)
            }
        }
    }
}

// Tweak of https://github.com/rust-lang/rust/blob/bf843eb9c2d48a80a5992a5d60858e27269f9575/src/libstd/io/mod.rs#L1471
// After more than max_size bytes are read as part of a single line, this discard the remaining bytes
// in that line, and then starts again on the next line.
fn read_until_with_max_size<R: BufRead + ?Sized>(
    r: &mut R,
    delim: u8,
    out_buffer: &mut Vec<u8>,
    max_size: usize,
) -> io::Result<usize> {
    let mut total_read = 0;
    let mut discarding = false;
    out_buffer.clear();
    loop {
        let in_buffer = match r.fill_buf() {
            Ok(buf) => buf,
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };

        let line_end_pos = memchr::memchr(delim, in_buffer);

        let end_read_pos = line_end_pos.unwrap_or(in_buffer.len());
        if !discarding {
            if out_buffer.len() + end_read_pos > max_size {
                warn!("Found line that exceeds max_line_bytes; discarding whole line.");
                discarding = true;
            } else {
                out_buffer.extend_from_slice(&in_buffer[..end_read_pos]);
            }
        }

        let line_ends = line_end_pos.is_some();
        let used = end_read_pos + if line_ends { 1 } else { 0 };
        r.consume(used);
        total_read += used;

        if line_ends && discarding {
            out_buffer.clear();
            discarding = false;
        } else if line_ends || used == 0 {
            return Ok(total_read);
        }
    }
}

#[cfg(test)]
mod test {
    use super::read_until_with_max_size;
    use std::io::Cursor;

    #[test]
    fn test_read_until_with_max_size() {
        let mut buf = Cursor::new(&b"12"[..]);
        let mut v = Vec::new();
        assert_eq!(
            read_until_with_max_size(&mut buf, b'3', &mut v, 1000).unwrap(),
            2
        );
        assert_eq!(v, b"12");

        let mut buf = Cursor::new(&b"1233"[..]);
        let mut v = Vec::new();
        assert_eq!(
            read_until_with_max_size(&mut buf, b'3', &mut v, 1000).unwrap(),
            3
        );
        assert_eq!(v, b"12");
        v.truncate(0);
        assert_eq!(
            read_until_with_max_size(&mut buf, b'3', &mut v, 1000).unwrap(),
            1
        );
        assert_eq!(v, b"");
        v.truncate(0);
        assert_eq!(
            read_until_with_max_size(&mut buf, b'3', &mut v, 1000).unwrap(),
            0
        );
        assert_eq!(v, []);

        let mut buf = Cursor::new(&b"short\nthis is too long\nexact size\n11 eleven11\n"[..]);
        let mut v = Vec::new();
        assert_eq!(
            read_until_with_max_size(&mut buf, b'\n', &mut v, 10).unwrap(),
            6
        );
        assert_eq!(v, b"short");
        v.truncate(0);
        assert_eq!(
            read_until_with_max_size(&mut buf, b'\n', &mut v, 10).unwrap(),
            28
        );
        assert_eq!(v, b"exact size");
        v.truncate(0);
        assert_eq!(
            read_until_with_max_size(&mut buf, b'\n', &mut v, 10).unwrap(),
            12
        );
        assert_eq!(v, []);
    }
}

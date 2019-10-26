use std::io::{SeekFrom, Seek, Read};
use std::fs::File;
use std::collections::VecDeque;
use std::iter::FromIterator;

use nom::IResult;

use crate::core::parsers::tbasket2vec;
use crate::tree_reader::container::Container;

/// An buffered reader which knows in advances which regions will be
/// seeked. It clusters adjacent regions together to reduce the number
/// of reads needed. If it is necessary to presere the number of bins
/// this book-keeping must be done elsewhere
#[derive(Debug)]
pub(crate) struct ClusteredBucketIter {
    /// Buckets for which data will be read, [start, len]
    seek_buckets: VecDeque<Container>,
    /// Data read from inner reader; this may span several bins
    buffer: Vec<u8>,
}

impl ClusteredBucketIter {
    pub fn new(containers: impl IntoIterator<Item=Container>) -> Self {
        let seek_buckets = VecDeque::from_iter(containers);
        Self {
            seek_buckets,
            buffer: vec![],
        }
    }
}

impl Iterator for ClusteredBucketIter {
    type Item=(u32, Vec<u8>);
    fn next(&mut self) -> Option<Self::Item>{
        match self.seek_buckets.pop_front()? {
            Container::InMemory(buf) => match tbasket2vec(&buf) {
                IResult::Done(_, v) => Some(v),
                _ => panic!("tbasket2vec parser failed"),
            },
            Container::OnDisk(next_path, SeekFrom::Start(next_seek), next_len) => {
                if next_len > self.buffer.len() {
                    // Refill buffer
                    let mut chunk_end = next_seek + next_len as u64;
                    for container in self.seek_buckets.iter() {
                        let mut cnt = 0;
                        while let Container::OnDisk(path, SeekFrom::Start(seek), len) = container {
                            // break once the following chunk is not adjecent to the current one
                            if *seek == chunk_end && next_path.eq(path) {
                                chunk_end += *len as u64;
                                cnt += 1;
                            } else {
                                break;
                            }
                        }
                        std::dbg!(cnt);
                    }
                    let mut f = File::open(&next_path).unwrap();
                    self.buffer.resize((chunk_end - next_seek) as usize, 0);
                    f.seek(SeekFrom::Start(next_seek)).unwrap();
                    f.read_exact(self.buffer.as_mut_slice()).unwrap();
                }
                // Do a little dance to keep the borrow checker happy
                let (s1, s2) = self.buffer.split_at(next_len as usize);
                let ret = match tbasket2vec(&s1) {
                    IResult::Done(_, v) => Some(v),
                    _ => panic!("tbasket2vec parser failed"),
                };
                self.buffer = s2.to_vec();
                ret
            }
            _ => panic!("Unexpected seek value"),
        }
    }
}

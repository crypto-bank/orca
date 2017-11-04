
extern crate sled;

use sled::Log;

fn main() {

    let log = sled::Config::default().log();
    let first_offset = log.write(b"1".to_vec());
    log.write(b"22".to_vec());
    log.write(b"333".to_vec());

    // stick an abort in the middle, which should not be returned
    let res = log.reserve(b"never_gonna_hit_disk".to_vec());
    res.abort();

    log.write(b"4444".to_vec());
    let last_offset = log.write(b"55555".to_vec());
    log.make_stable(last_offset);
    let mut iter = log.iter_from(first_offset);
    assert_eq!(iter.next().unwrap().1, b"1".to_vec());
    assert_eq!(iter.next().unwrap().1, b"22".to_vec());
    assert_eq!(iter.next().unwrap().1, b"333".to_vec());
    assert_eq!(iter.next().unwrap().1, b"4444".to_vec());
    assert_eq!(iter.next().unwrap().1, b"55555".to_vec());
    assert_eq!(iter.next(), None);
}

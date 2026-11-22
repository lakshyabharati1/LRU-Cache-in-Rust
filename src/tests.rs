use super::*;

#[test]
fn basic_insertion_test() {
    let mut cache = LruCache::new(3);

    cache.put(1, 10);
    cache.put(2, 20);
    cache.put(3, 30);

    assert_eq!(cache.get(1), Some(&10));
    assert_eq!(cache.get(2), Some(&20));
    assert_eq!(cache.get(3), Some(&30));

    cache.put(4, 40);

    assert_eq!(cache.get(1), None);
    assert_eq!(cache.get(4), Some(&40));

    cache.put(2, 200);

    assert_eq!(cache.get(2), Some(&200));

    cache.put(5, 50);
    cache.put(6, 60);

    assert_eq!(cache.get(5), Some(&50));
    assert_eq!(cache.get(6), Some(&60));

    assert_eq!(cache.get(2), Some(&200));
    assert_eq!(cache.get(2), Some(&200));
}

#[test]
fn test_duplicate_key_insertion() {
    let mut cache = LruCache::new(3);

    cache.put(1, 10);
    cache.put(2, 20);
    cache.put(1, 100);

    assert_eq!(cache.get(1), Some(&100));
    assert_eq!(cache.get(2), Some(&20));
}

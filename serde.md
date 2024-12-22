# Types supported by Serde

From:
https://docs.rs/serde/latest/serde/ser/index.html#implementations-of-serialize-provided-by-serde

## Primitive types

* X bool
* X i8, i16, i32, i64, i128, isize
* X u8, u16, u32, u64, u128, usize
* X f32, f64
* X char
* X str
* &str
* &T and &mut T

## Compound types

* [T]
* [T; 0] through [T; 32]
* tuples up to size 16

## Common standard library types

* X String
* X Option<T>
* X Result<T, E>
* X PhantomData<T>

## Wrapper types

* X Box<T>
* Cow<'a, T>
* X Cell<T>
* X RefCell<T>
* X Mutex<T>
* X RwLock<T>
* X Rc<T> (if features = [“rc”] is enabled)
* X Arc<T> (if features = [“rc”] is enabled)

## Collection types

* BTreeMap<K, V>
* BTreeSet<T>
* BinaryHeap<T>
* HashMap<K, V, H>
* HashSet<T, H>
* LinkedList<T>
* VecDeque<T>
* X Vec<T>

## FFI types

* CStr
* CString
* OsStr
* OsString

## Miscellaneous standard library types

* Duration
* SystemTime
* Path
* PathBuf
* Range<T>
* RangeInclusive<T>
* Bound<T>
* X num::NonZero*
* ! (unstable)

## Net types

* IpAddr
* Ipv4Addr
* Ipv6Addr
* SocketAddr
* SocketAddrV4
* SocketAddrV6

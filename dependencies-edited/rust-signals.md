[futures-signals](https://github.com/Pauan/rust-signals)
> last check date = 28/03/2026
- remove gensym
- fixed elide lifetime

## Cargo.toml:24
```diff
-    gensym = "0.1.0"
```

## /src/lib.rs:17
```diff
-    #[doc(hidden)]
-    pub use gensym::gensym as __internal_gensym;
```

## src/signals/macros.rs:36
```diff
-    #[doc(hidden)]
-    #[macro_export]
-    macro_rules! __internal_identifier {
-        ($gensym:ident, $macro:ident, { $($bindings:tt)* }, let $name:pat = $value:expr, $($rest:tt)+) => {{
-            let mut $gensym = $crate::internal::MapRef1::new($value);
-    
-            $crate::__internal_map!($macro, { $($bindings)* $gensym = $name, }, $($rest)+)
-        }};
-    
-        ($gensym:ident, $macro:ident, { $($bindings:tt)* }, let $name:pat = $value:expr => $($rest:tt)+) => {{
-            let mut $gensym = $crate::internal::MapRef1::new($value);
-    
-            $crate::__internal_map!($macro, { $($bindings)* $gensym = $name, }, => $($rest)+)
-        }};
-    
-        ($gensym:ident, $macro:ident, { $($bindings:tt)* }, $name:ident, $($rest:tt)+) => {{
-            let mut $gensym = $crate::internal::MapRef1::new($name);
-    
-            $crate::__internal_map!($macro, { $($bindings)* $gensym = $name, }, $($rest)+)
-        }};
-    
-        ($gensym:ident, $macro:ident, { $($bindings:tt)* }, $name:ident => $($rest:tt)+) => {{
-            let mut $gensym = $crate::internal::MapRef1::new($name);
-    
-            $crate::__internal_map!($macro, { $($bindings)* $gensym = $name, }, => $($rest)+)
-        }};
-    }
```

:70
```diff
    macro_rules! __internal_map {
        // This is only included for backwards compatibility
        // TODO remove in next major version
        ($macro:ident, { $($bindings:tt)* }, => move $f:expr) => {
-           $crate::__internal_map!($macro, { $($bindings)* }, => $f)
+           $crate::__internal_map!{$macro, { $($bindings)* }, => $f}
        };
```

:95
```diff
    })
        };
    
-       ($($rest:tt)*) => {
-           $crate::__internal_gensym!($crate::__internal_identifier!($($rest)*))
+       ($macro:ident, { $($bindings:tt)* }, let $name:pat = $value:expr, $($rest:tt)+) => {
+           let mut signal = $crate::internal::MapRef1::new($value);
+   
+           $crate::__internal_map!{$macro, { $($bindings)* signal = $name, }, $($rest)+}
+       };
+   
+       ($macro:ident, { $($bindings:tt)* }, let $name:pat = $value:expr => $($rest:tt)+) => {
+           let mut signal = $crate::internal::MapRef1::new($value);
+   
+           $crate::__internal_map!{$macro, { $($bindings)* signal = $name, }, => $($rest)+}
+       };
+   
+       ($macro:ident, { $($bindings:tt)* }, $name:ident, $($rest:tt)+) => {
+           let mut $name = $crate::internal::MapRef1::new($name);
+   
+           $crate::__internal_map!{$macro, { $($bindings)* $name = $name, }, $($rest)+}
+       };
+   
+       ($macro:ident, { $($bindings:tt)* }, $name:ident => $($rest:tt)+) => {
+           let mut $name = $crate::internal::MapRef1::new($name);
+   
+           $crate::__internal_map!{$macro, { $($bindings)* $name = $name, }, => $($rest)+}
        };
    }

:106
```diff
    /// `map_ref` instead.
    #[macro_export]
    macro_rules! map_mut {
-       ($($input:tt)*) => {
-           $crate::__internal_map!(__internal_value_mut, {}, $($input)*)
-       };
+       ($($input:tt)*) => {{
+           $crate::__internal_map!{__internal_value_mut, {}, $($input)*}
+       }};
    }
```

:304
```diff
    /// number of Signals. However, polling is ***very*** fast.
    #[macro_export]
    macro_rules! map_ref {
-       ($($input:tt)*) => {
-           $crate::__internal_map!(__internal_value_ref, {}, $($input)*)
-       };
+       ($($input:tt)*) => {{
+           $crate::__internal_map!{__internal_value_ref, {}, $($input)*}
+       }};
    }
```

fixed lifetime
## src/signal/mutable.rs

:75
```diff
    impl<A> ReadOnlyMutable<A> {
        // TODO return Result ?
        #[inline]
-       pub fn lock_ref(&self) -> MutableLockRef<A> {
+       pub fn lock_ref(&self) -> MutableLockRef<'_, A> {
            MutableLockRef {
                lock: self.0.lock.read().unwrap(),
            }
```

:310
```diff
    // TODO lots of unit tests to verify that it only notifies when the object is mutated
    // TODO return Result ?
    // TODO should this inline ?
-   pub fn lock_mut(&self) -> MutableLockMut<A> {
+   pub fn lock_mut(&self) -> MutableLockMut<'_, A> {
        MutableLockMut {
            mutated: false,
            lock: self.state().lock.write().unwrap(),
```

## src/signal_map.rs

:817
```diff
    // TODO return Result ?
    #[inline]
-   pub fn lock_ref(&self) -> MutableBTreeMapLockRef<K, V> {
+   pub fn lock_ref(&self) -> MutableBTreeMapLockRef<'_, K, V> {
        MutableBTreeMapLockRef {
            lock: self.0.read().unwrap(),
        }
```

: 825
```diff
    // TODO return Result ?
    #[inline]
-   pub fn lock_mut(&self) -> MutableBTreeMapLockMut<K, V> {
+   pub fn lock_mut(&self) -> MutableBTreeMapLockMut<'_, K, V> {
        MutableBTreeMapLockMut {
            lock: self.0.write().unwrap(),
        }
```
## src/signal_vec.rs

:3387
```diff
    // TODO return Result ?
    #[inline]
-   pub fn lock_ref(&self) -> MutableVecLockRef<A> {
+   pub fn lock_ref(&self) -> MutableVecLockRef<'_, A> {
        MutableVecLockRef {
            lock: self.0.read().unwrap(),
        }
```

:3395
```diff
    // TODO return Result ?
    #[inline]
-   pub fn lock_mut(&self) -> MutableVecLockMut<A> {
+   pub fn lock_mut(&self) -> MutableVecLockMut<'_, A> {
        MutableVecLockMut {
            lock: self.0.write().unwrap(),
        }
```

## tests/mutable_vec.rs
- change all `drop(v.truncate(`
```diff
-   is_eq(vec![], vec![], |v| drop(v.truncate(0)), vec![
+   is_eq(vec![], vec![], |v| {let _ = v.truncate(0);}, vec![
```

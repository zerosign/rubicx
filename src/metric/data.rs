use std::{default, fmt, hash, path::PathBuf};

#[derive(Debug)]
pub struct Size(u64, u64);

impl default::Default for Size {
    #[inline]
    fn default() -> Self {
        Size(500u64, 500u64)
    }
}

///
/// Index(a, b)
/// - a : index of segment file from segment pools
/// - b : cursor (seek) position of segment file
///
/// As representation of reverse index in b?-tree
///
#[derive(Debug)]
pub struct Index(u64, u64);

#[derive(Debug)]
pub struct Metadata {
    pub(crate) base: PathBuf,
    pub(crate) size: Size,
}

pub enum Value<T> {
    Found { data: Vec<T>, index: usize },
    NotFound,
}

pub struct Sparse<T>
where
    T: hash::Hash + Sized,
{
    index: Vec<usize>,
    data: Vec<T>,
}

pub struct Dense<T>
where
    T: hash::Hash + Sized,
{
    data: Vec<T>,
}

impl<T: hash::Hash + Sized> Sparse<T> {
    pub fn get(self, idx: usize) -> Value<T> {
        match self.index.binary_search(&idx).ok() {
            Some(idx) => Value::Found {
                data: self.data,
                index: idx,
            },
            None => Value::NotFound,
        }
    }

    pub fn with_index(index: Vec<usize>, data: Vec<T>) -> Sparse<T> {
        Sparse {
            index: index,
            data: data,
        }
    }

    pub fn size(self) -> usize {
        *self.index.iter().max().unwrap_or(&0)
    }
}

pub enum Data<T>
where
    T: Sized + hash::Hash,
{
    Sparse(Sparse<T>),
    Dense(Dense<T>),
}

macro_rules! sparse {
    ($($idx:expr => $value:expr),*) => {
        {
            let mut indexes = vec![];
            let mut values = vec![];

            $(
                indexes.push($idx);
                values.push($value);
            )*

            Sparse::new(indexes, values)
        }
    }
}

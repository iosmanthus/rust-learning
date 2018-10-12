use std::collections::HashSet;
use std::ops::Deref;

type Set = HashSet<usize>;

#[derive(PartialEq, Debug)]
pub struct MultiplicativeGroup {
    modulo: usize,
    set: Set,
}

impl MultiplicativeGroup {
    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }

    pub fn new(modulo: usize) -> Self {
        Self {
            modulo,
            set: (1..modulo).filter(|&x| Self::gcd(x, modulo) == 1).collect(),
        }
    }

    pub fn subgroup_of(&self, element: usize) -> Option<Self> {
        self.set.iter().find(|&&x| x == element).map(|&a| {
            let mut element = 1;
            let mut subgroup = Set::new();

            while {
                // do
                subgroup.insert(element);
                element = (element * a) % self.modulo;

                // while
                element != 1
            } {}

            Self {
                modulo: self.modulo,
                set: subgroup,
            }
        })
    }

    pub fn is_generator(&self, g: usize) -> bool {
        self.subgroup_of(g).map_or(false, |group| group == *self)
    }

    pub fn size(&self) -> usize {
        self.set.len()
    }
}

/*impl IntoIterator for MultiplicativeGroup {
    type Item = usize;
    type IntoIter = hash_set::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.set.into_iter()
    }
}

impl MultiplicativeGroup {
    pub fn iter(&self) -> hash_set::Iter<usize> {
        self.set.iter()
    }
}*/

impl Deref for MultiplicativeGroup {
    type Target = Set;
    fn deref(&self) -> &Self::Target {
        &self.set
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn constructor_checer() {
        let g = MultiplicativeGroup::new(8);
        assert_eq!(
            MultiplicativeGroup {
                modulo: 8,
                set: vec![1, 3, 5, 7].into_iter().collect()
            },
            g
        );
    }

    #[test]
    fn subgroup_of_checker() {
        let g = MultiplicativeGroup::new(8);
        assert_eq!(
            MultiplicativeGroup {
                modulo: 8,
                set: vec![1, 3].into_iter().collect()
            },
            g.subgroup_of(3).unwrap()
        );
        assert_eq!(0, g.size() % g.subgroup_of(3).unwrap().size());
    }

    #[test]
    fn is_generator_checker() {
        let g = MultiplicativeGroup::new(7);
        assert_eq!(true, g.is_generator(3));
        assert_eq!(false, g.is_generator(4));
    }

    #[test]
    fn size_checker() {
        let g = MultiplicativeGroup::new(1024);
        assert_eq!(256, g.subgroup_of(283).unwrap().size());
    }
}

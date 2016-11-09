pub struct UnionFind {
    parents: Vec<usize>,
    ranks: Vec<u64>
}

impl UnionFind {
    pub fn new(size: usize) -> UnionFind {
        UnionFind {
            parents: (0..size).map(|i| i).collect(),
            ranks: (0..size).map(|_| 0).collect()
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        let parent = self.parents[x];

        if parent != x {
            self.parents[x] = self.find(parent);
        }

        self.parents[x]
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let xr = self.find(x);
        let yr = self.find(y);

        if xr == yr {
            return;
        }

        if self.ranks[xr] < self.ranks[yr] {
            self.parents[xr] = yr;
        } else if self.ranks[xr] > self.ranks[yr] {
            self.parents[yr] = xr;
        } else {
            self.parents[yr] = xr;
            self.ranks[xr] += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut uf = UnionFind::new(10);
        uf.union(3, 5);
        uf.union(7, 8);
        uf.union(7, 9);
        uf.union(5, 9);

        assert!(uf.find(3) == uf.find(8));
        assert!(uf.find(3) != uf.find(6));
        assert!(uf.find(2) != uf.find(6));
    }
}

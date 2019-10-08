## designing with lifetimes

- figure out who owns the data
- create and know the barriers
- isolate static and dynamic parts of state
- restrict pub fn to super to maintain sanity
  - reduce api surface for modules
  - enforce a single point of entry
- have separate impl statements for &self, &mut self, self


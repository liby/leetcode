/*
 * @lc app=leetcode id=706 lang=rust
 *
 * [706] Design HashMap
 */

// @lc code=start
const TABLE_SIZE: usize = 32;

#[derive(Default)]i
struct MyHashMap {
    table: [Vec<(i32, i32)>; TABLE_SIZE],
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {

    fn new() -> Self {
        Default::default()
    }

    fn put(&mut self, key: i32, value: i32) {
        let i = (key as usize) % TABLE_SIZE;
        if let Some(j) = self.table[i].iter().position(|&(k, _)| k == key) {
            self.table[i][j].1 = value;
        } else {
            self.table[i].push((key, value));
        }
    }

    fn get(&self, key: i32) -> i32 {
        let i = (key as usize) % TABLE_SIZE;
        self.table[i]
            .iter()
            .position(|&(k, _)| k == key)
            .map(|j| self.table[i][j].1)
            .unwrap_or(-1)
    }

    fn remove(&mut self, key: i32) {
        let i = (key as usize) % TABLE_SIZE;
        if let Some(j) = self.table[i].iter().position(|&(k, _)| k == key) {
            self.table[i].remove(j);
        }
    }
}

/**
 * Your MyHashMap object will be instantiated and called as such:
 * let obj = MyHashMap::new();
 * obj.put(key, value);
 * let ret_2: i32 = obj.get(key);
 * obj.remove(key);
 */
// @lc code=end


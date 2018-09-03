use bit_vec::BitVec;
use std::collections::*;
use std::cmp::Ordering;


#[derive(Debug, PartialOrd, PartialEq, Eq)]
pub enum HuffmanTree {
    Node { freq: u32, l: Box<HuffmanTree>, r: Box<HuffmanTree> },
    Leaf { char: char, freq: u32 },
}

impl Ord for HuffmanTree {
    fn cmp(&self, other: &Self) -> Ordering {
        self.root_frequency().cmp(&other.root_frequency()).reverse()
    }
}

fn max_and_min(x: HuffmanTree, y: HuffmanTree) -> (HuffmanTree, HuffmanTree) {
    if x.root_frequency() >= y.root_frequency() {
        (x, y)
    } else {
        (y, x)
    }
}

impl HuffmanTree {
    pub fn encode(&self, s: &str) -> BitVec {
        let table = self.encode_code();
        let mut result = BitVec::new();
        for c in s.chars() {
            if let Some(ref v) = table.get(&c) {
                result.extend(v.iter());
            } else {
                panic!("key not found");
            }
        }
        result
    }

    pub fn decode(&self, from: &BitVec) -> String {
        let mut result = String::new();
        let mut _iter = from.iter();

        fn go<I: Iterator<Item=bool>>(t: &HuffmanTree, root: &HuffmanTree, bit: Option<bool>, from: &mut I, result: &mut String) {
            match t {
                &HuffmanTree::Leaf { char: c, .. } if bit.is_none() => {
                    result.push(c);
                    return;
                }
                &HuffmanTree::Leaf { char: c, .. } => {
                    result.push(c);
                    go(&root, &root, bit, from, result);
                }
                &HuffmanTree::Node { l: _, ref r, .. } if bit.unwrap() => {
                    go(&r, &root, from.next(), from, result);
                }

                &HuffmanTree::Node { ref l, r: _, .. } if !bit.unwrap() => {
                    go(&l, &root, from.next(), from, result);
                }
                &HuffmanTree::Node { .. } => unreachable!()
            }
        }
        go(&self, &self, _iter.next(), &mut _iter, &mut result);
        result
    }


    fn root_frequency(&self) -> u32 {
        match self {
            &HuffmanTree::Node { freq, .. } => freq,
            &HuffmanTree::Leaf { freq, .. } => freq,
        }
    }

    pub fn new(s: &str) -> HuffmanTree {
        let mut frequency: HashMap<char, u32> = HashMap::new();
        for c in s.chars() {
            if let Some(&i) = frequency.get(&c) {
                frequency.insert(c, i + 1);
            } else {
                frequency.insert(c, 1);
            }
        }
        let mut char_frequency: BinaryHeap<HuffmanTree> = frequency
            .into_iter()
            .map(|cf| HuffmanTree::Leaf { char: cf.0, freq: cf.1 })
            .collect();

        while let Some(first) = char_frequency.pop() {
            if let Some(second) = char_frequency.pop() {
                let f = first.root_frequency() + second.root_frequency();
                let (max, min) = max_and_min(first, second);

                let tree = HuffmanTree::Node {
                    freq: f,
                    l: Box::new(max),
                    r: Box::new(min),
                };
                char_frequency.push(tree);
            } else {
                return first;
            }
        }
        panic!("empty string");
    }

    pub fn encode_code(&self) -> HashMap<char, BitVec> {
        let mut codes: HashMap<char, BitVec> = HashMap::new();
        fn go(t: &HuffmanTree, code: BitVec, codes: &mut HashMap<char, BitVec>) {
            match t {
                &HuffmanTree::Leaf { char: c, .. } => {
                    codes.insert(c, code);
                    ()
                }
                &HuffmanTree::Node { ref l, ref r, .. } => {
                    let mut left_code = code.clone();
                    let mut right_code = code.clone();
                    left_code.push(false);
                    right_code.push(true);
                    go(&l, left_code, codes);
                    go(&r, right_code, codes);
                }
            }
        }
        go(self, BitVec::new(), &mut codes);
        codes
    }
}
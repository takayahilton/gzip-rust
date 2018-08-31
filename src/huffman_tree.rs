use std::collections::HashMap;

#[derive(Debug)]
pub enum HuffmanTree {
    Node { freq: u32, l: Box<HuffmanTree>, r: Box<HuffmanTree> },
    Leaf { char: char, freq: u32 },
}

impl HuffmanTree {

    fn frequency(&self) -> u32 {
        match self {
            &HuffmanTree::Node { freq, .. } => freq,
            &HuffmanTree::Leaf { freq, .. } => freq,
        }
    }

    fn sort_char_frequency_list(char_frequency: &mut Vec<HuffmanTree>) {
        char_frequency.sort_by_key(|cf| cf.frequency() )
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
        let mut char_frequency: Vec<HuffmanTree> = frequency
            .into_iter()
            .map(|cf| HuffmanTree::Leaf { char: cf.0, freq: cf.1 })
            .collect();

        HuffmanTree::sort_char_frequency_list(&mut char_frequency);

        while let Some(first) = char_frequency.pop() {
            if let Some(second) = char_frequency.pop() {
                let tree = HuffmanTree::Node {
                    freq: first.frequency() + second.frequency(),
                    l: Box::new(second),
                    r: Box::new(first)
                };
                char_frequency.push(tree);
                HuffmanTree::sort_char_frequency_list(&mut char_frequency);
            } else {
                return first;
            }
        }
        panic!("empty string");
    }
}
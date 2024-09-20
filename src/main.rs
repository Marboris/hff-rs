use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::io::{self, Read, Write};
use std::fs::File;


#[derive(Debug, Eq, PartialEq)]
enum HuffmanNode {
    Internal(Box<HuffmanNode>, Box<HuffmanNode>),
    Leaf(char, usize),
}

impl HuffmanNode {
    fn frequency(&self) -> usize {
        match self {
            HuffmanNode::Internal(left, right) => left.frequency() + right.frequency(),
            HuffmanNode::Leaf(_, freq) => *freq,
        }
    }
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.frequency().cmp(&self.frequency())
    }
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn build_huffman_tree(frequencies: &HashMap<char, usize>) -> Option<HuffmanNode> {
    let mut heap = BinaryHeap::new();

    for (ch, freq) in frequencies.iter() {
        heap.push(HuffmanNode::Leaf(*ch, *freq));
    }

    while heap.len() > 1 {
        let left = heap.pop()?;
        let right = heap.pop()?;
        heap.push(HuffmanNode::Internal(Box::new(left), Box::new(right)));
    }

    heap.pop()
}

fn generate_codes(node: &HuffmanNode, prefix: VecDeque<bool>, codes: &mut HashMap<char, VecDeque<bool>>) {
    match node {
        HuffmanNode::Internal(left, right) => {
            let mut left_prefix = prefix.clone();
            left_prefix.push_back(false);
            generate_codes(left, left_prefix, codes);

            let mut right_prefix = prefix.clone();
            right_prefix.push_back(true);
            generate_codes(right, right_prefix, codes);
        }
        HuffmanNode::Leaf(ch, _) => {
            codes.insert(*ch, prefix);
        }
    }
}

fn compress(data: &str) -> Option<(Vec<bool>, HashMap<char, VecDeque<bool>>)> {
    let mut frequencies = HashMap::new();

    for ch in data.chars() {
        *frequencies.entry(ch).or_insert(0) += 1;
    }

    let tree = build_huffman_tree(&frequencies)?;
    let mut codes = HashMap::new();
    generate_codes(&tree, VecDeque::new(), &mut codes);

    let mut compressed_data = Vec::new();
    for ch in data.chars() {
        let code = codes.get(&ch)?;
        compressed_data.extend(code.iter().cloned());
    }

    Some((compressed_data, codes))
}

fn to_bytes(bit_vector: &[bool]) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut byte = 0u8;
    let mut bit_count = 0;

    for bit in bit_vector {
        byte <<= 1;
        if *bit {
            byte |= 1;
        }
        bit_count += 1;

        if bit_count == 8 {
            bytes.push(byte);
            byte = 0;
            bit_count = 0;
        }
    }

    if bit_count > 0 {
        byte <<= 8 - bit_count;
        bytes.push(byte);
    }

    bytes
}

fn save_codes(file: &mut File, codes: &HashMap<char, VecDeque<bool>>) -> io::Result<()> {
    for (ch, code) in codes {
        let ch_byte = *ch as u8;
        let code_len = code.len() as u8;
        file.write_all(&[ch_byte, code_len])?;

        let code_bytes = to_bytes(&code.iter().cloned().collect::<Vec<_>>());
        file.write_all(&code_bytes)?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    match compress(&input) {
        Some((compressed_data, codes)) => {
            println!("compress: {:?}", compressed_data);
            
            // تبدیل داده‌های فشرده‌شده به بایت‌ها
            let compressed_bytes = to_bytes(&compressed_data);
            println!("Compressed data (bytes): {:?}", compressed_bytes);

            // ساخت فایل و نوشتن در آن
            let mut file = File::create("compressed_data.bin")?;

            // ذخیره کدهای هافمن در فایل
            save_codes(&mut file, &codes)?;

            // نوشتن داده‌های فشرده‌شده در فایل
            file.write_all(&compressed_bytes)?;

            println!("Compressed data saved to compressed_data.bin");
        }
        _none => {
            println!("Failed to compress data. Please check the input.");
        }
    }

    Ok(())
}

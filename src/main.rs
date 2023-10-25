use hex::encode;
use ring::{digest::{Context, SHA256}};

fn main() {
    let mut transactions: Vec<String> = Vec::new();
    transactions.push("Ayush".to_string());
    transactions.push("Amit".to_string());
    transactions.push("Aman".to_string());
    transactions.push("Shivam".to_string());
    transactions.push("Goldi barar".to_string());


    let merkle_root=generate_merkleroot(transactions);
    println!("{}",merkle_root);
}
pub fn generate_merkleroot(transactions:Vec<String>)->String
{
    let mut merkle_tree=transactions.clone();

    while merkle_tree.len() > 1 {
        let mut new_merkle_tree = Vec::new();

        for i in (0..merkle_tree.len()).step_by(2) {
            let left = &merkle_tree[i];
            let right = if i + 1 < merkle_tree.len() {
                &merkle_tree[i + 1]
            } else {
                left
            };
            let mut left_hash = Context::new(&SHA256);
            let mut right_hash = Context::new(&SHA256);

            left_hash.update(left.as_bytes());
            left_hash.update(right.as_bytes());


            let combined = format!("{}{}",encode(left_hash.finish()) ,encode(right_hash.finish()));
            
            let mut combined_hash=Context::new(&SHA256);
            combined_hash.update(combined.as_bytes());

            new_merkle_tree.push(encode(combined_hash.finish()))
        }

        merkle_tree = new_merkle_tree;
    }

    return merkle_tree[0].to_string();
}
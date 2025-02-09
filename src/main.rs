fn generate_msgblk(input_str: String) -> Vec<u8> {
    let mut msgblk = input_str.into_bytes();
    let msg_len = (msgblk.len() * 8).to_be_bytes();

    msgblk.push(128); // Push '1' to vector in the form of '10000000'

    while (msgblk.len() % 64) < 56 {
        msgblk.push(0); // Fill remainder of block with '00000000'
    }

    msgblk.extend_from_slice(&msg_len); // Append length (as BE u64) of inital input to end of message block

    /* for x in &msgblk {
            println!("{}", x)
        }

    println!("\n{}", msgblk.len()); */

    return msgblk
}

fn calculate_schedule(msgblk: Vec<u8>) {
    let mut sch = [0u32; 64];

    for i in (0..64).step_by(4) {
        sch[i / 4] = u32::from_be_bytes([msgblk[i], msgblk[i+1], msgblk[i+2], msgblk[i+3]]);
    }

    for (i, num) in sch.iter().enumerate() {
        println!("w{}: {:0>32b}", i, num);
    }
}

fn main() {
    let mut input = String::new();
    input.push_str("Hello World! Hello World! Hello World!");

    let msgblk = generate_msgblk(input);
    calculate_schedule(msgblk);
}

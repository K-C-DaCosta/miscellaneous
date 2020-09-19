fn main() {
    let n = 15u32; 
    print_hex(n);
    print_binary(n);
}

fn print_hex(mut num:u32){
    print!("0x");
    for _ in 0..8{
        let nibble = (num>>28)&0xff;
        // print!("nubble = {}\n",nibble);
        let digit;
        if nibble >=10{
            digit = ( ('a' as u8) + ((nibble-10) as u8) ) as char;
        }else{
            digit  = ('0' as u8 + nibble as u8 ) as char;
        }
        print!("{}",digit);
        //shift next nibble
        num<<=4;
    }
    print!("\n");
}

fn print_binary(mut num:u32){
    for _ in 0..32{
        let bit = (num>>31)&1;
        print!("{}",bit);
        num<<=1;
    }
    println!("b\n");
}

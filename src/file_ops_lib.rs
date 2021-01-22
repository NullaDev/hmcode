pub mod file_operator {
    use std::fs;
    use std::io::prelude::*;

    use crate::packet_lib::packet_handle;

    pub fn process_file_sample(filename: &str) {
        let data = fs::read(filename).expect("Some things happend when reading file.");
        let mut pak = packet_handle::handle_single_packet(&data);
        let packet_name_prefix = filename;
        let mut pak_file = fs::File::create(packet_name_prefix.to_string() + "_0.pak")
            .expect("Some things happend when creating file.");
        println!("Creating file done.");
        pak_file
            .write(&pak.to_raw_bytes())
            .expect("Some things happend when writing file.");
        println!("Writing file done.");
        println!("The packet info:\n{}", pak.info());
        println!("Now start to restore the packet.");
        println!(
            "The restore data is: \n {:?}",
            pak.to_real_bytes().expect("msg")
        );
    }
}

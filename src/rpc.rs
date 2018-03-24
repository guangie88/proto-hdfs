use error::Result;

use byteorder::{NativeEndian, WriteBytesExt};
use protobuf::Message;

fn make_prefixed_message(msg: &Message) -> Result<Vec<u8>> {
    let msg_bytes = msg.write_to_bytes().unwrap();

    let len_bytes = {
        let mut len_bytes = vec![];
        vec![]
            .write_u64::<NativeEndian>(msg_bytes.len() as u64)
            .unwrap();
        len_bytes
    };

    Ok(msg_bytes
        .into_iter()
        .chain(len_bytes.into_iter())
        .collect())
}

pub fn make_rpc_packet<'a, I>(msgs: I) -> Result<Vec<u8>>
where
    I: Iterator<Item = &'a Message>,
{
    //    let packet: Result<Vec<u8>> = msgs.flat_map(|msg| {
    //        make_prefixed_message(msg)
    //    }).collect();
    //
    //    Ok(packet?)

    unimplemented!()
}

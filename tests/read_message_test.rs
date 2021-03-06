extern crate messenger_plus;

use messenger_plus::stream;

use std::io;
use std::mem;

#[derive(Debug)]
struct RandomRead {
    info: Vec<u8>,
}

impl RandomRead {

    fn new(message: &str, num_payloads: i32) -> RandomRead {
        let mut data: Vec<u8> = Vec::new();
        for _ in 0..num_payloads {
            data.append(&mut Vec::from("--boundary"));
            data.append(&mut Vec::from(mem::size_of_val(message.as_bytes()).to_string().as_bytes()));
            data.append(&mut Vec::from("--"));
            data.append(&mut Vec::from(message));
            data.append(&mut Vec::from("--endboundary--"));
        }
        RandomRead {
            info: data,
        }
    }

}

impl io::Read for RandomRead {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        for i in 0..buf.len() {
            if self.info.is_empty() {
                return Ok(i as usize);
            }
            buf[i] = self.info.remove(0);
        }
        Ok(buf.len())
    }
}

#[test]
fn read_next_message_test() {
    let payload_one = "payload_one";
    let mut data = RandomRead::new(payload_one, 1);

    let mut message_reader: messenger_plus::stream::MessageReader<RandomRead> = messenger_plus::stream::MessageReader::new("--", "boundary", "endboundary", data, false);

    assert_eq!(message_reader.read_next_message(), Ok(Vec::from(payload_one)));
}

#[test]
fn special_characters_test() {
    let payload_one = "!@#$%^&*()_+-=[]{}|;:/?><";
    let mut data = RandomRead::new(payload_one, 1);
    let mut message_reader: messenger_plus::stream::MessageReader<RandomRead> = messenger_plus::stream::MessageReader::new("--", "boundary", "endboundary", data, false);

    assert_eq!(message_reader.read_next_message(), Ok(Vec::from(payload_one)));
}

#[test]
fn read_multiple_payloads_test() {
    let payload_one = "payload_one";
    let num_payloads = 3;
    let mut data = RandomRead::new(payload_one, num_payloads);

    let mut message_reader: messenger_plus::stream::MessageReader<RandomRead> = messenger_plus::stream::MessageReader::new("--", "boundary", "endboundary", data, false);

    for _ in 0..num_payloads {
        assert_eq!(message_reader.read_next_message(), Ok(Vec::from(payload_one)));
    }
}

#[test]
fn read_empty_payload_test() {
    let mut data = RandomRead::new("", 0);
    let mut message_reader: messenger_plus::stream::MessageReader<RandomRead> = messenger_plus::stream::MessageReader::new("--", "boundary", "endboundary", data, false);

    assert_eq!(message_reader.read_next_message(), Err(stream::Error::from(stream::ErrorKind::BufferEmpty)));
}
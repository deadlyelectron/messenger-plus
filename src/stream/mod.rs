pub mod read_stream;

#[cfg(test)]
mod tests {

    use super::*;

        #[test]
    fn vec_slice_begin_location() {
        let mut vec: Vec<u8> = Vec::new();
        vec.push(2);
        vec.push(13);
        vec.push(17);
        vec.push(18);

        let slice: &[u8] = &[2, 13];

        assert_eq!(read_stream::find_where_slice_begins(&vec, slice), Some(0));
    }

    #[test]
    fn vec_slice_begin_no_loc() {
        let mut vec: Vec<u8> = Vec::new();
        vec.push(2);
        vec.push(13);
        vec.push(17);
        vec.push(18);

        let slice: &[u8] = &[2, 2];

        assert_eq!(read_stream::find_where_slice_begins(&vec, slice), None);
    }

}
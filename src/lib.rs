use std::fmt;

const ONE_BYTE_BITS_COUNT: i8 = 8;

#[derive(Debug)]
struct BitArray {
    size: i64,
    bit_array: Vec<u8>,
}
#[derive(Clone, Debug)]
struct OutOfRangeError {
    bitarray_size: i64,
    bitarray_position: i64,
}

impl OutOfRangeError {
    fn new(bitarray_size: i64, bitarray_position: i64) -> Self {
        Self {
            bitarray_size,
            bitarray_position,
        }
    }
}

impl fmt::Display for OutOfRangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Given position: {} is out of the bitarray size {}.",
            self.bitarray_position, self.bitarray_size
        )
    }
}

impl BitArray {
    fn new(size: i64) -> Self {
        let _capacity: usize = (size / ONE_BYTE_BITS_COUNT as i64) as usize + 1;
        let mut bit_array: Vec<u8> = Vec::with_capacity(_capacity);

        for _ in 0.._capacity {
            bit_array.push(0);
        }

        Self { size, bit_array }
    }

    fn set(&mut self, position: i64, flag: bool) -> Result<(), OutOfRangeError> {
        if position >= self.size {
            Err(OutOfRangeError::new(self.size, position as i64))
        } else {
            let chunk: i64 = position as i64 / ONE_BYTE_BITS_COUNT as i64;
            let pow: i64 = position as i64 % ONE_BYTE_BITS_COUNT as i64;
            let offset: u8 = 2u64.pow(pow as u32) as u8;

            println!("chunk: {}, offset: {}, pow: {}", chunk, offset, pow);

            if flag {
                self.bit_array[chunk as usize] |= offset;
            } else {
                self.bit_array[chunk as usize] &= !offset;
            }

            Ok(())
        }
    }

    fn calc_vec_position(position: i64) -> i64 {
        position / ONE_BYTE_BITS_COUNT as i64
    }

    fn calc_byte_offset(position: i64) -> u8 {
        let _pow: i64 = position % ONE_BYTE_BITS_COUNT as i64;

        2u64.pow(_pow as u32) as u8
    }

    fn get(&self, position: i64) -> Result<bool, OutOfRangeError> {
        if position >= self.size {
            Err(OutOfRangeError::new(self.size, position))
        } else {
            let chunk: u64 = position as u64 / ONE_BYTE_BITS_COUNT as u64;
            let pow: u64 = position as u64 % ONE_BYTE_BITS_COUNT as u64;
            let offset: u8 = 2u64.pow(pow as u32) as u8;

            Ok(self.bit_array[chunk as usize] == (self.bit_array[chunk as usize] | offset))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BitArray, ONE_BYTE_BITS_COUNT};

    #[test]
    fn test_init_bitarray() {
        // Bit array size is a number of bits that will be allocated
        // in a given bitarray.
        // Bit array based on the vector of u8 integers
        // Each u8 integer occupy 8 bits of memory.
        // So the bitarray's vector should be of bitarray_size size / 8 size.
        let bitarray_size: i64 = 10;

        let bitarray: BitArray = BitArray::new(bitarray_size);

        assert_eq!(
            bitarray.bit_array.len(),
            (bitarray_size / ONE_BYTE_BITS_COUNT as i64) as usize + 1
        );
        assert_eq!(bitarray.bit_array.len(), 2);
    }

    #[test]
    fn test_bitarray_set_true() {
        let bitarray_size: i64 = 10;
        let bitarray_position: i64 = 9;

        let mut bitarray: BitArray = BitArray::new(bitarray_size);

        let success: bool = match bitarray.set(bitarray_position, true) {
            Ok(_) => true,
            Err(_) => false,
        };

        assert!(success);
    }

    #[test]
    fn test_bitarray_set_with_error() {
        let bitarray_size: i64 = 10;
        let bitarray_position: i64 = 10;

        let mut bitarray: BitArray = BitArray::new(bitarray_size);

        let success: bool = match bitarray.set(bitarray_position, true) {
            Ok(_) => false,
            Err(err) => {
                err.bitarray_position == 10
                    && err.bitarray_size == 10
                    && format!("{}", err) == "Given position: 10 is out of the bitarray size 10."
            }
        };

        assert!(success);
    }

    #[test]
    fn test_bitarray_get_with_error() {
        let bitarray_size: i64 = 10;
        let bitarray_position: i64 = 10;

        let bitarray: BitArray = BitArray::new(bitarray_size);

        let success: bool = match bitarray.get(bitarray_position) {
            Ok(_) => false,
            Err(err) => {
                err.bitarray_position == 10
                    && err.bitarray_size == 10
                    && format!("{}", err) == "Given position: 10 is out of the bitarray size 10."
            }
        };

        assert!(success);
    }

    #[test]
    fn test_bit_array_get_set() {
        let bitarray_size: i64 = 74845;

        let mut bitarray: BitArray = BitArray::new(bitarray_size);

        for bitarray_position in 0..bitarray_size - 1 {
            assert!(!bitarray.get(bitarray_position).unwrap());
        }

        for bitarray_position in 0..bitarray_size {
            bitarray.set(bitarray_position, true).unwrap();

            assert!(bitarray.get(bitarray_position).unwrap());
        }

        for bitarray_position in 0..bitarray_size - 1 {
            assert!(bitarray.get(bitarray_position).unwrap());
        }

        for bitarray_position in 0..bitarray_size {
            bitarray.set(bitarray_position, false).unwrap();

            assert!(!bitarray.get(bitarray_position).unwrap());
        }

        for bitarray_position in 0..bitarray_size - 1 {
            assert!(!bitarray.get(bitarray_position).unwrap());
        }
    }
}

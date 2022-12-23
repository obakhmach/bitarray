use std::fmt;

#[derive(Debug)]
struct BitArray {
    size: u64,
    bit_array: Vec<u8>,
}
#[derive(Clone, Debug)]
struct OutOfRangeError {
    bitarray_size: u64,
    bitarray_position: u64,
}

impl OutOfRangeError {
    fn new(bitarray_size: u64, bitarray_position: u64) -> Self {
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
            "Given position: {} is out of the bitarray size {}",
            self.bitarray_position, self.bitarray_size
        )
    }
}

impl BitArray {
    fn new(size: u64) -> Self {
        let _capacity: usize = (size / 8) as usize;
        let mut bit_array: Vec<u8> = Vec::with_capacity(_capacity);

        for _ in 0.._capacity {
            bit_array.push(0);
        }

        Self { size, bit_array }
    }

    fn set(&mut self, position: u64, flag: bool) -> Result<(), OutOfRangeError> {
        if position >= self.size {
            Err(OutOfRangeError::new(self.size, position))
        } else {
            let chunk: u64 = position / 8;
            let pow: u64 = position % 8;
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

    fn get(&self, position: u64) -> Result<bool, OutOfRangeError> {
        if position >= self.size {
            Err(OutOfRangeError::new(self.size, position))
        } else {
            let chunk: u64 = position / 8;
            let pow: u64 = position % 8;
            let offset: u8 = 2u64.pow(pow as u32) as u8;

            Ok(self.bit_array[chunk as usize] == (self.bit_array[chunk as usize] | offset))
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(true);
    }
}

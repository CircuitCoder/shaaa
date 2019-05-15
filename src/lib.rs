// TODO: use rust static evaluation to calculate
const ROTC: [u32; 25] = [0,1,62,28,27,36,44,6,55,20,3,10,43,25,39,41,45,15,21,8,18,2,61,56,14];
// const PFROM: [usize; 25] = [0,15,5,20,10,6,21,11,1,16,12,2,17,7,22,18,8,23,13,3,24,14,4,19,9];
const PFROM: [usize; 25] = [0,6,12,18,24,3,9,10,16,22,1,7,13,19,20,4,5,11,17,23,2,8,14,15,21];

const RNDC: [u64; 24] = [
    0x0000000000000001,
    0x0000000000008082,
    0x800000000000808a,
    0x8000000080008000,
    0x000000000000808b,
    0x0000000080000001,
    0x8000000080008081,
    0x8000000000008009,
    0x000000000000008a,
    0x0000000000000088,
    0x0000000080008009,
    0x000000008000000a,
    0x000000008000808b,
    0x800000000000008b,
    0x8000000000008089,
    0x8000000000008003,
    0x8000000000008002,
    0x8000000000000080,
    0x000000000000800a,
    0x800000008000000a,
    0x8000000080008081,
    0x8000000000008080,
    0x0000000080000001,
    0x8000000080008008,
];

pub trait Shaaa {
    fn update(&mut self, data: &[u8]);
    fn digest(self) -> Vec<u8>;
    fn digest_renew(&mut self) -> Vec<u8>;
}

macro_rules! shaaa_impl {
    ($name:ident, $B:expr, $R:expr, $D:expr) => {
        pub struct $name {
            state: [u64; 25], // Row major
            counter: usize,
            waiting: [u8; ($B - $D * 2) / 8],
        }

        impl $name {
            const RATE64: usize = ($B - $D * 2) / 64;
            const RATE8: usize = ($B - $D * 2) / 8;

            pub fn new() -> Self {
                let state = [0; $B / 64];
                Self {
                    state,
                    counter: 0,
                    waiting: unsafe { std::mem::uninitialized() },
                }
            }

            unsafe fn update_block(&mut self, block: *const u64) {
                let block: &[u64] = std::slice::from_raw_parts(block, Self::RATE64);

                let mut ptr = 0;
                
                // SIMD actually slows down the hashing, because the data is mostly unaligned
                /*
                while ptr + 8 <= Self::RATE64 {
                    let s = packed_simd::u64x8::from_slice_unaligned_unchecked(&self.state[ptr..]);
                    let b = packed_simd::u64x8::from_slice_unaligned_unchecked(&block[ptr..]);

                    let result = s ^ b;
                    result.write_to_slice_unaligned_unchecked(&mut self.state[ptr..]);

                    ptr += 8;
                }
                */

                while ptr < Self::RATE64 {
                    self.state[ptr] ^= block[ptr];
                    ptr += 1;
                }

                // Block permu
                self.permu();
            }

            pub fn update(&mut self, data: &[u8]) {
                let len = data.len();
                // Handle previously unfinished chunks
                let mut ptr = 0;


                if self.counter > 0 {
                    if self.counter + len < Self::RATE8 {
                        unsafe {
                            std::ptr::copy_nonoverlapping(
                                &data[0] as *const u8,
                                std::mem::transmute(&mut self.waiting[self.counter] as *mut u8),
                                len,
                            );
                        }

                        self.counter += len;

                        return;
                    }

                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            &data[0] as *const u8,
                            std::mem::transmute(&mut self.waiting[self.counter] as *mut u8),
                            Self::RATE8 - self.counter,
                        );
                    }

                    unsafe {
                        self.update_block(std::mem::transmute(&self.waiting[0]));
                    }

                    ptr = Self::RATE8 - self.counter;
                }

                loop {
                    if ptr + Self::RATE8 > len {
                        break;
                    }

                    unsafe {
                        self.update_block(std::mem::transmute(&data[ptr]));
                    }
                    ptr += Self::RATE8;
                }

                self.counter = len - ptr;

                if self.counter > 0 {
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            &data[ptr] as *const u8,
                            std::mem::transmute(&mut self.waiting[0] as *mut u8),
                            self.counter,
                            );
                    }
                }
            }

            fn pad(&mut self) {
                unsafe {
                    std::ptr::write_bytes(&mut self.waiting[self.counter] as *mut u8, 0, Self::RATE8 - self.counter);
                }

                // As specified, 0,1,1(0b110 = 0x06 on first bit) as delimiter for 2-bit sequence 0b01000000 ending for SHA3
                self.waiting[self.counter] ^= 0x06;
                self.waiting[Self::RATE8 - 1] ^= 0x80;

                unsafe {
                    self.update_block(std::mem::transmute(&self.waiting[0]));
                }
            }

            fn permu(&mut self) {
                for i in 0..$R {
                    if cfg!(feature = "internal") {
                        println!("======");
                        println!("Round {}", i);
                    }

                    self.round(RNDC[i]);
                }
            }

            fn round(&mut self, rndc: u64) {
                if cfg!(feature = "internal") {
                    println!("State:");
                    self.print_state();
                }

                // theta
                let mut xorcol: [u64; 5] = [0; 5];
                for i in 0..25 {
                    xorcol[i % 5] ^= self.state[i]; 
                }

                for i in 0..5 {
                    let val = xorcol[(i + 1) % 5].rotate_left(1) ^ xorcol[(i + 4) % 5];
                    for j in 0..5 {
                        self.state[i+j*5] ^= val;
                    }
                }

                if cfg!(feature = "internal") {
                    println!("After theta:");
                    self.print_state();
                }

                /*
                // rho, state[0] untouched
                for i in 1..25 {
                        self.state[i] = self.state[i].rotate_left(ROTC[i]);
                }

                // pi
                let mut transformed: [u64; 25] = unsafe { std::mem::uninitialized() };
                for i in 0..25 {
                    transformed[i] = self.state[PFROM[i]];
                }
                */

                // Rho + pi
                let mut transformed: [u64; 25] = unsafe { std::mem::uninitialized() };
                for i in 0..25 {
                    transformed[i] = self.state[PFROM[i]].rotate_left(ROTC[PFROM[i]]);
                }

                // chi
                // TODO: transmute state into [[u64; 5]; 5]
                for i in 0..5 {
                    for j in 0..5 {
                        self.state[i*5 + j] =
                            transformed[i * 5 + j]
                            ^ ((!transformed[i * 5 + (j+1)%5])
                               & transformed[i * 5 + (j+2)%5]);
                    }
                }

                // iota
                self.state[0] ^= rndc;

                if cfg!(feature = "internal") {
                    println!("After rho + pi + chi + iota");
                    self.print_state();
                }
            }

            // Consume self
            pub fn digest(mut self) -> [u8; $D / 8] {
                self.pad();

                const RESULT_LEN: usize = $D / 8;

                let mut result: [u8; RESULT_LEN] = unsafe { std::mem::uninitialized() };
                let mut start = 0 ;

                loop {
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            std::mem::transmute(&self.state[0] as *const u64),
                            &mut result[start] as *mut u8,
                            std::cmp::min(Self::RATE8, RESULT_LEN - start),
                        );
                    }

                    start += Self::RATE8;
                    if start >= RESULT_LEN {
                        break;
                    }

                    self.permu();
                }

                result
            }

            pub fn print_state(&self) {
                for i in 0..5 {
                    for j in 0..5 {
                        let transmuted: [u8; 8] = unsafe { std::mem::transmute(self.state[i * 5 + j]) };
                        for k in 0..8 {
                            print!("{:0>2x}", transmuted[k]);
                        }
                        print!(" ");
                    }
                    println!("");
                }
            }
        }

        impl Shaaa for $name {
            fn update(&mut self, data: &[u8]) {
                self.update(data);
            }
            fn digest(self) -> Vec<u8> {
                self.digest().to_vec()
            }
            fn digest_renew(&mut self) -> Vec<u8> {
                let inner = std::mem::replace(&mut *self, $name::new());
                inner.digest().to_vec()
            }
        }
    }
}

shaaa_impl!(Shaaa224, 1600, 24, 224);
shaaa_impl!(Shaaa256, 1600, 24, 256);
shaaa_impl!(Shaaa384, 1600, 24, 384);
shaaa_impl!(Shaaa512, 1600, 24, 512);

pub fn from_length(length: usize) -> Option<Box<Shaaa>> {
    match length {
        224 => Some(Box::new(Shaaa224::new())),
        256 => Some(Box::new(Shaaa256::new())),
        384 => Some(Box::new(Shaaa384::new())),
        512 => Some(Box::new(Shaaa512::new())),
        _ => None,
    }
}

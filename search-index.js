var searchIndex = JSON.parse('{\
"bitfrob":{"doc":"A crate to help with bit manipulation of integers.","t":"DDDDDDDDDDRRRLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFDLL","n":["U128BitIterHigh","U128BitIterLow","U16BitIterHigh","U16BitIterLow","U32BitIterHigh","U32BitIterLow","U64BitIterHigh","U64BitIterLow","U8BitIterHigh","U8BitIterLow","U8_SCALE_1_TO_8","U8_SCALE_2_TO_8","U8_SCALE_4_TO_8","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","clone","clone","clone","clone","clone","clone","clone","clone","cmp","default","eq","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","from","from","from","from","from","from_count_and_bits","from_count_and_bits","from_count_and_bits","from_count_and_bits","from_count_and_bits","from_count_and_bits","from_count_and_bits","from_count_and_bits","from_count_and_bits","from_count_and_bits","hash","high","into","into","into","into","into","into","into","into","into","into","into","into_iter","into_iter","into_iter","into_iter","into_iter","into_iter","into_iter","into_iter","into_iter","into_iter","low","next","next","next","next","next","next","next","next","next","next","partial_cmp","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","u128_get_bit","u128_get_region","u128_get_value","u128_region_mask","u128_replicate_bits","u128_with_bit","u128_with_region","u128_with_value","u16_get_bit","u16_get_region","u16_get_value","u16_region_mask","u16_replicate_bits","u16_with_bit","u16_with_region","u16_with_value","u32_get_bit","u32_get_region","u32_get_value","u32_region_mask","u32_replicate_bits","u32_with_bit","u32_with_region","u32_with_value","u64_get_bit","u64_get_region","u64_get_value","u64_region_mask","u64_replicate_bits","u64_with_bit","u64_with_region","u64_with_value","u8_bit_split_1x8","u8_bit_split_1x8_rev","u8_bit_split_2x4","u8_bit_split_2x4_rev","u8_bit_split_4x2","u8_bit_split_4x2_rev","u8_get_bit","u8_get_region","u8_get_value","u8_region_mask","u8_replicate_bits","u8_with_bit","u8_with_region","u8_with_value","u8x2","with_high","with_low"],"q":[[0,"bitfrob"]],"d":["Iterator for groups of bits in an integer (low to high).","Iterator for groups of bits in an integer (low to high).","Iterator for groups of bits in an integer (low to high).","Iterator for groups of bits in an integer (low to high).","Iterator for groups of bits in an integer (low to high).","Iterator for groups of bits in an integer (low to high).","Iterator for groups of bits in an integer (low to high).","Iterator for groups of bits in an integer (low to high).","Iterator for groups of bits in an integer (low to high).","Iterator for groups of bits in an integer (low to high).","When used as a multiplier, scales a “1 bit” <code>u8</code> to …","When used as a multiplier, scales a “2 bit” <code>u8</code> to …","When used as a multiplier, scales a “4 bit” <code>u8</code> to …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","Returns the argument unchanged.","Constructs a new iterator for <code>bits_per_iter</code> at a time out …","Constructs a new iterator for <code>bits_per_iter</code> at a time out …","Constructs a new iterator for <code>bits_per_iter</code> at a time out …","Constructs a new iterator for <code>bits_per_iter</code> at a time out …","Constructs a new iterator for <code>bits_per_iter</code> at a time out …","Constructs a new iterator for <code>bits_per_iter</code> at a time out …","Constructs a new iterator for <code>bits_per_iter</code> at a time out …","Constructs a new iterator for <code>bits_per_iter</code> at a time out …","Constructs a new iterator for <code>bits_per_iter</code> at a time out …","Constructs a new iterator for <code>bits_per_iter</code> at a time out …","","The upper byte","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","The lower byte","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Determines if the <code>b</code> bit is set in <code>u</code>.","Get the <code>low</code> to <code>high</code> bit region of <code>u</code>.","Get the <code>low</code> to <code>high</code> bit region of <code>u</code>, down shifted by <code>low</code>.","Generates a bit mask where all bits in the region are 1.","Replicates the lowest <code>count</code> bits across the entire value.","Replaces the <code>b</code> bit in <code>u</code>.","Replaces the <code>low</code> to <code>high</code> bit region of <code>old</code>.","Replaces the <code>low</code> to <code>high</code> bit region of <code>old</code> with an input …","Determines if the <code>b</code> bit is set in <code>u</code>.","Get the <code>low</code> to <code>high</code> bit region of <code>u</code>.","Get the <code>low</code> to <code>high</code> bit region of <code>u</code>, down shifted by <code>low</code>.","Generates a bit mask where all bits in the region are 1.","Replicates the lowest <code>count</code> bits across the entire value.","Replaces the <code>b</code> bit in <code>u</code>.","Replaces the <code>low</code> to <code>high</code> bit region of <code>old</code>.","Replaces the <code>low</code> to <code>high</code> bit region of <code>old</code> with an input …","Determines if the <code>b</code> bit is set in <code>u</code>.","Get the <code>low</code> to <code>high</code> bit region of <code>u</code>.","Get the <code>low</code> to <code>high</code> bit region of <code>u</code>, down shifted by <code>low</code>.","Generates a bit mask where all bits in the region are 1.","Replicates the lowest <code>count</code> bits across the entire value.","Replaces the <code>b</code> bit in <code>u</code>.","Replaces the <code>low</code> to <code>high</code> bit region of <code>old</code>.","Replaces the <code>low</code> to <code>high</code> bit region of <code>old</code> with an input …","Determines if the <code>b</code> bit is set in <code>u</code>.","Get the <code>low</code> to <code>high</code> bit region of <code>u</code>.","Get the <code>low</code> to <code>high</code> bit region of <code>u</code>, down shifted by <code>low</code>.","Generates a bit mask where all bits in the region are 1.","Replicates the lowest <code>count</code> bits across the entire value.","Replaces the <code>b</code> bit in <code>u</code>.","Replaces the <code>low</code> to <code>high</code> bit region of <code>old</code>.","Replaces the <code>low</code> to <code>high</code> bit region of <code>old</code> with an input …","Splits a byte into 1-bit chunks.","Splits a byte into 1-bit chunks (reversed).","Splits a byte into 2-bit chunks.","Splits a byte into 2-bit chunks (reversed).","Splits a byte into 4-bit chunks.","Splits a byte into 4-bit chunks (reversed).","Determines if the <code>b</code> bit is set in <code>u</code>.","Get the <code>low</code> to <code>high</code> bit region of <code>u</code>.","Get the <code>low</code> to <code>high</code> bit region of <code>u</code>, down shifted by <code>low</code>.","Generates a bit mask where all bits in the region are 1.","Replicates the lowest <code>count</code> bits across the entire value.","Replaces the <code>b</code> bit in <code>u</code>.","Replaces the <code>low</code> to <code>high</code> bit region of <code>old</code>.","Replaces the <code>low</code> to <code>high</code> bit region of <code>old</code> with an input …","Two <code>u8</code> values packed as a <code>u16</code>.","Updates the upper byte value, returning the new <code>u8x2</code>","Updates the lower byte value, returning the new <code>u8x2</code>"],"i":[0,0,0,0,0,0,0,0,0,0,0,0,0,1,2,3,4,5,6,7,8,9,10,11,1,2,3,4,5,6,7,8,9,10,11,1,2,3,4,5,6,7,8,9,10,11,11,11,11,1,2,3,4,5,6,7,8,9,10,11,1,2,3,4,5,6,7,8,9,10,11,11,1,2,3,4,5,6,7,8,9,10,11,11,1,2,3,4,5,6,7,8,9,10,11,1,2,3,4,5,6,7,8,9,10,11,1,2,3,4,5,6,7,8,9,10,11,1,2,3,4,5,6,7,8,9,10,11,1,2,3,4,5,6,7,8,9,10,11,1,2,3,4,5,6,7,8,9,10,11,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,11,11],"f":[0,0,0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[1,1],[2,2],[3,3],[4,4],[5,5],[6,6],[7,7],[8,8],[9,9],[10,10],[11,11],[[11,11],12],[[],11],[[11,11],13],[[1,14],15],[[2,14],15],[[3,14],15],[[4,14],15],[[5,14],15],[[6,14],15],[[7,14],15],[[8,14],15],[[9,14],15],[[10,14],15],[[11,14],15],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[[17,[16]]],11],[[]],[[18,16],1],[[18,19],2],[[18,18],3],[[18,20],4],[[18,21],5],[[18,16],6],[[18,19],7],[[18,18],8],[[18,20],9],[[18,21],10],[[11,22]],[11,16],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[11,16],[1,[[23,[16]]]],[2,[[23,[19]]]],[3,[[23,[18]]]],[4,[[23,[20]]]],[5,[[23,[21]]]],[6,[[23,[16]]]],[7,[[23,[19]]]],[8,[[23,[18]]]],[9,[[23,[20]]]],[10,[[23,[21]]]],[[11,11],[[23,[12]]]],[[],24],[[],24],[[],24],[[],24],[[],24],[[],24],[[],24],[[],24],[[],24],[[],24],[[],24],[[],24],[[],24],[[],24],[[],24],[[],24],[[],24],[[],24],[[],24],[[],24],[[],24],[[],24],[[],25],[[],25],[[],25],[[],25],[[],25],[[],25],[[],25],[[],25],[[],25],[[],25],[[],25],[[18,21],13],[[18,18,21],21],[[18,18,21],21],[[18,18],21],[[18,21],21],[[18,21,13],21],[[18,18,21,21],21],[[18,18,21,21],21],[[18,19],13],[[18,18,19],19],[[18,18,19],19],[[18,18],19],[[18,19],19],[[18,19,13],19],[[18,18,19,19],19],[[18,18,19,19],19],[[18,18],13],[[18,18,18],18],[[18,18,18],18],[[18,18],18],[[18,18],18],[[18,18,13],18],[[18,18,18,18],18],[[18,18,18,18],18],[[18,20],13],[[18,18,20],20],[[18,18,20],20],[[18,18],20],[[18,20],20],[[18,20,13],20],[[18,18,20,20],20],[[18,18,20,20],20],[16,[[17,[16]]]],[16,[[17,[16]]]],[16,[[17,[16]]]],[16,[[17,[16]]]],[16,[[17,[16]]]],[16,[[17,[16]]]],[[18,16],13],[[18,18,16],16],[[18,18,16],16],[[18,18],16],[[18,16],16],[[18,16,13],16],[[18,18,16,16],16],[[18,18,16,16],16],0,[[11,16],11],[[11,16],11]],"c":[],"p":[[3,"U8BitIterHigh"],[3,"U16BitIterHigh"],[3,"U32BitIterHigh"],[3,"U64BitIterHigh"],[3,"U128BitIterHigh"],[3,"U8BitIterLow"],[3,"U16BitIterLow"],[3,"U32BitIterLow"],[3,"U64BitIterLow"],[3,"U128BitIterLow"],[3,"u8x2"],[4,"Ordering"],[15,"bool"],[3,"Formatter"],[6,"Result"],[15,"u8"],[15,"array"],[15,"u32"],[15,"u16"],[15,"u64"],[15,"u128"],[8,"Hasher"],[4,"Option"],[4,"Result"],[3,"TypeId"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};

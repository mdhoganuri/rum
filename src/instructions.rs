use crate::memory::MemoryManager;

pub fn conditional_move (){
    // if $r[C] 6= 0 then $r[A] := $r[B]
}

pub fn segmented_load (){
    // $r[A] := $m[$r[B]][$r[C]]
}

pub fn segmented_store (){
    // $m[$r[A]][$r[B]] := $r[C]
}

pub fn addition (){
    // $r[A] := ($r[B] + $r[C]) mod 2^32
}

pub fn multiplication (){
    // $r[A] := ($r[B] * $r[C]) mod 2^32
}

pub fn division () {
    // $r[A] := ($r[B] ÷ $r[C]) (integer division)
}

pub fn bitwise_nand () {
    // $r[A] :=¬($r[B]∧$r[C])
}

pub fn halt () {
    // Computation stops
}

pub fn map_segment (mut memory: MemoryManager) {
    /* A new segment is created with a number of words
    equal to the value in $r[C]. Each word in the
    new segment is initialized to zero. A bit pattern
    that is not all zeroes and does not identify any
    currently mapped segment is placed in $r[B].
    */
    memory.allocate_memory();

}

pub fn unmap_segment () {
    /*The segment $m[$r[C]] is unmapped.
    Future Map Segment instructions may reuse the
    identifier $r[C].
    */
}

pub fn output () {
    /* The value in $r[C] is displayed on the I/O
    device immediately. Only values from 0 to 255
    are allowed
    */
}

pub fn input () {
    /* The UM waits for input on the I/O device. When
    input arrives, $r[c] is loaded with the input,
    which must be a value from 0 to 255. If the end
    of input has been signaled, then $r[C] is loaded
    with a full 32-bit word in which every bit is 1.
    */
}

pub fn load_program () {

}

pub fn load_value () {
    
}
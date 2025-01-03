/**
 * The Slice Type:
 * - Slices let us ref a contiguous sequence of elements in a collection rather than the whole
 * collection.
 * - A slice is a kind of ref, so it does not have ownership.
 */


fn first_word(s: &String) -> usize {
    // converting the string to an array of bytes 
    let bytes = s.as_bytes();

    // iter method returns each element in a collection and that enumerate wraps
    // the result of iter and returns each element as a part of a tuple instead.
    // The first element of the tuple returned from enumerate is the index, and 
    // second element is a ref to tha element.
    for (i, &item) bytes.iter().enumerate() {
        // search for the bytes that represents space by using byte literal syntax
        // If we find space return the position, otherwise return the length of the string 
        //
        if item == b' ' {
            return i;
        }
    }
    s.len()
}


fn main(){
    let mut s = String::from("Hello world");
:
    let word = first_word(&s); // it will get the value 5 
                               //
    s.clear(); // this empties the string, making it equal to ""
               // word still has the value 5 here, but there's no more string 
               // that we could meaningfully use the value 5 with.
               // So word is totally invalid now!
}

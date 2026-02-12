fn main () {
    let mut v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter_mut();

    while let Some(val) = v1_iter.next() {
        println!("{}", val);
    }

    print!()
}


// itrator

//.iter - when immutable and not sending ownership. just referencing.
// .iter_mut - when mut and not sending ownership. just referencing.
// iterInto - when sending ownership.
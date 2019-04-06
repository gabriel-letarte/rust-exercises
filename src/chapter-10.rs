/*
 * Basic example taken from https://doc.rust-lang.org/book/ch10-00-generics.html#removing-duplication-by-extracting-a-function
 * */
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/*
 * Char implements the Copy trait which allow us to return the
 * biggest char instead of a reference to it.
 * https://doc.rust-lang.org/beta/std/marker/trait.Copy.html
 *
 * Note, this is very inefficient as it will copy the value
 * instead of keeping a reference to it.
*/
fn largest_char_copy(list: &[char]) -> char {
    // Largest copies the first char of the list
    let mut largest: char = list[0];

    // iter() will return pointers to items
    for item in list.iter() {
        // We deference item (char reference) to compare it to an actual char
        if *item > largest {
            // We deference item and copy it's value in largest
            largest = *item;
        }
    }

    // Return an actual char
    largest
}

/*
 * Same implementation but using ref to chars
 * */
fn largest_char_ref(list: &[char]) -> &char {
    // Largest copies the first char of the list
    let mut largest: &char = &list[0];

    // iter() will return pointers to items
    for item in list.iter() {
        // We deference item (char reference) to compare it to an actual char
        if item > largest {
            // We deference item and copy it's value in largest
            largest = item;
        }
    }

    // Return an actual char
    largest
}

/*
 * Implementation over a list of generic implementing PartialOrd trait
 * */
fn largest_generic<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest: &T = &list[0];

    // Item is a reference to a &T because .iter() return references to &T
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/*
 * Implementation over a list of generic implementing PartialOrd + Copy trait
 * Return an actual T instead of a reference to a T
 * */
fn largest_generic_copy<T:PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for item in list.iter() {
        if *item > largest {
            largest = *item;
        }
    }

    largest
}

fn main() {
    // Using primitives
    let number_list = vec![34, 50, 25, 100, 65];
    let result: i32 = largest_i32(&number_list);
    println!("largest_i32 is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result: char = largest_char_copy(&char_list);
    println!("largest_char_copy is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result: &char = largest_char_ref(&char_list);
    println!("largest_char_ref is {}", *result);

    // Using generics
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result: &char = largest_generic(&char_list);
    println!("largest_generic is {}", *result);

    let number_list = vec![34, 50, 25, 100, 65];
    let result: &i32 = largest_generic(&number_list);
    println!("largest_generic is {}", *result);

    // Using generics + copy
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result: char = largest_generic_copy(&char_list);
    println!("largest_generic is {}", result);

    let number_list = vec![34, 50, 25, 100, 65];
    let result: i32 = largest_generic_copy(&number_list);
    println!("largest_generic is {}", result);
}

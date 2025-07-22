fn main() {
    // You can optionally experiment here.
    let mut numbers = [1, 2, 3];
    {
        let num_selected = &mut numbers;   

        num_selected[0] = 5;
    }

    println!("Numbers {numbers:?}");
    //println!("Number selected {num_selected:?}");

    //num_selected[2] = 5;
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);

        // TODO: Use a tuple index to access the second element of `numbers`
        // and assign it to a variable called `second`.
        let second = numbers.1;

        assert_eq!(second, 2, "This is not the 2nd number in the tuple!");
    }
}

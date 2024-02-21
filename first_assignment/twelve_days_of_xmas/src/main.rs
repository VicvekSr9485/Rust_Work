fn main() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three French hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];

    println!("The Twelve Days of Christmas:");

    for day in 0..12 {
        println!("On the {} day of Christmas", days[day]);
        println!("My true love sent to me:");

        for j in (0..=day).rev() {
            println!("{}", gifts[j]);
        }

        println!();
    }
}

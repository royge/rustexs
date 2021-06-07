pub fn lyrics() -> String {
    let mut lyrics = String::new();

    let days = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth",
    ];

    let all_gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a laying",
        "Seven swans a swimming",
        "Eight maids a milking",
        "Nine ladies dancing",
        "Ten lords a leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let mut day_count = 0;

    for day in days.iter() {
        lyrics.push_str("On the ");
        lyrics.push_str(day);
        lyrics.push_str(" day of Christmas my true love gave to me\n");

        let gifts = &all_gifts[..day_count+1];
        let mut gift_count = 0;

        for gift in gifts.iter().rev() {
            if is_1st_gift(day_count, gift_count) {
                lyrics.push_str(" and\n");
            }

            lyrics.push_str(gift);

            if is_before_2nd_gift(day_count, gift_count) {
                lyrics.push_str("\n");
            }

            gift_count += 1;
        }

        day_count += 1;

        if is_next_day(day_count, days.len()) {
            lyrics.push_str("\n\n");
        }
    }

    return lyrics;
}

fn is_1st_gift(day_count:usize, gift_count:usize) -> bool {
    day_count > 0 && day_count == gift_count
}

fn is_next_day(day_count:usize, total_days: usize) -> bool {
    day_count < total_days
}

fn is_before_2nd_gift(day_count:usize, gift_count:usize) -> bool {
    day_count > 1 && gift_count < day_count - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let song = "On the first day of Christmas my true love gave to me
A partridge in a pear tree

On the second day of Christmas my true love gave to me
Two turtle doves and
A partridge in a pear tree

On the third day of Christmas my true love gave to me
Three French hens
Two turtle doves and
A partridge in a pear tree

On the fourth day of Christmas my true love gave to me
Four calling birds
Three French hens
Two turtle doves and
A partridge in a pear tree

On the fifth day of Christmas my true love gave to me
Five gold rings
Four calling birds
Three French hens
Two turtle doves and
A partridge in a pear tree

On the sixth day of Christmas my true love gave to me
Six geese a laying
Five gold rings
Four calling birds
Three French hens
Two turtle doves and
A partridge in a pear tree

On the seventh day of Christmas my true love gave to me
Seven swans a swimming
Six geese a laying
Five gold rings
Four calling birds
Three French hens
Two turtle doves and
A partridge in a pear tree

On the eighth day of Christmas my true love gave to me
Eight maids a milking
Seven swans a swimming
Six geese a laying
Five gold rings
Four calling birds
Three French hens
Two turtle doves and
A partridge in a pear tree

On the ninth day of Christmas my true love gave to me
Nine ladies dancing
Eight maids a milking
Seven swans a swimming
Six geese a laying
Five gold rings
Four calling birds
Three French hens
Two turtle doves and
A partridge in a pear tree

On the tenth day of Christmas my true love gave to me
Ten lords a leaping
Nine ladies dancing
Eight maids a milking
Seven swans a swimming
Six geese a laying
Five gold rings
Four calling birds
Three French hens
Two turtle doves and
A partridge in a pear tree

On the eleventh day of Christmas my true love gave to me
Eleven pipers piping
Ten lords a leaping
Nine ladies dancing
Eight maids a milking
Seven swans a swimming
Six geese a laying
Five gold rings
Four calling birds
Three French hens
Two turtle doves and
A partridge in a pear tree

On the twelfth day of Christmas my true love gave to me
Twelve drummers drumming
Eleven pipers piping
Ten lords a leaping
Nine ladies dancing
Eight maids a milking
Seven swans a swimming
Six geese a laying
Five gold rings
Four calling birds
Three French hens
Two turtle doves and
A partridge in a pear tree";

        assert_eq!(song, lyrics().as_str());
    }
}

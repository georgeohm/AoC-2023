fn main() {
    let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";


    let start = input.lines()
        .next().unwrap()
        .split(" ")
        .skip(1)
        .map(|num| num.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| RangeState::Unscored(chunk.get(0).unwrap().clone(), chunk.get(1).unwrap().clone()));

    let maps = input.split("/n/n")
        .skip(1)
        .map(|group| {
            group.lines()
                .skip(1)
                .map(|val| {
                    let values: Vec<u32>  = val.split_whitespace()
                        .map(|num| num.parse::<u32>().unwrap())
                        .collect();

                    RangeMapping {
                        dest_val: values.get(0).unwrap().clone(),
                        src_val: values.get(1).unwrap().clone(),
                        range: values.get(2).unwrap().clone(),
                    }
                })
        });

    let out = maps.fold(start, |ranges, mappings| {
    });

    println!("Solution: {}", out);
}

enum RangeState {
    Unscored(i32, i32),
    Scored((i32, i32)),
}



struct RangeMapping {
    dest_val: u32,
    src_val: u32,
    range: u32
}


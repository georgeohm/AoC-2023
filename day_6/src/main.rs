fn main() {
    let input = "Time:        40     81     77     72
Distance:   219   1012   1365   1089";
    let test =input.lines().next().unwrap()
        .replace(" ", "");


    let times: Vec<f64> = input.lines().next().unwrap()
        .replace(" ", "")
        .split(":")
        .skip(1)
        .map(|num| num.parse::<f64>().unwrap())
        .collect();

    let distances: Vec<f64> = input.lines().skip(1).next().unwrap()
        .replace(" ", "")
        .split(":")
        .skip(1)
        .map(|num| num.parse::<f64>().unwrap())
        .collect();

    let out: u32 = times.iter().zip(distances)
        .map(|(time, distance)|  (time - 0.0001, distance))
        .flat_map(|(time, distance)| {
            // h (t - h) =  m
            // h (t - h) =  m
            // 0 = h^2 - ht + m
            // 0 = hold^2 - hold * time + distance
            let x = time * time - 4.0 * distance;
            if x < 0.0 { return None; }

            let min = (time - x.sqrt()) / 2.0;
            let max = (time + x.sqrt()) / 2.0;

            let min = min.max(0.0).ceil() as u32;
            let max = max.min(time).floor() as u32;

            println!("Time {} Distance {} Max {} Min {}", time, distance, max, min);
            Some(max - min + 1)
        }).product();

    println!("Hello, world!{}", out);
}

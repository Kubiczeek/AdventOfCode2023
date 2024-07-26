use std::fs;

fn main() {
    let contents = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");
    let lines = contents.lines();
    let mut seeds: Vec<i64> = Vec::new();
    let mut seed_to_soil_map: Vec<Vec<i64>> = Vec::new();
    let mut soil_to_fertilizer_map: Vec<Vec<i64>> = Vec::new();
    let mut fertilizer_to_water_map:  Vec<Vec<i64>> = Vec::new();
    let mut water_to_light_map:  Vec<Vec<i64>> = Vec::new();
    let mut light_to_temperature_map:  Vec<Vec<i64>> = Vec::new();
    let mut temperature_to_humidity_map:  Vec<Vec<i64>> = Vec::new();
    let mut humidity_to_location_map:  Vec<Vec<i64>> = Vec::new();
    let mut temp_vec:  Vec<Vec<i64>> = Vec::new();
    let mut lowest: Vec<i64> = Vec::new();

    //Gather input
    for line in lines {
        let mut l = line;
        if l.contains("seeds: ") {
            l = l.split("seeds: ").collect::<Vec<&str>>()[1];
            seeds = l.split(" ").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        } else if l.contains("soil-to-fertilizer map:"){
            seed_to_soil_map = temp_vec.clone();
            temp_vec.clear();
        } else if l.contains("soil-to-fertilizer map:") {
            soil_to_fertilizer_map = temp_vec.clone();
            temp_vec.clear();
        } else if l.contains("fertilizer-to-water map:") {
            soil_to_fertilizer_map = temp_vec.clone();
            temp_vec.clear();
        } else if l.contains("water-to-light map:") {
            fertilizer_to_water_map = temp_vec.clone();
            temp_vec.clear();
        } else if l.contains("light-to-temperature map:") {
            water_to_light_map = temp_vec.clone();
            temp_vec.clear();
        } else if l.contains("temperature-to-humidity map:") {
            light_to_temperature_map = temp_vec.clone();
            temp_vec.clear();
        } else if l.contains("humidity-to-location map:") {
            temperature_to_humidity_map = temp_vec.clone();
            temp_vec.clear();
        } else if l.contains("END") {
            humidity_to_location_map = temp_vec.clone();
            temp_vec.clear();

        } else if !l.contains("map:") && l != "" {
            temp_vec.push(l.split(" ").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>());
        }
    }

    //DEST SOURCE RANGE
    for mut seed in seeds {
        for s_t_s in &seed_to_soil_map {
            let d = s_t_s[0];
            let s = s_t_s[1];
            let r = s_t_s[2];
            let mut diff = 0;
            if seed >= s && seed < s + r {
                diff = d - s;
            }
            seed = seed + diff;
            if diff != 0 {
                break;
            }
        }

        for s_t_f in &soil_to_fertilizer_map {
            let d = s_t_f[0];
            let s = s_t_f[1];
            let r = s_t_f[2];
            let mut diff = 0;
            if seed >= s && seed < s+r {
                diff = d-s;
            }
            seed = seed + diff;
            if diff != 0 {
                break;
            }
        }

        for f_t_w in &fertilizer_to_water_map {
            let d = f_t_w[0];
            let s = f_t_w[1];
            let r = f_t_w[2];
            let mut diff = 0;
            if seed >= s && seed < s+r {
                diff = d-s;
            }
            seed = seed + diff;
            if diff != 0 {
                break;
            }
        }

        for w_t_l in &water_to_light_map {
            let d = w_t_l[0];
            let s = w_t_l[1];
            let r = w_t_l[2];
            let mut diff = 0;
            if seed >= s && seed < s+r {
                diff = d-s;
            }
            seed = seed + diff;
            if diff != 0 {
                break;
            }
        }

        for l_t_t in &light_to_temperature_map {
            let d = l_t_t[0];
            let s = l_t_t[1];
            let r = l_t_t[2];
            let mut diff = 0;
            if seed >= s && seed < s+r {
                diff = d-s;
            }
            seed = seed + diff;
            if diff != 0 {
                break;
            }
        }

        for t_t_h in &temperature_to_humidity_map {
            let d = t_t_h[0];
            let s = t_t_h[1];
            let r = t_t_h[2];
            let mut diff = 0;
            if seed >= s && seed < s+r {
                diff = d-s;
            }
            seed = seed + diff;
            if diff != 0 {
                break;
            }
        }

        for h_t_l in &humidity_to_location_map {
            let d = h_t_l[0];
            let s = h_t_l[1];
            let r = h_t_l[2];
            let mut diff = 0;
            if seed >= s && seed < s+r {
                diff = d-s;
            }
            seed = seed + diff;
            if diff != 0 {
                break;
            }
        }

        lowest.push(seed);
    }
    println!("{:?}", lowest);
    println!("{:?}", lowest.iter().min().unwrap());
}
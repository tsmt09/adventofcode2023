let input = input.split("\n");
let times = input[0];
let distances = input[1];

times.replace("Time:", "");
times = split_trim(times);
distances.replace("Distance:", "");
distances = split_trim(distances);
let len = len(times) - 1;
let prod = 1;

while len >= 0 {
    let high = floor(get_time_by_distance_high(distances[len]+1, times[len]));
    let low = ceiling(get_time_by_distance_low(distances[len]+1, times[len]));
    let solutions = high - low + 1;
    print("Race " + len + " solutions: " + solutions + ". Low: " + low + ". High: " + high);
    prod = prod * solutions;
    len -= 1;
}

print("result: " + prod);
// PART 2
// 
let times = input[0];
let distances = input[1];
times.replace("Time:", "");
times.replace(" ", "");
let time = parse_int(times);
distances.replace("Distance:", "");
distances.replace(" ", "");
let distance = parse_int(distances);

let high = floor(get_time_by_distance_high(distance, time));
let low = ceiling(get_time_by_distance_low(distance, time));
let solutions = high - low + 1;
print("solution: " + solutions + ". Low: " + low + ". High: " + high);


fn get_time_by_distance_high(distance, racetime) {
    0.5 * (sqrt((racetime*racetime) - 4.0*distance) + racetime) 
}
fn get_time_by_distance_low(distance, racetime) {
    0.5 * (racetime - sqrt((racetime*racetime) - 4.0*distance)) 
}
fn split_trim(input) {
    input.trim();
    input = input.split(" ");
    input = input.filter(|x| x != "");
    input = input.map(|x| parse_int(x));
    input
}
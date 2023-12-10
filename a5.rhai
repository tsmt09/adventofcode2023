fn get_map(lines, start, end) {
    let map = [];
    let index = start + 1;
    while index < end {
        let arr = lines[index].split(" ");
        arr = arr.map(|x| parse_int(x));
        map.push(arr);
        index += 1;
    }
    map.sort(|x,y| x[1] - y[1]);
    map
}

fn get_mapped(map, number) {
    // print(map);
    for arr in map {
        if number >= arr[1] && number < (arr[1] + arr[2]) {
            return arr[0] + (number - arr[1]);
        }
    }
    return number
}

fn map_number(num) {
    let num = get_mapped(global::s2s, num);
    let num = get_mapped(global::s2f, num);
    let num = get_mapped(global::f2w, num);
    let num = get_mapped(global::w2l, num);
    let num = get_mapped(global::l2t, num);
    let num = get_mapped(global::t2h, num);
    get_mapped(global::h2l, num)
}


print("AOC A5.1 - RHAI");
let lines = input.split('\n');
lines.retain(|line| line != "");
let seeds = lines[0];
seeds.replace("seeds: ", "");
seeds.trim();
seeds = seeds.split(" ");
seeds = seeds.map(|x| parse_int(x));
print(seeds);
let s2s_index = lines.index_of(|x| x.starts_with("seed-to-soil"));
let s2f_index = lines.index_of(|x| x.starts_with("soil"));
let f2w_index = lines.index_of(|x| x.starts_with("fert"));
let w2l_index = lines.index_of(|x| x.starts_with("wate"));
let l2t_index = lines.index_of(|x| x.starts_with("ligh"));
let t2h_index = lines.index_of(|x| x.starts_with("temp"));
let h2l_index = lines.index_of(|x| x.starts_with("humi"));
print("starting lines: s2s:" +  s2s_index + ",s2f:" +  s2f_index + ",f2w:" +  f2w_index + ",w2l:" +  w2l_index + ",l2t:" +  l2t_index + ",t2h:" +  t2h_index + ",h2l:" +  h2l_index);
const s2s = get_map(lines, s2s_index, s2f_index);
const s2f = get_map(lines, s2f_index, f2w_index);
const f2w = get_map(lines, f2w_index, w2l_index);
const w2l = get_map(lines, w2l_index, l2t_index);
const l2t = get_map(lines, l2t_index, t2h_index);
const t2h = get_map(lines, t2h_index, h2l_index);
const h2l = get_map(lines, h2l_index, len(lines));

let results = seeds.map(|num| {
    map_number(num)
});

results.sort(|x,y| x - y);
print("result: " + results[0]);

/////
// PART 2
/////

print("AOC A5.2 - RHAI");

fn get_map_rev(lines, start, end) {
    let map = [];
    let index = start + 1;
    while index < end {
        let arr = lines[index].split(" ");
        arr = arr.map(|x| parse_int(x));
        map.push(arr);
        index += 1;
    }
    map.sort(|x,y| x[0] - y[0]);
    map
}

fn get_mapped_reverse(map, number) {
    // print(map);
    for arr in map {
        if number >= arr[0] && number < (arr[0] + arr[2]) {
            return arr[1] + (number - arr[0]);
        }
    }
    return number
}

fn map_number_reverse(num) {
    num = get_mapped_reverse(global::rh2l, num);
    num = get_mapped_reverse(global::rt2h, num);
    num = get_mapped_reverse(global::rl2t, num);
    num = get_mapped_reverse(global::rw2l, num);
    num = get_mapped_reverse(global::rf2w, num);
    num = get_mapped_reverse(global::rs2f, num);
    get_mapped_reverse(global::rs2s, num)
}

const rs2s = get_map_rev(lines, s2s_index, s2f_index);
const rs2f = get_map_rev(lines, s2f_index, f2w_index);
const rf2w = get_map_rev(lines, f2w_index, w2l_index);
const rw2l = get_map_rev(lines, w2l_index, l2t_index);
const rl2t = get_map_rev(lines, l2t_index, t2h_index);
const rt2h = get_map_rev(lines, t2h_index, h2l_index);
const rh2l = get_map_rev(lines, h2l_index, len(lines));

let seed_buckets = [];
while len(seeds) > 0 {
    let start = seeds.shift();
    let len = seeds.shift();
    seed_buckets.push([start, start + (len - 1)])
}

print("buckets: " + seed_buckets);
let start = 0;

while start >= 0 {
    let number = map_number_reverse(start);
    if start % 100000 == 0 {
        print("... " + start + " => " + number);
    }
    if number != () {
        // find bucket
        let bucket = seed_buckets.find(|x| x[0] <= number && x[1] >= number );   
        if bucket != () {
            print("found number " + number + " for location " + start + " in bucket " + bucket);
            break
        }
    }
    start += 1;
}
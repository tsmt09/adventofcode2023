let games = input.split("\n");
games = games.map(|x| {
    let cards_bid = x.split(" ");
    cards_bid[1] = parse_int(cards_bid[1]);
    cards_bid
});
// PART 1 const card_order = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A"];
const card_order = ["J", "2", "3", "4", "5", "6", "7", "8", "9", "T", "Q", "K", "A"];

print(hand_type("335JJ"));
// Actual steps
// Sort Game
games.sort(|x,y| compare_game_cards(x[0], y[0]));
// Calculate result = r
let i = 0;
let r = 0;
while i < len(games) {
    r += games[i][1] * (i+1); 
    i += 1;
}
print("Result: " + r);


// logic functions
fn compare_card(card, other) {
    global::card_order.index_of(|x| x == card) - global::card_order.index_of(|x| x == other)
}

fn compare_game_cards(game, other) {
    let i = 0;
    // compare types
    let types = hand_type(game) - hand_type(other);
    if types != 0 {
        return types;
    }
    // if same, compare card worth
    while i < len(game) {
        let cmp = compare_card(game[i], other[i]);
        if cmp != 0 {
            return cmp;
        }
        i += 1;
    }
    return 0;
}

fn hand_type(game) {
    // THIS ONLY PART 2
    game = map_part_2(game);
    // THIS ONLY PART 2
    let counts = [];
    for char in game {
        let count = 0;
        for search in game {
            if char == search {
                count +=1;
            }
        }
        counts.push(count);
    }
    // replace all 0 with highest matching card
    counts.sort(|x,y| y - x); 
    if counts == [5, 5, 5, 5, 5] {
        return 6;
    }
    else if counts == [4, 4, 4, 4, 1] {
        return 5;
    }
    else if counts == [3, 3, 3, 2, 2] {
        return 4;
    }
    else if counts == [3, 3, 3, 1, 1] {
        return 3;
    }
    else if counts == [2, 2, 2, 2, 1] {
        return 2;
    }
    else if counts == [2, 2, 1, 1, 1] {
        return 1;
    } else if counts == [1, 1, 1, 1, 1] {
        return 0;
    } else {
        print("invalid hand:"+game+"|"+counts);
        return -1;
    }
}

fn map_part_2(game) {
    if "J" in game && game != "JJJJJ" {
        // find most common letter and replace
        let max_count = 0;
        let max_worth = 0;
        let most_common = "";
        for char in game {
            if char != "J" {
                let count = 0;
                for search in game {
                    if char == search {
                        count +=1;
                    }
                }
                if count > max_count {
                    max_count = count;
                    most_common = char;
                    max_worth = global::card_order.index_of(|x| x == char);
                } else if count == max_count {
                    let worth = global::card_order.index_of(|x| x == char); 
                    if worth > max_worth {
                        max_count = count;
                        most_common = char;
                        max_worth = global::card_order.index_of(|x| x == char);
                    }
                }
            }
        }
        // print("---: " + game);
        game.replace("J", most_common);
        // print("+++: " + game);
    }
    game
}

fn score(game) {
    // count duplicates
    let chars = game.split("");
    for char in chars {
        
    }
}
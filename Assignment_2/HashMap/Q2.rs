
use std::collections::HashMap;
fn main() {
    let teams = [
        ("Chinese Team", 100),
        ("American Team", 10),
        ("France Team", 50),
    ];

    let mut teams_map1 = HashMap::new();
    for team in &teams {
        teams_map1.insert(team.0, team.1);
    }

    // IMPLEMENT team_map2 in two ways
    // tips: one of the approaches is to use `collect` method
    // let teams_map2 = HashMap::from(teams);  // -> Method 1
    let teams_map2: HashMap<_,_> = teams.into_iter().collect();     // -> Method 2
    

    assert_eq!(teams_map1, teams_map2);

    println!("Success!")
}

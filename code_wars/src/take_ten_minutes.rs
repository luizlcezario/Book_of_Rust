pub fn is_valid_walk(walk: &[char]) -> bool {
    let north = walk.iter().filter(|&x| *x == 'n').count();
    let south = walk.iter().filter(|&x| *x == 's').count();
    let east = walk.iter().filter(|&x| *x == 'e').count();
    let west = walk.iter().filter(|&x| *x == 'w').count();
    if walk.len() == 10 && north == south && east == west {
        true
    } else {
        false
    }
}

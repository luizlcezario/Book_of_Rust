mod disemvowel_trolls;
mod take_ten_minutes;
mod which_are_in;
mod test;
fn main() {
    assert_eq!(disemvowel_trolls::disemvowel("This website is for losers LOL!"), "Ths wbst s fr lsrs LL!");
    assert!(  take_ten_minutes::is_valid_walk(&['n','s','n','s','n','s','n','s','n','s']));
    assert!(! take_ten_minutes::is_valid_walk(&['w','e','w','e','w','e','w','e','w','e','w','e']));
    assert!(! take_ten_minutes::is_valid_walk(&['w']));
    assert!(! take_ten_minutes::is_valid_walk(&['n','n','n','s','n','s','n','s','n','s']));
    assert!(! take_ten_minutes::is_valid_walk(&['e', 'e', 'e', 'e', 'w', 'w', 's', 's', 's', 's']));

    assert_eq!(which_are_in::in_array(
        &["xyz", "live", "strong"],
        &["lively", "alive", "harp", "sharp", "armstrong"],
    ), ["live", "strong"]);
    
    assert_eq!(which_are_in::in_array(
        &["live", "strong", "arp"],
        &["lively", "alive", "harp", "sharp", "armstrong"],
    ), ["arp", "live", "strong"]);
    
    assert_eq!(which_are_in::in_array(
        &["tarp", "mice", "bull"],
        &["lively", "alive", "harp", "sharp", "armstrong"],
    ), [] as [&str; 0]);
    
    assert_eq!(which_are_in::in_array(
        &["live", "strong", "arp", "arp"],
        &["lively", "alive", "harp", "sharp", "armstrong"],
    ), ["arp", "live", "strong"]);

        test::test();
  
}

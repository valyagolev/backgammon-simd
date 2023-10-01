#[cfg(test)]
pub mod tests {
    pub fn assert_all_sides_equivalence(
        ours: &crate::types::board::Board,
        theirs: &backgammon::rules::Board,
    ) {
        let ours_conv: backgammon::rules::Board = ours.into();
        // let theirs_conv: crate::types::board::Board = theirs.into();

        // println!("{:?}", &ours_conv);
        // println!("{:?}", &theirs_conv);

        assert!(ours_conv == *theirs);
        // assert!(theirs_conv == *ours);
    }

    #[test]
    pub fn test_default_boards() {
        let ours = crate::types::board::Board::default();
        let their = backgammon::rules::Board::default();

        // println!("{:?}", &ours_conv);
        // println!("{:?}", &their);

        assert_all_sides_equivalence(&ours, &their);
    }
}

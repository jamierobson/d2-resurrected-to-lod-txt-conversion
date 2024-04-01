pub fn convert_line(line: &str) -> String {
    let mut tokens = split_line_tab_deliminater(line);
    {
        replace_tokens_with_expected_defaults(&mut tokens)
    }
    {
        delete_irrelevent_entries_after_all_manipulations_complete(&mut tokens)
    }

    return tokens.join("\t");
}

fn split_line_tab_deliminater(line: &str) -> Vec<&str> {
    line.split('\t').collect()
}

fn replace_tokens_with_expected_defaults(tokens: &mut Vec<&str>){
    
    let is_runeword = tokens[0].starts_with("Runeword");
    
    if is_runeword {
        override_runeword_row_fields(tokens);
    } else {
        override_header_row_fields(tokens);
    }
}

fn override_header_row_fields(tokens: &mut Vec<&str>) {
    tokens[1] = "Rune Name";
    tokens[3] = "server";
    // note this eventually gets bumped down to 13, because we later delete two columns
    tokens[15] = "*runes"; 
    tokens[50] = "eol"; 
}

fn override_runeword_row_fields(tokens: &mut Vec<&str>) {
    tokens[3] = "";
    if tokens[5] == "109" {
        // The runes field doesn't seem filled in for runes introduced in patch 109
        tokens[15] = ""
    }
}

fn delete_irrelevent_entries_after_all_manipulations_complete(tokens: &mut Vec<&str>) {

    const INDICES_TO_REMOVE: [usize; 2] = [
        5, // *Patch
        4 // lastLadderSeason
    ];

    for index in INDICES_TO_REMOVE {
        tokens.remove(index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::d2_converters::runes::consts;


    #[test]
    fn convert_line_should_convert_header_from_resurrected_to_lod() {

        let expected = consts::lod::HEADER;
        let actual = convert_line(consts::resurrected::HEADER);

        println!("expected \n {}", expected);
        println!("actual \n {}", actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn convert_line_should_convert_ancients_pledge() {

        let expected = consts::lod::ANCIENTS_PLEDGE;
        let actual = convert_line(consts::resurrected::ANCIENTS_PLEDGE);

        println!("expected \n {}", expected);
        println!("actual \n {}", actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn convert_line_should_convert_black() {

        let expected = consts::lod::BLACK;
        let actual = convert_line(consts::resurrected::BLACK);
        
        println!("expected \n {}", expected);
        println!("actual \n {}", actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn convert_line_should_convert_spirit() {

        let expected = consts::lod::SPIRIT;
        let actual = convert_line(consts::resurrected::SPIRIT);
        
        println!("expected \n {}", expected);
        println!("actual \n {}", actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn split_correctly_split_ancients_pledge_including_all_empties(){
        let split:Vec<&str> = split_line_tab_deliminater(consts::resurrected::ANCIENTS_PLEDGE);
        assert_eq!(split.len(), 51);
        
        let emty_count = split.iter().filter(|x| x.is_empty()).count();
        assert_ne!(emty_count, 0);
    }

    #[test]
    fn split_correctly_split_header_row(){
        let split:Vec<&str> = split_line_tab_deliminater(consts::resurrected::HEADER);
        assert_eq!(split.len(), 51);
        
        let emty_count = split.iter().filter(|x| x.is_empty()).count();
        assert_eq!(emty_count, 0);
    }

    #[test]
    fn rename_header_entries_set_the_expected_tokens() {
        let mut split = split_line_tab_deliminater(consts::resurrected::HEADER);
        assert_eq!(split[1], "*Rune Name");
        assert_eq!(split[3], "firstLadderSeason");
        assert_eq!(split[15], "*RunesUsed");
        assert_eq!(split[50], "*eol");
        override_header_row_fields(&mut split);
        assert_eq!(split[1], "Rune Name");
        assert_eq!(split[3], "server");
        assert_eq!(split[15], "*runes");
        assert_eq!(split[50], "eol");
    }
}
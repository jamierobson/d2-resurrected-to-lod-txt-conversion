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

    pub mod resurrected {
        pub const HEADER: &str = "Name	*Rune Name	complete	firstLadderSeason	lastLadderSeason	*Patch Release	itype1	itype2	itype3	itype4	itype5	itype6	etype1	etype2	etype3	*RunesUsed	Rune1	Rune2	Rune3	Rune4	Rune5	Rune6	T1Code1	T1Param1	T1Min1	T1Max1	T1Code2	T1Param2	T1Min2	T1Max2	T1Code3	T1Param3	T1Min3	T1Max3	T1Code4	T1Param4	T1Min4	T1Max4	T1Code5	T1Param5	T1Min5	T1Max5	T1Code6	T1Param6	T1Min6	T1Max6	T1Code7	T1Param7	T1Min7	T1Max7	*eol";
        pub const ANCIENTS_PLEDGE: &str = "Runeword1	Ancients' Pledge	1			109	shld									RalOrtTal	r08	r09	r07				res-cold		30	30	res-all		13	13	ac%		50	50	dmg-to-mana		10	10													0";
        pub const BLACK: &str = "Runeword6	Black	1			109	club	hamm	mace							ThulIoNef	r10	r16	r04				crush		40	40	dmg%		120	120	swing2		15	15	red-mag		2	2	att		200	200	charged	74	12	4					0";
        pub const SPIRIT: &str = "Runeword130	Spirit	1			Previously Ladder Only	swor	shld								TalThulOrtAmn	r07	r10	r09	r11			balance3		55	55	mana		89	112	ac-miss		250	250	vit		22	22	cast3		25	35	abs-mag		3	8	allskills		2	2	0";
    }
    
    pub mod lod {
        pub const HEADER: &str = "Name	Rune Name	complete	server	itype1	itype2	itype3	itype4	itype5	itype6	etype1	etype2	etype3	*runes	Rune1	Rune2	Rune3	Rune4	Rune5	Rune6	T1Code1	T1Param1	T1Min1	T1Max1	T1Code2	T1Param2	T1Min2	T1Max2	T1Code3	T1Param3	T1Min3	T1Max3	T1Code4	T1Param4	T1Min4	T1Max4	T1Code5	T1Param5	T1Min5	T1Max5	T1Code6	T1Param6	T1Min6	T1Max6	T1Code7	T1Param7	T1Min7	T1Max7	eol";
        pub const ANCIENTS_PLEDGE: &str = "Runeword1	Ancients' Pledge	1		shld										r08	r09	r07				res-cold		30	30	res-all		13	13	ac%		50	50	dmg-to-mana		10	10													0";
        pub const BLACK: &str = "Runeword6	Black	1		club	hamm	mace								r10	r16	r04				crush		40	40	dmg%		120	120	swing2		15	15	red-mag		2	2	att		200	200	charged	74	12	4					0";
        pub const SPIRIT: &str = "Runeword130	Spirit	1		swor	shld								TalThulOrtAmn	r07	r10	r09	r11			balance3		55	55	mana		89	112	ac-miss		250	250	vit		22	22	cast3		25	35	abs-mag		3	8	allskills		2	2	0";
    }
    
    pub mod lod_1_10_beta {
        pub const HEADER: &str = "Name	Rune Name	complete	server	itype1	itype2	itype3	itype4	itype5	itype6	etype1	etype2	etype3	Rune1	Rune2	Rune3	Rune4	Rune5	Rune6	T1Code1	T1Param1	T1Min1	T1Max1	T1Code2	T1Param2	T1Min2	T1Max2	T1Code3	T1Param3	T1Min3	T1Max3	T1Code4	T1Param4	T1Min4	T1Max4	T1Code5	T1Param5	T1Min5	T1Max5	T1Code6	T1Param6	T1Min6	T1Max6";
        pub const ANCIENTS_PLEDGE: &str = "Runeword1	Ancients' Pledge	1	1	shld									r08	r09	r07				res-cold		30	30	res-all		13	13	ac%		50	50	dmg-to-mana		10	10								";
        pub const BLACK: &str = "Runeword6	Black	1	1	club	hamm	mace							r16	r12	r02				dmg-undead		25	25	att-undead		50	50	manasteal		7	7	dmg%		133	133	lifesteal		7	7				";
        pub const SPIRIT: &str = "Runeword129	Spirit		1	weap									r23	r01	r01	r30	r13		res-pois	0	25	25																				";
    }

    use super::*;
    // use crate::d2_converters::runes::consts;


    #[test]
    fn convert_line_should_convert_header_from_resurrected_to_lod() {

        let expected = lod::HEADER;
        let actual = convert_line(resurrected::HEADER);

        println!("expected \n {}", expected);
        println!("actual \n {}", actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn convert_line_should_convert_ancients_pledge() {

        let expected = lod::ANCIENTS_PLEDGE;
        let actual = convert_line(resurrected::ANCIENTS_PLEDGE);

        println!("expected \n {}", expected);
        println!("actual \n {}", actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn convert_line_should_convert_black() {

        let expected = lod::BLACK;
        let actual = convert_line(resurrected::BLACK);
        
        println!("expected \n {}", expected);
        println!("actual \n {}", actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn convert_line_should_convert_spirit() {

        let expected = lod::SPIRIT;
        let actual = convert_line(resurrected::SPIRIT);
        
        println!("expected \n {}", expected);
        println!("actual \n {}", actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn split_correctly_split_ancients_pledge_including_all_empties(){
        let split:Vec<&str> = split_line_tab_deliminater(resurrected::ANCIENTS_PLEDGE);
        assert_eq!(split.len(), 51);
        
        let emty_count = split.iter().filter(|x| x.is_empty()).count();
        assert_ne!(emty_count, 0);
    }

    #[test]
    fn split_correctly_split_header_row(){
        let split:Vec<&str> = split_line_tab_deliminater(resurrected::HEADER);
        assert_eq!(split.len(), 51);
        
        let emty_count = split.iter().filter(|x| x.is_empty()).count();
        assert_eq!(emty_count, 0);
    }

    #[test]
    fn rename_header_entries_set_the_expected_tokens() {
        let mut split = split_line_tab_deliminater(resurrected::HEADER);
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
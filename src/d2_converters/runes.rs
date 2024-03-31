pub fn convert_header() {

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn convert_header_should_convert_from_resurrected_to_lod() {
        let resurrected_header = "Name	*Rune Name	complete	firstLadderSeason	lastLadderSeason	*Patch Release	itype1	itype2	itype3	itype4	itype5	itype6	etype1	etype2	etype3	*RunesUsed	Rune1	Rune2	Rune3	Rune4	Rune5	Rune6	T1Code1	T1Param1	T1Min1	T1Max1	T1Code2	T1Param2	T1Min2	T1Max2	T1Code3	T1Param3	T1Min3	T1Max3	T1Code4	T1Param4	T1Min4	T1Max4	T1Code5	T1Param5	T1Min5	T1Max5	T1Code6	T1Param6	T1Min6	T1Max6	T1Code7	T1Param7	T1Min7	T1Max7	*eol";
        let lod_header = "Name	Rune Name	complete	server	itype1	itype2	itype3	itype4	itype5	itype6	etype1	etype2	etype3	Rune1	Rune2	Rune3	Rune4	Rune5	Rune6	T1Code1	T1Param1	T1Min1	T1Max1	T1Code2	T1Param2	T1Min2	T1Max2	T1Code3	T1Param3	T1Min3	T1Max3	T1Code4	T1Param4	T1Min4	T1Max4	T1Code5	T1Param5	T1Min5	T1Max5	T1Code6	T1Param6	T1Min6	T1Max6";

        

        assert!(true);
    }
}
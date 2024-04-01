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
# d2-resurrected-to-lod-txt-conversion
An attempt to write converters for some txt files, to mod some D2 Resurrected content into D2 LoD

# Getting started
- You need to own Diablo 2, Lord of Destruction, and Resurrected legitimately to be able to use this tooling, and you'll need them installed to get access to reference files.
- Install both a [Casc viewer](http://www.zezula.net/en/casc/main.html) for extracting resurrected content, and an [MPQ viewer](http://www.zezula.net/en/mpq/download.html) for extracting D2 Classic/LoD content
- Follow the instructions to export the `global\excel`folders
- Install the [Rust toolchain](https://www.rust-lang.org/learn/get-started)

# Converters
1. [Runes.txt](#runestxt)

# Runes.txt
The format changes between 1.14D and 2.7 aren't too extensive:

- Rename index 3 to "server"
- Rename index 15 from `*RunesUsed` to `*runes`
- remove the `*` from `*eol` in index 50
- Set index 3 to empty
- delete indices 4 and 5
- ~~delete whatever is in position *RunesUsed (index 15)~~
- ~~Delete the last 5 entries (indices 46-50): `T1Code7	T1Param7	T1Min7	T1Max7	*eol`. Note that this means that any runeswords with a 7th effect loses that effect. Manual intervention would be required here~~

By following my own steps, I seem to have acquired a pre 1.10 version of runes.txt. An example is my version of the black runeword. [(According to this thread)](https://planetdiablo.eu/forum/threads/neue-runenwoerter.656354/) which was `IoSolEld` (r16 r12 r02) according to the thread, but is, and I believe always has been `ThulIoNef` (r10	r16	r04) in retail.

```
Runeword6	Black	1	1	club	hamm	mace							r16	r12	r02				dmg-undead		25	25	att-undead		50	50	manasteal		7	7	dmg%		133	133	lifesteal		7	7				
```

Using the [reference runes.txt](#114d-runestxt), we can rethink the conversion strategy. It looks like there's no need to change fields

# Reference Material
## 1.14d runes.txt
References from [pastebin](https://pastebin.com/xGLWhmZ1)
```
Name	Rune Name	complete	server	itype1	itype2	itype3	itype4	itype5	itype6	etype1	etype2	etype3	*runes	Rune1	Rune2	Rune3	Rune4	Rune5	Rune6	T1Code1	T1Param1	T1Min1	T1Max1	T1Code2	T1Param2	T1Min2	T1Max2	T1Code3	T1Param3	T1Min3	T1Max3	T1Code4	T1Param4	T1Min4	T1Max4	T1Code5	T1Param5	T1Min5	T1Max5	T1Code6	T1Param6	T1Min6	T1Max6	T1Code7	T1Param7	T1Min7	T1Max7	eol
Runeword1	Ancient's Pledge	1		shld										r08	r09	r07				res-cold		30	30	res-all		13	13	ac%		50	50	dmg-to-mana		10	10													0
Runeword6	Black	1		club	hamm	mace								r10	r16	r04				crush		40	40	dmg%		120	120	swing2		15	15	red-mag		2	2	att		200	200	charged	74	12	4					0
Runeword44	Fury	1		mele										r31	r25	r05				dmg%		209	209	swing2		40	40	noheal		1	1	openwounds		66	66	lifesteal		6	6	deadly		33	33	skill	147	5	5	0
Runeword54	Holy Thunder	1		scep										r05	r08	r09	r07			dmg%		60	60	dmg-ltng		20	60	dmg-max		10	10	res-ltng		60	60	res-ltng-max		5	5	skill	118	3	3	charged	53	60	7	0
Runeword55	Honor	1		mele										r11	r01	r06	r03	r12		dmg%		160	160	regen		10	10	allskills		1	1	att		200	200	deadly		25	25	str		10	10					0
Runeword65	King's Grace	1		swor	scep									r11	r08	r10				dmg%		100	100	att		150	150	dmg-demon		100	100	dmg-undead		50	50	att-demon		100	100	att-undead		100	100					0
Runeword72	Leaf	1		staf										r03	r08					fireskill		3	3	ac/lvl		16	16	res-cold		33	33	skill	41	3	3	skill	36	3	3	skill	37	3	3					0
Runeword74	Lionheart	1		tors										r15	r17	r19				str		15	15	vit		20	20	dex		15	15	dmg%		20	20	hp		50	50	res-all		30	30					0
Runeword75	Lore	1		helm										r09	r12					enr		10	10	allskills		1	1	light		2	2	mana-kill		2	2													0
Runeword81	Malice	1		mele										r06	r01	r05				openwounds		100	100	dmg-ac		-100	-100	noheal		1	1	dmg%		33	33	light		-1	-1	regen		-5	-5					0
Runeword82	Melody	1		miss										r13	r18	r04				dmg%		50	50	skilltab	0	3	3	skill	9	3	3	skill	13	3	3	skill	17	3	3	dmg-undead		300	300					0
Runeword83	Memory	1		staf										r17	r16	r12	r05			mana%		20	20	red-mag		7	7	ac%		50	50	cast2		33	33	sor		3	3	skill	58	3	3	skill	42	2	2	0
Runeword88	Nadir	1		helm										r04	r03					ac%		50	50	ac		10	10	light		-3	-3	charged	264	9	13	gold%		-33	-33	str		5	5					0
Runeword116	Radiance	1		helm										r04	r12	r06				light		5	5	enr		10	10	vit		10	10	red-mag		3	3	mana		33	33	ac%		75	75					0
Runeword120	Rhyme	1		shld										r13	r05					block2		20	20	block		20	20	res-all		25	25	nofreeze		1	1	gold%		50	50	mag%		25	25					0
Runeword126	Silence	1		weap										r14	r02	r15	r24	r03	r26	manasteal		4	4	stupidity		33	33	dmg%		200	200	swing2		20	20	res-all		75	75	allskills		2	2	balance2		20	20	0
Runeword128	Smoke	1		tors										r04	r17					ac-miss		250	250	ac%		75	75	res-all		50	50	balance2		20	20	light		-1	-1	charged	72	18	6					0
Runeword133	Stealth	1		tors										r07	r05					red-mag		3	3	dex		6	6	stam		15	15	move2		25	25	cast2		25	25	balance2		25	25					0
Runeword134	Steel	1		swor	axe	mace								r03	r01					swing2		25	25	dmg-min		3	3	dmg-max		3	3	openwounds		50	50	dmg%		20	20									0
Runeword139	Strength	1		mele										r11	r03					str		20	20	dmg%		35	35	vit		10	10	crush		25	25													0
Runeword154	Venom	1		weap										r07	r14	r23				dmg-pois	175	312	312	ignore-ac		1	1	charged	83	27	15	charged	92	11	13	manasteal		7	7									0
Runeword160	Wealth	1		tors										r20	r18	r03				gold%		250	250	mag%		100	100																					0
Runeword162	White	1		wand										r14	r16					skilltab	7	3	3	red-mag		4	4	cast2		20	20	mana		13	13	skill	68	3	3	skill	84	2	2	skill	69	4	4	0
Runeword170	Zephyr	1		miss										r09	r05					move2		25	25	swing2		25	25	dmg%		33	33	att		66	66	gethit-skill	240	7	1	ac		25	25					0
Runeword14	Bound by Duty	1		tors									DolUmBerIst	r14	r22	r30	r24			res-all		50	50	ac%		70	70	dmg-demon		200	200	dmg-undead		100	100	lifesteal		8	8	allskills		2	2	str		20	20	0
Runeword9	Bramble	1		tors									RalOhmSurEth	r08	r27	r29	r05			balance2		50	50	ac		300	300	aura	Thorns	15	21	heal-kill		13	13	extra-pois		25	50	res-pois		100	100	charged	Spirit of Barbs	33	13	0
Runeword11	Breath of the Dying	1		weap									VexHelElEldZodEth	r26	r15	r01	r02	r33	r05	swing2		60	60	dmg-undead		125	125	lifesteal		12	15	noheal		1	1	kill-skill	Poison Nova	50	20	dmg%		350	400	all-stats		30	30	0
Runeword13	Call to Arms	1		weap									AmnRalMalIstOhm	r11	r08	r23	r24	r27		swing2		40	40	dmg%		200	240	allskills		1	1	oskill	Battle Command	2	6	oskill	Battle Orders	1	6	oskill	Battle Cry	1	4	regen		12	12	0
Runeword17	Crescent Moon	1		axe	swor	pole							ShaelUmTir	r13	r22	r03				pierce-ltng		35	35	ignore-ac		1	1	dmg%		180	220	abs-mag		9	11	charged	Summon Spirit Wolf	30	18	hit-skill	Static Field	7	13	hit-skill	Chain Lightning	10	17	0
Runeword22	Delirium	1		helm									LemIstIo	r20	r24	r16				hit-skill	Confuse	11	18	charged	Attract	60	17	gethit-skill	Terror	14	13	ac		261	261	gethit-skill	Mind Blast	6	14	gethit-skill	Delerium Change	1	50	allskills		2	2	0
Runeword26	Doomsayer	1		axe	pole	hamm							HelOhmUmLoCham	r15	r27	r22	r28	r32		dmg%		280	320	aura	Holy Freeze	12	12	swing2		45	45	noheal		1	1	pierce-cold		40	60	allskills		2	2	hit-skill	Volcano	5	18	0
Runeword33	Enigma	1		tors									JahIthBer	r31	r06	r30				ac		750	775	heal-kill		14	14	move2		45	45	str/lvl	6			allskills		2	2	mag%/lvl	8			oskill	Teleport	1	1	0
Runeword137	Stone	1		tors									ShaelUmPulLum	r13	r22	r21	r17			ac%		220	260	charged	Clay Golem	16	16	ac-miss		300	300	charged	Molten Boulder	80	16	str		16	16	vit		16	16	balance2		40	40	0
Runeword36	Eternity	1		mele									AmnBerIstSolSur	r11	r30	r24	r12	r29		dmg%		260	310	indestruct		1	1	slow		33	33	charged	Revive	88	8	regen		16	16	regen-mana		16	16	nofreeze		1	1	0
Runeword37	Exile's Path	1		pala									VexOhmIstDol	r26	r27	r24	r14			block2		30	30	freeze		1	1	ac%		220	260	aura	Defiance	13	16	skilltab	10	2	2	hit-skill	Life Tap	15	5	rep-dur	25			0
Runeword39	Famine	1		axe	hamm								FalOhmOrtJah	r19	r27	r09	r31			dmg%		270	320	lifesteal		12	12	swing2		30	30	noheal		1	1	dmg-mag		180	200	dmg-elem	100	50	200	ethereal		1	1	0
Runeword48	Hand of Justice	1		weap									SurChamAmnLo	r29	r32	r11	r28			swing2		33	33	dmg%		280	330	aura	Holy Fire	16	16	levelup-skill	Blaze	100	36	death-skill	Meteor	100	48	ignore-ac		1	1	pierce-fire		20	20	0
Runeword51	Heart of the Oak	1		staf	mace								KoVexPulThul	r18	r26	r21	r10			cast2		40	40	charged	Oak Sage	25	4	mana%		15	15	allskills		3	3	regen		20	20	res-all		30	40	charged	Raven	60	14	0
Runeword66	Kingslayer	1		swor	axe								MalUmGulFal	r23	r22	r25	r19			swing2		30	30	dmg%		230	270	reduce-ac		25	25	crush		33	33	openwounds		25	25	oskill	Vengeance	1	1	gold%		40	40	0
Runeword95	Passion	1		weap									DolOrtEldLem	r14	r09	r02	r20			dmg%		160	210	oskill	Zeal	1	1	att%		50	80	oskill	Berserk	1	1	swing2		25	25	charged	Heart of Wolverine	12	3	stupidity		10	10	0
Runeword45	Gloom	1		tors									FalUmPul	r19	r22	r21				ac%		170	230	res-all		30	30	gethit-skill	Dim Vision	15	3	balance2		10	10	dmg-to-mana		5	5	light		-3	-3	half-freeze		1	1	0
Runeword122	Sanctuary	1		shld									KoKoMal	r18	r18	r23				block		20	20	block2		20	20	ac%		130	160	ac-miss		250	250	res-all		50	70	balance2		20	20	charged	Slow Missiles	60	12	0
Runeword16	Chaos	1		h2h									FalOhmUm	r19	r27	r22				demon-heal		15	15	dmg%		240	290	dmg-mag		216	471	oskill	Whirlwind	1	1	swing2		35	35	hit-skill	Frozen Orb	9	11	hit-skill	Charged Bolt	11	9	0
Runeword4	The Beast	1		axe	scep	hamm								r30	r03	r22	r23	r17		swing2		40	40	aura	Fanaticism	9	9	dmg%		240	270	str		25	40	charged	Summon Grizzly	5	13	oskill	Wearbear	3	3	oskill	Shape Shifting	3	3	0
Runeword123	Serendipity																																															0
Runeword124	Shadow																																															0
Runeword125	Shadow of Doubt																																															0
Runeword127	Siren's Song																																															0
Runeword129	Sorrow																																															0
Runeword130	Spirit	1		swor	shld								TalThulOrtAmn	r07	r10	r09	r11			balance3		55	55	mana		89	112	ac-miss		250	250	vit		22	22	cast3		25	35	abs-mag		3	8	allskills		2	2	0
Runeword131	Splendor	1		shld										r05	r17					light		3	3	gold%		50	50	mag%		20	20	ac%		60	100	block2		20	20	cast2		10	10	allskills		1	1	0
Runeword132	Starlight																																															0
Runeword135	Still Water																																															0
Runeword30	Duress	1		tors									ShaelUmThul	r13	r22	r10				dmg-cold	50	37	133	dmg%		10	20	ac%		150	200	balance2		20	20	openwounds		33	33	crush		15	15	stamdrain		-20	-20	0
Runeword138	Storm																																															0
Runeword140	Tempest																																															0
Runeword141	Temptation																																															0
Runeword142	Terror																																															0
Runeword90	Nightfall																																															0
Runeword91	Oath	1		swor	axe	mace							ShaelPulMalLum	r13	r21	r23	r17			dmg%		210	340	swing1		30	30	hit-skill	Bone Spirit	30	20	charged	IronGolem	14	17	charged	Heart of Wolverine	20	16	abs-mag		10	15	indestruct		1	1	0
Runeword92	Obedience	1		pole									HelKoThulEthFal	r15	r18	r10	r05	r19		dmg%		370	370	crush		40	40	kill-skill	enchant	30	21	pierce-fire		25	25	ac		200	300	balance3		40	40	res-all		20	30	0
Runeword93	Oblivion																																															0
Runeword94	Obsession																																															0
Runeword49	Harmony	1		miss									TirIthSolKo	r03	r06	r12	r18			dmg%		200	275	dmg-elem		55	160	charged	Revive	25	20	aura	Vigor	10	10	oskill	Valkyrie	2	6	regen-mana		20	20	light		2	2	0
Runeword50	Hatred																																															0
Runeword40	Flickering Flame																																															0
Runeword41	Fortitude	1		weap	tors								ElSolDolLo	r01	r12	r14	r28			ac%		200	200	dmg%		300	300	cast3		25	25	gethit-skill	Chilling Armor	20	15	dmg-to-mana		12	12	hp/lvl		8	12	res-all		25	30	0
Runeword42	Fortune's Favor																																															0
Runeword89	Nature's Kingdom																																															0
Runeword46	Glory																																															0
Runeword38	Faith	1		miss									OhmJahLemEld	r27	r31	r20	r02			dmg%		280	280	att%		300	300	dmg-fire		120	120	res-all		15	15	aura	fanaticism	12	15	reanimate	1	10	10	allskills		1	2	0
Runeword43	Amity																																															0
Runeword2	Armageddon																																															0
Runeword3	Authority																																															0
Runeword5	Beauty																																															0
Runeword7	Blood			helm																																												0
Runeword8	Bone	1		tors									SolUmUm	r12	r22	r22				hit-skill	Bone Spear	15	10	gethit-skill	Bone Armor	15	10	nec		2	2	mana		100	150													0
Runeword10	Brand	1		miss									JahLoMalGul	r31	r28	r23	r25			dmg%		260	340	dmg-demon		280	330	hit-skill	Bone Spear	100	18	gethit-skill	Amplify Damage	35	14	knock		1	1	noheal		1	1	explosivearrow		15	15	0
Runeword12	Broken Promise																																															0
Runeword15	Chance																																															0
Runeword136	Sting																																															0
Runeword18	Darkness																																															0
Runeword19	Daylight																																															0
Runeword20	Death	1		swor	axe								HelElVexOrtGul	r15	r01	r26	r09	r25		dmg%		300	385	deadly/lvl	4			charged	BloodGolem	15	22	att-skill	Glacial Spike	25	18	death-skill	Chain Lightning	100	44	crush		50	50	indestruct		1	1	0
Runeword21	Deception																																															0
Runeword23	Desire																																															0
Runeword24	Despair																																															0
Runeword32	Destiny's Daughter																																															0
Runeword25	Destruction	1		pole	swor								VexLoBerJahKo	r26	r28	r30	r31	r18		dmg%		350	350	dmg-mag		100	180	hit-skill	Molten Boulder	5	23	death-skill	Meteor	100	45	att-skill	Nova	15	22	hit-skill	Volcano	23	12	noheal		1	1	0
Runeword27	Dragon	1		tors	shld								SurLoSol	r29	r28	r12				ac		360	360	ac-miss		230	230	str/lvl	3			hit-skill	Hydra	12	15	gethit-skill	Venom	20	18	aura	Holy Fire	14	14	all-stats		3	5	0
Runeword28	Dread																																															0
Runeword29	Dream	1		helm	shld								IoJahPul	r16	r31	r21				ac		150	220	gethit-skill	Confuse	10	15	mana/lvl	5			res-all		5	20	balance3		20	30	aura	Holy Shock	15	15	mag%		12	25	0
Runeword56	Dweomer																																															0
Runeword31	Edge	1		miss									TirTalAmn	r03	r07	r11				dmg-demon		320	380	dmg-undead		280	280	swing2		35	35	noheal		1	1	aura	Thorns	15	15	all-stats		5	10	cheap		15	15	0
Runeword34	Enlightenment	1		tors									PulRalSur	r21	r08	r12				hit-skill	Fire Ball	5	15	gethit-skill	Blaze	5	15	sor		2	2	oskill	Warmth	1	1													0
Runeword35	Envy																																															0
Runeword52	Heaven's Will																																															0
Runeword53	Holy Tears																																															0
Runeword57	Humility																																															0
Runeword58	Hunger																																															0
Runeword59	Ice	1		miss									AmnShaelJahLo	r11	r13	r31	r28			dmg%		140	210	aura	Holy Freeze	18	18	extra-cold		25	30	hit-skill	Frost Nova	25	22	levelup-skill	Blizzard	100	40	pierce-cold		20	20	gold%/lvl	25			0
Runeword60	Infinity	1		pole									BerMalBerIst	r30	r23	r30	r24			dmg%		255	325	move3		35	35	vit/lvl	4			aura	Conviction	12	12	kill-skill	Chain Lightning	50	20	pierce-ltng		45	55	charged	Cyclone Armor	30	21	0
Runeword61	Innocence																																															0
Runeword62	Insight	1		pole	staf								RalTirTalSol	r08	r03	r07	r12			dmg%		200	260	att%		180	250	mag%		23	23	oskill	Critical Strike	1	6	cast2		35	35	aura	Meditation	12	17	all-stats		5	5	0
Runeword63	Jealousy																																															0
Runeword64	Judgment																																															0
Runeword67	Knight's Vigil																																															0
Runeword69	Last Wish	1		swor	hamm	axe							JahMalJahSurJahBer	r31	r23	r31	r29	r31	r30	dmg%		330	375	att-skill	Charged Bolt	20	20	hit-skill	Life Tap	10	18	gethit-skill	Fade	6	11	crush		40	50	mag%/lvl	4			aura	Might	17	17	0
Runeword70	Law																																															0
Runeword71	Lawbringer	1		swor	hamm	scep							AmnLemKo	r11	r20	r18				dmg-cold		130	180	dmg-fire		150	210	aura	Sanctuary	16	18	hit-skill	Decrepify	20	15	ac-miss		200	250	rip		1	1	reduce-ac		50	50	0
Runeword73	Lightning																																															0
Runeword77	Loyalty																																															0
Runeword78	Lust																																															0
Runeword79	Madness																																															0
Runeword84	Mist																																															0
Runeword85	Morning Dew																																															0
Runeword86	Mystery																																															0
Runeword87	Myth	1		tors									HelAmnNef	r15	r11	r04				hit-skill	Taunt	10	1	gethit-skill	Howl	3	1	bar		2	2	regen		10	10													0
Runeword95	Patience																																															0
Runeword97	Pattern			h2h										r07	r09	r03				att%		10	10	dmg%		40	80	dmg-fire		12	32	res-all		15	15	str		6	6	dex		6	6	block2		30	30	0
Runeword98	Peace	1		tors									ShaelThulAmn	r13	r10	r11				hit-skill	Valkyrie	2	15	gethit-skill	Slow Missiles	4	5	ama		2	2	oskill	Critical Strike	2	2													0
Runeword100	Penitence																																															0
Runeword101	Peril																																															0
Runeword102	Pestilence																																															0
Runeword103	Phoenix	1		weap	shld								VexVexLoJah	r26	r26	r28	r31			dmg%		350	400	ac-miss		350	400	hit-skill	Firestorm	40	22	levelup-skill	Blaze	100	40	pierce-fire		28	28	aura	Redemption	10	15	abs-fire		15	21	0
Runeword104	Piety																																															0
Runeword105	Pillar of Faith																																															0
Runeword106	Plague			weap									ChamFalUm	r32	r19	r22				dmg-demon		260	380	gethit-skill	Lower Resist	20	12	hit-skill	Poison Nova	25	15	pierce-pois		23	23	deadly/lvl	3			aura	Cleansing	13	17	allskills		1	2	0
Runeword107	Praise																																															0
Runeword108	Prayer																																															0
Runeword109	Pride	1		pole									ChamSurIoLo	r32	r29	r16	r28			dmg-dem/lvl	8			dmg-ltng		50	280	att%		260	300	aura	Concentration	16	20	gethit-skill	Fire Wall	25	17	regen		8	8	gold%/lvl	15			0
Runeword110	Principle	1		tors									RalGulEld	r08	r25	r02				hit-skill	Holy Bolt	100	5	pal		2	2	hp		100	150	dmg-undead		50	50													0
Runeword111	Prowess in Battle																																															0
Runeword112	Prudence	1		tors										r23	r03					ac%		140	170	red-mag		10	10	red-dmg		3	3	res-all		25	35	balance2		25	25	rep-dur	25			light		1	1	0
Runeword113	Punishment																																															0
Runeword114	Purity																																															0
Runeword115	Question																																															0
Runeword117	Rain	1		tors									OrtMalIth	r09	r23	r06				hit-skill	Twister	5	15	gethit-skill	Cyclone Armor	5	15	dru		2	2	mana		100	150													0
Runeword118	Reason																																															0
Runeword119	Red																																															0
Runeword121	Rift	1		pole	scep								HelKoLemGul	r15	r18	r20	r25			dmg-mag		160	250	dmg-fire		60	180	dmg-to-mana		38	38	hit-skill	Tornado	20	16	att-skill	Frozen Orb	16	21	charged	Iron Maiden	40	15	all-stats		5	10	0
Runeword76	The Lovers																																															0
Runeword143	Thirst																																															0
Runeword68	Thirst for Knowledge																																															0
Runeword144	Thought																																															0
Runeword145	Thunder																																															0
Runeword146	Time																																															0
Runeword147	Tradition																																															0
Runeword148	Treachery	1		tors									ShaelThulLem	r13	r10	r20				hit-skill	Venom	25	15	gethit-skill	Fade	5	15	ass		2	2	swing2		45	45													0
Runeword149	Trust																																															0
Runeword150	Truth																																															0
Runeword151	Unbending Will																																															0
Runeword152	Valor																																															0
Runeword153	Vengeance			weap										r06	r11	r01																																0
Runeword155	Victory																																															0
Runeword156	Voice			helm										r03	r06																																	0
Runeword157	Void																																															0
Runeword158	War																																															0
Runeword159	Water																																															0
Runeword161	Whisper																																															0
Runeword47	Widowmaker	1		swor	axe								EthTirLoMalRal	r05	r03	r28	r23	r08		dmg-dem/lvl	15			dmg		340	400	swing3		30	40	hit-skill	Venom	35	15	pierce-pois		20	25	ignore-ac		1	1	heal-kill		10	15	0
Runeword163	Wind	1		mele										r29	r01					dmg%		120	160	swing2		40	40	move2		20	20	reduce-ac		50	50	hit-skill	245	10	9	charged	240	127	13	balance2		15	15	0
Runeword164	Wings of Hope																																															0
Runeword99	Winter	1		swor	mace								LemKoElEld	r20	r18	r01	r02			dmg-demon		220	350	dmg-undead		280	300	dmg-cold		100	220	pierce-cold		24	24	hit-skill	Ice Blast	18	20	hit-skill	Frozen Orb	15	13	nofreeze		1	1	0
Runeword165	Wisdom			shld	staf	wand																																										0
Runeword166	Woe			weap	miss																																											0
Runeword167	Wonder			wand										r04	r12	r24																																0
Runeword168	Wrath	1		miss									PulLumBerMal	r21	r17	r30	r23			dmg-demon		300	300	dmg-undead		250	300	dmg-ltng		41	240	dmg-mag		85	120	hit-skill	Life Tap	5	10	hit-skill	decrepify	30	1	nofreeze		1	1	0
Runeword169	Youth																																															0
```
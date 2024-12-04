use std::collections::HashMap;

use regex::Regex;

struct Instructions<'a> {
    source: &'a str,
    multiplications: HashMap<usize, (usize, usize)>,
    conditionals: HashMap<usize, bool>,
}

impl Instructions<'_> {
    fn all_multiplications(&self) -> usize {
        self.multiplications
            .values()
            .map(|(x, y)| x * y)
            .fold(0, |acc, cur: usize| acc + cur)
    }

    fn only_enabled_multiplications(&self) -> usize {
        let mut enabled = false;
        let mut enabled_multiplications: Vec<&(usize, usize)> = vec![];

        for i in 0..self.source.len() {
            if self.conditionals.get(&i).is_some() {
                enabled = *self.conditionals.get(&i).unwrap();
            }

            if !enabled {
                continue;
            }

            match self.multiplications.get(&i) {
                Some(conditional) => enabled_multiplications.push(conditional),
                _ => (),
            };
        }

        enabled_multiplications
            .iter()
            .map(|(x, y)| x * y)
            .fold(0, |acc, cur: usize| acc + cur)
    }
}

fn main() {
    assert_eq!(161, part1(&test_input()));
    assert_eq!(170807108, part1(&input()));
    assert_eq!(48, part2(&test_input()));
    assert_eq!(0, part2(&input()));
}

fn part1(input: &str) -> usize {
    parse(input).all_multiplications()
}

fn part2(input: &str) -> usize {
    parse(input).only_enabled_multiplications()
}

fn parse(input: &str) -> Instructions {
    let regex = Regex::new(r"mul\((\d+)\,(\d+)\)").unwrap();

    let multiplications: HashMap<usize, (usize, usize)> = regex
        .captures_iter(input)
        .map(|captures| {
            let (_, [x_str, y_str]) = captures.extract();

            let i: usize = captures.get(0).unwrap().start();
            let x: usize = x_str.parse().unwrap();
            let y: usize = y_str.parse().unwrap();

            (i, (x, y))
        })
        .collect();

    let regex = Regex::new(r"(don't|do)").unwrap();

    let conditionals: HashMap<usize, bool> = regex
        .captures_iter(input)
        .map(|captures| {
            let (_, [conditional_str]) = captures.extract();

            let i: usize = captures.get(0).unwrap().start();
            let conditional: bool = match conditional_str {
                "do" => true,
                "don't" => false,
                &_ => panic!("something else than do or don't was encountered!"),
            };

            (i, conditional)
        })
        .collect();

    Instructions {
        source: input,
        multiplications,
        conditionals: HashMap::new(),
    }
}

#[allow(dead_code)]
fn test_input() -> String {
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string()
}

#[allow(dead_code)]
fn input() -> String {
    "}@/]({why()why()mul(857,200)}&)){!from()-mul(158,758):who():!who()what()@>%mul(953,214)from()why()when()$'[+mul(277,918)select()[~$)mul(708,736)^~from()/why()^#mul(588,753)who()]$where()mul(562,499)->')!don't()mul(677,889)#]##?,-?don't()&$when()$mul(554,958)how()?how() ~)(;don't()>+~when(){~what()+}'mul(446,878)/:&]how()!mul(207,471)}what()~-+/mul(398,851)where()how()@where()}-]how()mul(847,472)mul(244,19){-#:$mul(895,91)(!:mul(872,18)+}why(251,250)mul(630,73)%:,!who(981,677)!!mul(103,541)why(),how() )%:mul(749,545)what()^mul(22,489)mul(699,518)-~,where()[where()~mul(291,9)select()-when()]do()!<<select();who()}%mul(830,471):?@-how()when(){$how()(mul(528,980)from()what() [?&$mul(930,234)where()}mul(992,63)>,[what()!//+why()/mul(38,546):mul(297,2)why()<who()[what()$)#!don't()^#:?mul(458,249)<%^*)];&mul(816,429)* ;what()select()when()mul(15,824):#mul(669,34)from()$mul(363,251)$select(757,257)*@^?@~what()#mul(551,812)!>mul(745,860) mul(110,274)mul(30,694)'%#^)why()mul(491,133))]where(835,197)[/mul(212,917)+^*^{when(421,261)-where()<)mul(448,147)where()]mul(448,494);)!what()%~#?#~do()/?/@[why()//select()mul(992,463)/mul(447,526){-mul(907,965):!who()#how()mul(95,36)where()where()#;~+~mul(898,41)- !^-*#where()mul(63,801):^@from()^mul(742,163)where()*'mul(979,736)when(627,44)!who()mul(92,637)!~/%]>~^mul(723,185)select()~%]);^when(384,799)mul(597,721),%'* !mul(915,963)*;don't()select()how())~:,how()mul(498,757)'/mul(361,423)///where(){what()^who()$why()mul(700,270)[~what():from()mul(442,15)?)+mul(134,759)who()-@}select()when()'&who()<mul(197,114)+'@@:#;:+mul(456,291)when();[mul(810,734)&;select()mul(779,730)@mul(48,436)who()},],mul(856,377)how()@]mul(231,214)mul(289,948))how()from()where()/}why()select()mul(413,445)from()[who()@]mul(547,89)~'[(!who()@}mul(433,361)-how()?>who()?what(697,478)?-what(516,904)mul(435,284)@!> ^<mul(798,836)<(mul(701,253)}'#%}mul(896,763)mul(607,689)'where()'}^mul**when()why()<(what()why(127,499)%mul(124,832)%why()where()^] mul(815,705)@who()]mul(362,182)};mul,!mul(551,219)where()?)&(:[)mul(958,936)when()&$>what(){mul(118,686)&mul(478,122)when(586,470)$>mul(835,231){]@[do()~$$]%~;-mul(236,742){&,mul(512,497)[where()()where()mul(295,913)[mul(959,932)~+@mul(868,214)'>}'mul(549,147),'/<,mul(121,328)(how()-mul(262,843)what() ;&when()]mul(886,378)? {what()who()*mul(194,970)when()from()$who()mul(79>%don't()*~ :select()&mul(340,662) -<@;where()++)what()mul(367,860)}:/;why()/,-mul(973,473)what()-{)mul(775,358);what():@select()do())(from()]@<what(216,350)why()who()@mul(296(when(),&mul(441,314),++{:*who()mul(458,712)?]'$select()why()[@mul(228,934)select(){#%mul(800,434)mul(32,355)mul(895,206)$':from()!mul(238,945),}[?}$why()<mul(904,931)*$mul(948,562)where()]why(601,835):!!*}select(280,167)mul(541,249)who()<-~who()mul(323,779)>(%@when(764,373)mul(60,926)mul(998,76)[how()*/do()@who()*from()<+]^select()&mul(2,30)
)who()select();mul(121,859)from()@},who():mul(606,797)from();what()what()*&,)*mul(478,201):/<;&from()}select(509,683)^mul(442,760) ],how()@ where()mul(325,315)mul(377,534)+when(953,408)how()/who()$@select()select(82,159)where()mul(590,633)how()>'why(136,599)&$~why()mul(44,530)*$mul'$what()+mul(634,476)[>{~-do()$mul(689,54)from()~+#%{,+!what()mul(360@[>/how()from()#,;mul(794,106)why()why()mul(822,201)}{)~?how()mul(440,483)where()who(356,874)+how(274,674);mul(114,558))how(),]})]}</mul(397,596),~<![#,:(do()%who()mul(969,996)~{/@how()],select()@mul(666,207),mul(692,414who();^)what()</ ]]mul(447,106)who()]>}^)&+:%mul(85,419)mulfrom(),why()))#[<?mul(162,336)!;[[->^how()#mul(711}*&:do()mul(832,303)what(720,682)}]^mul(848,34)#{)!,where()why()select()/mul(297,312);*(: <why(67,348)) }mul(167,517)) &+$where()&(*do()~#who()?&#});when(93,740)mul(516,233^: {#?&mul(549,677)&%:mul(506,998)where()~[/when()-<:,?mul(622,56),mul(710,886^mul(629,502)why()${;],{mul(7,729)why():#'>why();&[select()mul(835,973) >mul(976,433)where();'}?]@select()mul(613,866)-how()#mul(121,104)!-mul(25,379) ]*why(){~+mul@^~where()*[who()(/@mul(256,212)~what()mul(855,364) !?;mul>:/from(945,813)/mul(996,850)!&(mul(369,875)%<mul(891,144);from()%]<'<mul(631,980)[#~%why(),~mul(705,849)how()%+?;>mul(917,57)-select();mul(868,942)]?~{[mul(352,436)!~how()-@mul(629,784),%<}~&mul(648,951)from(153,561))&$why(917,532)mul(640,970)why():[ ?%mul^mul(428,879))what()mul(199,603))mul(845,646)!$:!,?[select()mul(792,917]when(),[*<mul(422,243)}select()who()>&+:,>mul(283,734);mul(799,547)<^from()@mul(113,216)@{^ mul@mul(679,251)[*mul(899,455)why() select()/[%mul(4,45)(*^what()^:select(923,55)-#<mul(701,340)who()<don't()!^)-@[mul>mul(191,622)when()'?mul(784,659)/where()$:where(46,726)why()/!];mul(96,630)mul(601,327)%;,mul(336%where()^#>;why():/mul(754,451)select()how(491,36){%!}]mul(868,7)$select()! $where()select()&$mul(814,614)mul(535,117)%:#;{]!{(mul(537,533)-!%>#mul(794,183)mul(135,122)!(>,)do()~'who()from(504,62);what(){mul(959,794)!}-(mul(890,934)>'what()from()mul(412,476)^~mul(852how()mul(277,740)!!<#%^]$where()mul(694,253)#~>@{mul(570@]}!&@$%select()'mul(492,943)where()/mul(893,661)}mul(343,224)from(178,398)!#what()+&^^who(){mul(197,133)from()mul(334,570)mul(374,818),&^ $[don't()mul(781,459)why()from(959,545)-;[@mul(147,509)select()!}when()mul(778,892)what()#mul(960,67)<[from(),select()!?how()mul(376,145)-!; (+from()> ^mul(780,339):,$what()+mul(491,818)+>*^>@how()[what(879,204)how()mul(70!'@$#,how()>mul(492,341)/from()?&<+?select()[how(707,662)mul(40,129)]where()select(501,969)-}+mul(651,382)mul(582,658)&>%<from()(>#who()mul(27,768)!how()when()who()why()mul(860,651)%^[from()*where())(mul(2,858)select()}#];$mul(654,738)mul(538,332)[{(@mul(270,261)]/~mul(756,771)/mul(311,298)how();]~>;{<mul(663,677)+>?mul(260,313)<}~;from()}why()!}mul(902,613)mul(458,68)@:how()when()@)[^from()>mul(651,358) when()?]select():%)~*mul(769,230)*%%how()<]from()do()?who() :!,mul(256,752)#select():who()what(){;+<&mul(429,732)[^~where()what(425,533),[',$mul(546,869)how() <mul(424,645)why()'(((mul(233,853)
,:?[:}how():*mul(356,31)$~>what()do()who()!-#] +mul}}[#who()from()&-+*mul(613,565)#&-)<select()-mul(653,985)!#;mul(432,634),what()$/$[<don't()mul(203*${?]:>?mul(49,724)mul(250,187)mul(389,591)+from()*why()%:why()where()when()(mul(732,636)%~what()mul(170,116)+-}/%what()'mul(765,501)[^]?how()#mul(186,825))-}why()mul(302,601){]#where()*;from()mul(862,742)why()select()<where()where()mul(666,401)who()~who()[]+where()%^mul(874,116'mul(439,908)where()()where()when()!+mul(560,829),mul(286,115)}mul(273,259)+[#@*}mul(832,167)#<<(where()why()>!who()mul(51,136)$how()select()mul(116,301)mul*&(from()~> mul(456,87)?${%#mul(796,323),]>why()mul(622,227)']{~<how()mul(518,891)where()what()^]select()~'who(140,499)mul(616,446)mul(849,298)<+!+~]^where()mul(777,50)}%''<]+mulfrom(527,421)~mul(880,854)$+>'+where(){#-mul(37,539)?<)]why():[>mul(369,873) who()%from(342,759)&don't()}why()?mul(472,266)when()mul(413,298)when()[]$!@mul(739,890)+$'*who()mul(215,821)how()}%what()how()^mul(837,240)+}mul(882,381),;select()'~>[]}{mul(772,663)[^%,])who(){mul(398,367);}how() ,-who()-{who()mul(964,744)(mul(90,658)]<{[when()^@mul(216,425)}?[!mul(374,641);how(){]#^~where()from()mul(909,90)>:@{mul(491,477)<what()mul(338,691)(*why()-)?[mul(844,138)&+)mul(3$) mul(892,565)mul(913,244)**where()<who()/-&@*do()]&&&what()({![mul(239[why()]mul(643,590)/*-(mul(79,823)where(),%?~-~where()select(255,453)where()mul(634,892)-{ &how()[ <mul(50,27)+^:when()]+,*mul(706,164)#,~from(),>@$mul(932,629){][#+mul(993,863)]mul(405,521),+why(25,778)mul)@why(986,941)who()] >how())mul(163,617):-don't()when()? mul(144,275)>-when()/'~don't(){mul(111,133)],where(): }mul(351,780)select()why()mul(325,938)} where()#/'where()[what(){mul(936,880)select()%]$?^)@>mul(806?:what(){what()$who(992,398)@how()}mul(507,635);,when()/what(83,275)}?*;mul(905,828)+where()how()!mul(59,265)$>: ]},:-!do()when()where() why()<'*^mul(262,857/~@'{!what()$/select()'mul(669,373)mul(990,115)*select();{(mul(364,943)>> &{when()mul(338,512)>( #:mul(761,640)what()select()don't(),^select();why()'><when()]mul(942,498)what()$)why()mul(342,38)!?<@mul(959,310)**@mul(919,890)@!select()where()from()[where()what()+mulwhy()what(),>+-($?what()@mul(774,119)-how()!select()mul(578,919)when()>>-mul(744,442) -)^why()-mul(194,582)mul(112,346)+!from()mul(75,217)what()<don't()(<%why()mul(738$'^] '?*'&^mul(485,611)+who()&!-mul(970,390)who()select()@how()}!mul(748,964)'why()[how()select(),select()&who(),mul(731,138)}^&]>how()*@,mul(781,19)!]how()<?;<;mul(573,28);%$% ^%}mul(698,965)mul(440,100)<,<why()]:select()>}mul(840#~%+~~)mul(564,143)[% /<~}/mul(575,875)mul(386,110)*#{mul(913,496);why()-}mul(608,777)?@+>mul(264,527);;**mul(906,263)(':don't()mul(452,213)@when())when():(who()$!why()do()$}(!%[from() -mul(198,318)::)<<,&mul}[%#mul(919,501)/~#~mul(332,157)~<[select()mul(619,931),how():%where();how()mul(633,846) from()>}how(),%mul(456,751)when(), mul(896,30)~{how()]where()from()(why()+don't()![#!from()(-~mul(679,191)@+<select(),!who()mul(441,683)!mul(481,982)<how()/who()'what():mul(39,637)?what()select())mul(124,424)*@)]$' mulhow()~[{{'#!/usr/bin/perl!]where()mul(897,698)
{select()[[),:[mul(28,169)]^/~from(),when()'!mul(929,62))mul(2,658)$>why(950,435)-mul(563,357)why()from()@who(),select(666,833)who()+mul(641*mul(110,352)-(from(257,512)select()%who()don't(){'mul(997,134)%**$mul(180,648)*when() -:/</mul(276,397)where()select()when()<how()why(){<']mul(425,558)why()&?:%)mul(259,204)]@[who()mul(349,64)what()$[?/))who()+how()mul(569,112)when()mul(831*{;[]@select()#'%don't()%mul(517~/&@mul(696,90)&)how()-mul(488,361)/@when()how():mul(696,154)when()select()]]?}#,what()who()mul(625,403):who(602,811)-&)select()'mul;@%mul(598,280)]!&![% do()!^select()'-mul(674,329)*why()-mul(336,597)*'$what()mul(762,169):#~#~>>!from(){mulselect()'[}?mul(760,580); </who()&don't()! (,<,mul(822,761)where()#what(),mul(621,508)why()$>mul(565,110)how(437,37)who()>-from()mul(171,296),/(select()!>!([]mul(951,550)&~&<what()'mul(78,890)how()#{how()who()!mul(858,123)-mul(808,41)}!<$;'select()mul(755*why()![ ^why()mul(178,567)^#$^who()-!mul(21,680)*how()-?mul(916,174) >,/(/mul(813,539)*+when(887,78)where()&why() / ?do()from()#:@}}~+$&mul(708,100);{'mul(490,889)mul(307,49)%}when()(#$why()mul(663,680) [$?[from()>don't()when()mul(503,241)///{!{select()mul(823,841)why()!>@>{!%@?mulwhen()+>?,>mul(244,508)~~}@mul(350,498){*from()-%from()mul(153,341)</where()*how()}*(select()mul(103,232))!%from();%+@?mulwhen()*when()(select()(#:@' do()'mul(826,640)~select()<who()>select(595,211)mul(790,795:,mul(906,897)%,$'why()[*&/>mul(8,207from()(#+who()don't()mul(459,429){@)select()how()why(),:#mulwhat()#who()what(121,599)%(how()-'mul(624,45)[+mul(808,781)>-@who()who(){where()}mul(924,821)what()$what(),:;~-mul(706,810)@-+what()&*'!mul(355,925)?how()]**from()from()*$*mul(741,733)how()'mul(341,144)<$select()from()@+do()>mul(152,910/select()~ select()]]{@mul(523,181)#+{mul(395,973)who()mul(64,860)?when()<from()don't()')why()(~mul(642,919)mul(689,633)>&mul(29,453)&mul(984,624)'~-mul(359,973)~what()mul(383,158)/(~mul(921,686)how(632,127) from()how() <from()))why()mul(407,977))what()+~+'{mul(638,303)(?why()mul(887,590)how()mul(508,492)@%^${}+mul(613,829)(what()where()%mul(488,38)do()+}~+what(538,242) %mul(813,912)<]mul(103,950)select()}who() who()#[mul(215,783)from()?(^mul(25,319)##&?,mul(922@when())/)%why()mul(544,417)what()(^{mul(137,343)}>:why()-/#^{where()mul(917,718)[]mul(434,737);+$*$mul(642,210))(why(),+mul(172,967)-$(!)from(268,882)(!mul(736,352)how(955,746)what()?&%mul(913,683)^mul(917,50)(#>'->&select() mul(7,425)@ (&?> <++do()^#-mul(116,337)*mul(65,555)where()) (#})?-mul(914,671)/;;${who()mul(909,642)from()& mul(118,302)when()^,do()+$:&mul(317,868)!<select() +]mul(734,951)who()what(607,933)why()^/*+what()when()don't()]> &@mul(797,580)when()from();?how(),who()!?mul%'mul(72,803)mul(623,829)?from()#~){mul(369,29)from()(('->}why()&why()mul(559,871)#)/how(454,608)who(299,39)what()why()from()mul(170,324)@/}mul(609,232)>,]<&from(){&{mul(573,297)(when()'[mul(113,821)where()where()what()$*mul(645,755)when()how()'{how()$-select()$mul(691,961)when()(-@mul(506,24)who():!},mul(271,406) mul(484,521)select()when(130,935)!,[mul(260,340)select()how()when()mul(271,829)select()'from(){[-}'-{don't()when():-who()what()$+mul(939,919)&}$-what()mul(527,489)<why()%}+when(826,514)mul(547,624?what()?<'-$*how(),mul(281,242)?:[who()when()when()select()mul)@mul(572,52)
from()-^#*<)mul(704,198)*>what()>}{mul(992,257)*select() mul(613,810)why()?@*select()&;&@~mul(403,823)-*,](/why()mul(720,443)/$#how()who(){^mul(241,87)mul(650,931)<& -what()%!mul(547,874)]mul!mul(434,880)mul(775-!^where()$^,~from(40,598)what()where()don't() @how(519,628)from()(mul(302,37)<who()+mul(342,272)(>'mul(781,582)when()*select()):>~mul(779,697)>%'mul(943,75)what()!+#<(;]when()'do()what()mul(102,269)/@?(&#-who(542,407)mul(509,667select(274,878)^~<>how()#why()&#mul(268,871)[%why()~/[ ~!select()mul(767,191)>where()where()mul(933,796)$#%-who()**@-mul(203,818)where()mul(799,499)[*,+mul(909,701){$,mul(46,448)/][&}[,mul(310,979)from(),*;?-<when()mul(343,376),select() [{when(986,80)< ;mul(819,963)!mul(776,355)$who(128,894):,$~:mul(170,751)from()!*,where()}]mul(942,455)?{</@&,$mul(45,703)}<$''~[<mul(880,946);)?:)?+mul(746,737)-from()~do()when())mul(918,557):$&mul(710,332)from()<&/!what()mul(715,606),~^:how()when()~/&mul(438,91)+%~!~where()don't()((mul(457,120)when(121,808){(:^what()*from()mul(756why()from()]#]when()mul(399,120)<$@why()#from(288,288)]$who()mul(310,496)}mul(720,252)@#<:[]{mul(98@@%< mul(770,72)*why()mul(664,585)from()]+mul(737,351)!~$$)who()(mulwhere()how(267,616)where(62,517)how(){,),-[select(521,287)mul(858,912)how()-,-&mul(711,711)what()what()select(){^don't(),*~@mul(201,501)where()}'don't();-}+:mul(595,333) :~#,mul(95+<from()[:>$*%what()mul(34,562) what()'why(579,402)what();do()?/@/> </*what()mul(188{~how()mul(142,899)){#*<[ where(),mul(657,790)#who()why()mul(573,341)what()>&mul(230select()(,why(){{mul(780,887)?[@; mul(766,478)what()/ ?;do()/]-&when();  when():mul(426,588)[+ mul(687,427):!:$why()/who()who()from()^do()*>'mul(537,242),-;,#where()/~when()mul(362,214<what()^*how()mul(503,89))mul(773,824)]from()*;where()mul(758,993)+:why()what()mul(400,725)++{*[who(),:mul(707,292))do()mul(705,577>[where()/$#+where() mul(967,770)-+?/##>#mul(749,239)where()?(/why()#[mul(767,98)>+@^;#mul(378,187))%(< (do()@mul(958,879)[;#what()~^from()mul(892,531)( :&where()mul(173,372)mul(11,471)(where()where()}{$from()from()'mul(887,95)when()select():*$%mul(534,793)(^mul~~who()~when()from()%{>why()mul(507,869)where()where()<:mul(772,705)*who()what()do()how()/$mul(14,754)mul(966,261)when()>who()%& mul(795,477)$(:mul(381,47)^< select()from()mul(963,221)'mul(918,106)'what(344,90)who()where()@do())&mul(433,764)#(]/'&^, ~mul(22,392)<mul(380,322)mul(47,165)how()(>mul(998,135));how()~mul(280,141)mul(872,362)when())+who()what()#mul(342,86)]&-+<+^mul(996,141)>,*mul<how()+#from()%$why(520,611)mul(494,82)what(768,970)&mul(69,536)>)where()'/~*+mul(389,594)how()'where()don't()who()*&who()mul(962,541)mul(752,696)+mul(659,926);}when(){~!,^^who()mul(401,356)how()]&:}when()mul(84,233)where()<@/((}@*mul(149,810)how()'who()/>from()mul(535,710)who(358,296)mul(850<*@select()mul(695,834)why()$&select()when()*mul(225,980) )}mul(491,900),from()[{why()/from();mul(868,47)}%*select()'<!mul(877,658)^why(469,725)who()#;:don't()~)?mul(539,174)why()[where()'$}%<mul(64,491)/(where()mul(178,455) /@mul(423,219)mul(740,720)[who()$]--mul(717,716)mul(529,364)&' <what()how()'#-who()mul(923,495)
)what(507,115)}<mul(343,622)*what():@~why(755,344)*select(786,77)<mul(253,810)! when() /select(400,692)mul(785,187)'$what(){mul(854,194)!why()where()%)from()<how()$:mul(978,677)why()-why()mul(707,444)+who()^mul(391,711)why()select()mul(221,80)-from(354,688),~*from()mul(697,532):%from()&mul(579,115)]^why()what()mul(81,717)]from()&from()<#@mul(715,125)~&)#from()why()who(629,830)]mul(845,52)where()mul(567,206)!select()*:]} ]mul(55,87)&why(),%&<$<:>mul(849,471)?$(mul(494,276)?*!-?!do(),what()*;mul(567,782)/^from()mul(905what()]]^((mul(75,486)where()where()select()mul(284,874)why()>mul(601,804))>]-from()from()where()]:&mul(330,34);{#mul(156,247)( ]+mul(999,763)$:mul(751,281){;*from()&?]mul(332,55)!-[&?mul(619,81)-?mul(96,239)?<>@#select()/mul(444,677)mul(618,319)why()%how()how(942,957))mul(748,564)}$++/mul(727,414)'mul(242,138)mul(931,661)@-what()mul(924,836)@mul(47,603)#>how()#mul(507,887)@/,#mul(146why()'don't(),))#who()select()select()when(374,670)mul(844,486]!-(where()^when()who()mul(415,866)%:%mul(544,385))/mul(426,967)where() +when();mul(20,241)]([@;[{who()}mul(90,379)where()}what()<what()where()select()[mul(436,687) )~)what()&mul(350,617)!why()>^@why()+,mul(20,933);*when()<?*;from()mul(491,830)<what()who(),where())[mul(821,946):how(712,542)mul(503,324)-$^*]mul(782,9)mul(981,296)&+who()$mul(763,721)]from(423,565)%how()^)^who()mul(207,493):-(where()}mul(357,921)+from()?who()(when()what()who()mul(179,292)what()don't()'[?]^}?)mul(520,59)-what()mul(670,986)]}select()select()>>&<^]mul(378,57){$mul(671,520)+mul(436,843)]mul(918,197)$[(:[#mul(280,176)%];>-(,why()why()when()mul(814,169)];+}mul(460,558)(*when()mul(948,922)}}?where()what()mul(751,374){+)@]mul(751,39)-:~~how()/!where()when(),mul(729,454)mul(76,943)mul(42,187)]:?#%mul(821,797)#%:[mul(329,209),#why()~$}select(302,971)when()?!mul(668,103)>when()%where()who()<^who()@/mul(882,873):$(&:@;' {mul(161,494)~mul(936,354);why()/how()why(186,585),;from(565,361)mul(642,336)[']*,!where()what(41,193)@mul(5,192)($$%mul(160,833)[@*where()mul(120,135)mul(301,599)?/mul(972,839):[from()who()#<how()mul(206,990)!>,[mul(936,580)*~why()?select()-/how()/-mul(781,329)from()}?'what()mul(838,340)?>#select()mul(40,712)>why()how(673,894)who()why()%mul(534,292)' /!mul(951,435)!?mul(656,237)^>%mul(937,63)from()/%#why()+>(#[mul(555,182)mul(956,211))}&?--~mul(101,767)@;who()()mul(639,997)@^{[</mul(31,930)&}how(993,334))%+:select()]why()mul(736,595)[&who()mul(54,475))from(204,393)'/<how()from()where()select()mul(769,146)mul(645,182):(^/{-:mul(654,200) ^how()'<!mul(40,655)'&^<mul(398,41):select()select(687,327)#@mul(688,746)&#-mul(973,150)-who(779,158)]'&what()mul(54,194)[who()$$when()-#>%/mul(658,528)mul(797,165)when(){@;what()who()(}mul(145,899)&{}:!#/mul(527,57)why(),]%-][$&,".to_string()
}

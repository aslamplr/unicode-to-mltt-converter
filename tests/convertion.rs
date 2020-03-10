extern crate unicode_to_mltt_converter;

use std::error::Error;
use unicode_to_mltt_converter::convert_text;

fn get_map_content() -> Result<String, Box<dyn Error>> {
    Ok(std::fs::read_to_string("www/public/karthika.map")?)
}

fn test_convertion(input_text: &str, expected_text: &str) -> Result<(), Box<dyn Error>> {
    let map_content = get_map_content()?;
    assert_eq!(
        convert_text(&input_text, &map_content)?,
        String::from(expected_text)
    );
    Ok(())
}

#[test]
fn convert_case_1() -> Result<(), Box<dyn Error>> {
    test_convertion(
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890",
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890"
    )
}

#[test]
fn convert_case_short_1() -> Result<(), Box<dyn Error>> {
    test_convertion(
        r#"മലയാളം"#, 
        r#"aebmfw"#
    )
}

#[test]
fn convert_case_short_2() -> Result<(), Box<dyn Error>> {
    test_convertion(
        r#"കേരളം"#, 
        r#"tIcfw"#
    )
}

#[test]
fn convert_case_short_3() -> Result<(), Box<dyn Error>> {
    test_convertion(
        r#"കാണപ്പെടുന്ന"#, 
        r#"ImWs¸Sp¶"#
    )
}

#[test]
fn convert_case_short_4() -> Result<(), Box<dyn Error>> {
    test_convertion(
        r#"ഉൾപ്പെടുന്ന"#, 
        r#"DÄs¸Sp¶"#
    )
}

#[test]
fn convert_case_short_5() -> Result<(), Box<dyn Error>> {
    test_convertion(
        r#"ഫ്രൈ"#, 
        r#"ss{^"#
    )
}

#[test]
fn convert_case_short_6() -> Result<(), Box<dyn Error>> {
    test_convertion(
        r#"സ്ഥലത്തേ"#, 
        r#"Øet¯"#
    )
}

#[test]
fn convert_case_short_7() -> Result<(), Box<dyn Error>> {
    test_convertion(
        r#"കണ്ടെത്താനാവൂ"#, 
        r#"Is­¯m\mhq"#
    )
}

#[test]
fn convert_case_short_8() -> Result<(), Box<dyn Error>> {
    test_convertion(
        r#"ക്രിമിനൽ"#, 
        r#"{Inan\Â"#
    )
}

#[test]
fn convert_case_short_9() -> Result<(), Box<dyn Error>> {
    test_convertion(
        r#"പൊടുന്നനെ"#, 
        r#"s]mSp¶s\"#
    )
}

#[test]
fn convert_case_short_10() -> Result<(), Box<dyn Error>> {
    test_convertion(
        r#"കിളികൊല്ലൂർ"#, 
        r#"InfnsImÃqÀ"#
    )
}

#[test]
fn convert_case_short_11() -> Result<(), Box<dyn Error>> {
    test_convertion(
        r#"കോയമ്പത്തൂർ"#, 
        r#"tImb¼¯qÀ"#
    )
}

#[test]
fn convert_case_short_12() -> Result<(), Box<dyn Error>> {
    test_convertion(
        r#"മുക്കോലയ്ക്കൽ"#, 
        r#"apt¡mebv¡Â"#
    )
}

#[test]
#[ignore]
fn convert_case_short_13() -> Result<(), Box<dyn Error>> {
    test_convertion(
        r#"മക്രോണി"#, 
        r#"at{ImWn"#
    )
}

#[test]
fn convert_case_long_1() -> Result<(), Box<dyn Error>> {
    test_convertion(
        r#"കേരളത്തിൽ വിരളമായി കാണപ്പെടുന്ന ഒരിനം പൂമ്പാറ്റയാണ് മലബാർ മിന്നൻ (Rapala lankana). ഇന്ത്യയിലെ പശ്ചിമഘട്ടത്തിലെ വളരെ കുറച്ച് സ്ഥലത്തേ അതായത് കേരളം, തമിഴ്നാട്, കർണ്ണാടകം ഉൾപ്പെടുന്ന പ്രദേശങ്ങളിൽ ഇവയെ കണ്ടെത്താനാവൂ."#, 
        r#"tIcf¯nÂ hncfambn ImWs¸Sp¶ Hcn\w ]q¼mäbmWv ae_mÀ an¶³ (Rapala lankana). C´ybnse ]ÝnaL«¯nse hfsc Ipd¨v Øet¯ AXmbXv tIcfw, Xangv\mSv, IÀ®mSIw DÄs¸Sp¶ {]tZi§fnÂ Chsb Is­¯m\mhq."#
    )
}

#[test]
#[ignore]
fn convert_case_long_2() -> Result<(), Box<dyn Error>> {
    test_convertion(
        r#"കേരളത്തിലെ സാമൂഹിക പരിഷ്കർത്താക്കളിലൊരാളായിരുന്നു സഹോദരൻ അയ്യപ്പൻ (ജീവിതകാലം: 21 ഓഗസ്റ്റ് 1889 - 6 മാർച്ച് 1968). ഒരു ജാതി ഒരു മതം മനുഷ്യന് എന്ന ശ്രീനാരായണഗുരുവിന്റെ വാക്യങ്ങളെ സാക്ഷാത്കരിക്കാൻ ശ്രമിച്ച വ്യക്തിയായിരുന്ന സഹോദരൻ അയ്യപ്പൻ; ഓജസ്സ് നഷ്ടപ്പെട്ട അപകടകരങ്ങളായ ആശയങ്ങളെ നവീകരിച്ച് ജാതിരഹിതവും വർഗ്ഗരഹിതവുമായ പുതിയ ഒരു സമൂഹത്തെ സൃഷ്ടിക്കാൻ പരിശ്രമിച്ച ഒരു നവോത്ഥാനനായകൻ കൂടിയായിരുന്നു. ശ്രീനാരായണ ഗുരുവിന്റെ മനുഷ്യദർശനത്തെ ശാസ്ത്രീയതയുടെയും യുക്തിചിന്തയുടെയും അടിസ്ഥാനത്തിൽ വിപുലീകരിച്ച് രാഷ്ട്രീയ പ്രയോഗമാക്കി മാറ്റുകയായിരുന്നു അയ്യപ്പൻ. ഈഴവ സമുദായത്തിൽ ജനിച്ച അദ്ദേഹം, തൊട്ടുകൂടാത്തവരായി അവഗണിക്കപ്പെട്ടിരുന്ന ദളിതരെ ചേർത്ത് മിശ്രഭോജനം നടത്തി. സമൂഹത്തിലെ അന്ധവിശ്വാസങ്ങൾക്കും അനാചാരങ്ങൾക്കുമെതിരെ സന്ധിയില്ലാ സമരം ചെയ്തു. ശ്രീ നാരായണഗുരുവിന്റെ "ഒരു ജാതി ഒരു മതം ഒരു ദൈവം മനുഷ്യന് " ജാതി ചോദിക്കരുത്, പറയരുത്, ചിന്തിക്കരുത് എന്ന സുപ്രസിദ്ധമായ ആപ്തവാക്യം ഗുരുവിന്റെ അംഗീകാരത്തോടെ "ജാതി വേണ്ട, മതം വേണ്ട, ദൈവം വേണ്ട മനുഷ്യന്", എന്ന് അദ്ദേഹം ഭേദഗതി വരുത്തി. കൊച്ചി നിയമസഭയുടെ 1928-ൽ രണ്ടാം തിരഞ്ഞെടുപ്പിൽ തെക്കേ ഈഴവ മണ്ഡലത്തിൽ നിന്നും ജയിച്ച് അയ്യപ്പൻ നിയമസഭയിലെത്തി. നിയമസഭാസാമാജികൻ എന്ന നിലയിൽ അവശരേയും പാവങ്ങളേയും ഉദ്ധരിക്കാൻ ആവശ്യമായ പ്രവർത്തനങ്ങളിൽ അദ്ദേഹം ഏർപ്പെട്ടു."#, 
        r#"tIcf¯nse kmaqlnI ]cnjvIÀ¯m¡fnsemcmfmbncp¶p ktlmZc³ A¿¸³ (PohnXImew: 21 HmKkväv 1889  6 amÀ¨v 1968). Hcp PmXn Hcp aXw a\pjy\v F¶ {io\mcmbWKpcphnsâ hmIy§sf km£mXvIcn¡m³ {ian¨ hyànbmbncp¶ ktlmZc³ A¿¸³; HmPkvkv \ãs¸« A]ISIc§fmb Bib§sf \hoIcn¨v PmXnclnXhpw hÀ¤clnXhpamb ]pXnb Hcp kaqls¯ krãn¡m³ ]cn{ian¨ Hcp \thm°m\\mbI³ IqSnbmbncp¶p. {io\mcmbW Kpcphnsâ a\pjyZÀi\s¯ imkv{XobXbpsSbpw bpànNn´bpsSbpw ASnkvYm\¯nÂ hn]peoIcn¨v cm{ãob {]tbmKam¡n amäpIbmbncp¶p A¿¸³. Cugh kapZmb¯nÂ P\n¨ At±lw, sXm«pIqSm¯hcmbn AhKWn¡s¸«ncp¶ ZfnXsc tNÀ¯v an{it`mP\w \S¯n. kaql¯nse AÔhnizmk§Ä¡pw A\mNmc§Ä¡psaXnsc kÔnbnÃm kacw sNbvXp. {io \mcmbWKpcphnsâ "Hcp PmXn Hcp aXw Hcp ssZhw a\pjy\v " PmXn tNmZn¡cpXv, ]dbcpXv, Nn´n¡cpXv F¶ kp{]kn²amb B]vXhmIyw Kpcphnsâ AwKoImct¯msS "PmXn th­, aXw th­, ssZhw th­ a\pjy\v", F¶v At±lw t`ZKXn hcp¯n. sIm¨n \nbak`bpsS 1928Â c­mw XncsªSp¸nÂ sXt¡ Cugh aWvUe¯nÂ \n¶pw Pbn¨v A¿¸³ \nbak`bnse¯n. \nbak`mkmamPnI³ F¶ \nebnÂ Ahitcbpw ]mh§tfbpw D²cn¡m³ Bhiyamb {]hÀ¯\§fnÂ At±lw GÀs¸«p."#
    )
}

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

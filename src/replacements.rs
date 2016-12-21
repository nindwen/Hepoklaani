extern crate regex;
use self::regex::Regex;

// The magic lives here
pub fn content_replace(content: String) -> String {
    // R G B => B G R for nice brown/pinkish theme
    let css_regex = Regex::new(r"#(?P<r>[A-Fa-f0-9]{2})(?P<g>[A-Fa-f0-9]{2})(?P<b>[A-Fa-f0-9]{2});").unwrap();
    css_regex.replace_all(&content, "#$b$g$r; /* changed */")

        // General
        .replace("bioklaani.fi",::DOMAIN)
        .replace("Bio-Klaani","Hepoklaani")
        .replace("Klaanon","Hevoset the fanfic")
        .replace("Klaanilehti","Hevossanomat")
        .replace("Bio-Logi","Heppapäiväkirja")
        .replace("ELKOM","SUURI HEVONEN")
        .replace("Kirjaudu sisään</a></h2>","Kirjaudu sisää</a></h2>Hepoklaanin taikahevoset huomauttaa että jos et täysin luota hepoklaanin taikahevosiin, kirjautuminen on teoriassa vaarallista. Boop.")

        // Users
        // (Some names are replaced multiple times,
        // for example alt. nick -> primary nick -> horsefied nick)
        .replace("Guardian","Shit Biscuit")
        .replace("Don","HooKoo")
        .replace("Matoro TBS","Matoro")
        .replace("Matoro","Warhistory Sparklehoof")
        .replace("MaKe@nurkka|_.)","Make")
        .replace("Make","Hepo@talli|🐎")
        .replace("Kerosiinipelle","Nanohep")
        .replace("Igor","Hegor")
        .replace("Kapura","Reptiliaanihevonen")
        .replace("Tongu","Keetongu")
        .replace("Keetongu","Aikahevonen")
        .replace("Visu","Visokki")
        .replace("Visokki","Kahdeksanjalkainen hevonen")
        .replace("Manu","Manfred")
        .replace("Manfred","Horsfred")
        .replace("Umbra","Dr.U")
        .replace("Dr.U","Heppatohtori")
        .replace("Tawa","Menkää Nukkumaan")
        .replace("Snowman","Snowie")
        .replace("Snowie","Lumihevonen")
        .replace("Killjoy","Horsejoy")
        .replace("Nenya","Neny")
        .replace("Neny","Lumiharja")
        .replace("Domek the light one","Domek")
        .replace("Domek","Heppataikatyttö")
        .replace("Paavo12","Pave")
        .replace("Pave","Ravitutkija")
        .replace("Suga","Heavy Metal Poica")
        .replace("Meistä","Hevosista")
        .replace("Baten","Hevosen")
        .replace("Bate","Hevonen")
        .replace("susemppu","Hevonen")

        // Images
        .replace("img src=\"./download/file.php?avatar=" ,"img src=\"https://files.nindwen.blue/hepoklaani/hepoava.png\" alt=\"")
        .replace("/headers/","https://files.nindwen.blue/hepoklaani/hepoklaani.png")
        .replace("/images/background2.png","https://files.nindwen.blue/hepoklaani/unicorn_bg.gif")
        .to_string()
}

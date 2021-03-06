extern crate regex;
use self::regex::Regex;

// The magic lives here
pub fn content_replace(content: String) -> String {
    // R G B => B G R for nice brown/pinkish theme
    let css_regex =
        Regex::new(r"(?x)
                   \#(?P<r>[A-Fa-f0-9]{2})
                   (?P<g>[A-Fa-f0-9]{2})
                   (?P<b>[A-Fa-f0-9]{2})
                   ( !important)?;
                   ").unwrap();
    let css_replaced = css_regex.replace_all(&content, "#$b$g$r;");

    // HEVOSIA
    let rank_regex = Regex::new(r"/images/ranks/.*\.png").unwrap();
    rank_regex.replace_all(&css_replaced, "/images/ranks/hevosia.png")

        // General
        .replace("bioklaani.fi",::DOMAIN)
        .replace("Bio-Klaani","Hepoklaani")
        .replace("Klaanon","Hevoset the fanfic")
        .replace("Klaanilehti","Hevossanomat")
        .replace("Bio-Logi","Heppapäiväkirja")
        .replace("ELKOM","SUURI HEVONEN")
        .replace("Kirjaudu sisään</a></h2>","Kirjaudu sisää</a></h2>Hepoklaanin taikahevoset muistuttaa että kirjautuminen hämärille sivuille toisen sivun tunnuksilla on yleisesti ottaen aika tyhmää/vaarallista/boop. Toisaalta taas se toimii joten pitäkää hauskaa.")

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
        .replace("Manfred","Pink Fluffy Unicorn")
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
        .replace("suga","Suga")
        .replace("Suga","Heavy Metal Poica")
        .replace("Ju0pp0","Janoinen hevonen")
        .replace("Kyösti","Hevonen Karjalasta")
        .replace("Taiksie","Avoshedmin")
        .replace("Jake","Elektroninen hevonen")
        // This breaks the code
        //.replace("BD","Melko Ei Hevonen")
        .replace("Peelo","Miten hevonen edes housut")
        .replace("Blezer","Hevonen joka on oppinut hallitsemaan magiaa")
        .replace("Meistä","Hevosista")
        .replace("Baten","Hevosen")
        .replace("Bate","Hevonen")
        .replace("susemppu","Hevonen")

        //Klaanon
        .replace("Nimda","MacPorkkana")
        .replace("Avde","Ilkeä hevonen")
        .replace("noita","taikahevonen")
        .replace("noidan","taikahevosen")
        .replace("noidaksi","taikahevoseksi")
        .replace("toa","sotaratsu")
        .replace("matoran","pikkuinen hevonen")
        .replace("turaga","viisas hevonen")
        .replace("Toa","sotaratsu")
        .replace("Matoran","pikkuinen hevonen")
        .replace("Turaga","viisas hevonen")
        .replace("ZMA","Zorak")
        .replace("Zorak","Orkesterinjohtajahevonen")
        .replace("Feterr","Heporr")

        // Images
        .replace("img src=\"./download/file.php?avatar=",
                 "img src=\"https://files.nindwen.blue/hepoklaani/hepoava.png\" alt=\"")
        .replace("/headers/",
                 "https://files.nindwen.blue/hepoklaani/hepoklaani.png")
        .replace("/images/background2.png",
                 "https://files.nindwen.blue/hepoklaani/unicorn_bg.gif")
        .replace("/wp/wp-content/themes/klaanon/header-images/header.php",
                 "https://files.nindwen.blue/hepoklaani/klaanon_header.png")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn css_replace() {
        let test = "kissa lol #12ab89; lol".to_string();
        let correct = "kissa lol #89ab12; lol".to_string();
        assert_eq!(content_replace(test), correct);
    }

    #[test]
    fn css_negative() {
        let too_short = "#abcde;".to_string();
        assert_eq!(content_replace(too_short.clone()), too_short);

        let too_long = "#abcdef1;".to_string();
        assert_eq!(content_replace(too_long.clone()), too_long);

        let hashtag = "#vapaus; lisäksi lol".to_string();
        assert_eq!(content_replace(hashtag.clone()), hashtag);

        let nonhex = "#hklk54".to_string();
        assert_eq!(content_replace(nonhex.clone()), nonhex);
    }

    #[test]
    fn general() {
        let test = "Bio-Klaanissa asuu ELKOM".to_string();
        let correct = "Hepoklaanissa asuu SUURI HEVONEN".to_string();
        assert_eq!(content_replace(test), correct);
    }

    #[test]
    fn both() {
        let test = "Bio-Klaanissa asuu ELKOM, sen lempiväri on: #66Ae8F;".to_string();
        let correct = "Hepoklaanissa asuu SUURI HEVONEN, sen lempiväri on: #8FAe66;".to_string();
        assert_eq!(content_replace(test), correct);
    }
}

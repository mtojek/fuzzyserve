use crate::find_best_match;

fn test_files() -> Vec<String> {
    vec![
        "Die Hard 2 (1990) [1080p] {5.1}/Die.Hard.2.BluRay.1080p.x264.5.1.Judas.mp4",
        "Die Hard (1988) [1080p] {5.1}/Die.Hard.BluRay.1080p.x264.5.1.Judas.mp4",
        "Lethal Weapon 4 (1998) [1080p]/Lethal.Weapon.4.1998.1080p.BrRip.x264.BOKUTOX.YIFY.mp4",
        "Lethal Weapon 3 (1992) [1080p]/Lethal.Weapon.3.1992.1080p.BrRip.x264.BOKUTOX.YIFY.mp4",
        "Lethal Weapon 2 (1989)  [1080p]/Lethal.Weapon.2.1989.1080p.BrRip.x264.BOKUTOX.YIFY.mp4",
        "Lethal Weapon (1987) [1080p]/Lethal.Weapon.1987.1080p.BrRip.x264.BOKUTOX.YIFY.mp4",
        "The.Simpsons.S37E06.720p.HDTV.x264-SYNCOPY[EZTVx.to].mkv",
        "The Simpsons S37E02 The Yellow Lotus 1080p DSNP WEB-DL DD 5 1 H 264-playWEB[EZTVx.to].mkv",
        "The.Simpsons.S37E05.720p.HEVC.x265-MeGusta[EZTVx.to].mkv",
        "The.Simpsons.S37E07.1080p.WEB.h264-EDITH[EZTVx.to].mkv",
        "The.Simpsons.S37E08.1080p.WEB.h264-EDITH[EZTVx.to].mkv",
        "The.Simpsons.S37E09.1080p.WEB.h264-EDITH[EZTVx.to].mkv",
        "The.Simpsons.S37E10.1080p.WEB.h264-EDITH[EZTVx.to].mkv",
        "The.Simpsons.S37E11.Parahormonal.Activity.720p.HEVC.x265-MeGusta[EZTVx.to].mkv",
        "The.Simpsons.S37E12.1080p.WEB.h264-EDITH[EZTVx.to].mkv",
        "The Simpsons S37E01 1080p x265-ELiTE[EZTVx.to].mkv",
        "Doctor.Who.2005.S01.COMPLETE.720p.BluRay.x264-GalaxyTV[TGx]/Doctor.Who.2005.S01E01.720p.BluRay.x264-GalaxyTV.mkv",
        "Doctor.Who.2005.S01.COMPLETE.720p.BluRay.x264-GalaxyTV[TGx]/Doctor.Who.2005.S01E02.720p.BluRay.x264-GalaxyTV.mkv",
        "Doctor.Who.2005.S01.COMPLETE.720p.BluRay.x264-GalaxyTV[TGx]/Doctor.Who.2005.S01E03.720p.BluRay.x264-GalaxyTV.mkv",
        "Doctor.Who.2005.S01.COMPLETE.720p.BluRay.x264-GalaxyTV[TGx]/Doctor.Who.2005.S01E04.720p.BluRay.x264-GalaxyTV.mkv",
        "Doctor.Who.2005.S01.COMPLETE.720p.BluRay.x264-GalaxyTV[TGx]/Doctor.Who.2005.S01E05.720p.BluRay.x264-GalaxyTV.mkv",
        "Harry Potter and the Sorcerers Stone (2001) [1080p]/Harry.Potter.and.the.Sorcerers.Stone.2001.1080p.BrRip.x264.YIFY ( FIRST TRY).mp4",
        "Harry Potter and the Chamber of Secrets (2002) [1080p]/Harry.Potter.and.the.Chamber.of.Secrets.2002.1080p.BrRip.x264.YIFY.mp4",
        "Harry Potter and the Prisoner of Azkaban (2004) 1080p.BRrip.scOrp.sujaidr (pimprg)/Harry Potter and the Prisoner of Azkaban (2004) 1080p.BRrip.scOrp.sujaidr (pimprg).mkv",
        "Harry Potter and the Goblet of Fire (2005) [1080p]/Harry.Potter.and.the.Goblet.of.Fire.2005.1080p.BrRip.x264.YIFY.mp4",
        "Harry Potter and the Order of the Phoenix (2007) [1080p]/Harry.Potter.and.the.Order.of.the.Phoenix.2007.1080p.BrRip.x264.YIFY.mp4",
        "Harry Potter and the Half Blood Prince (2009) [1080p]/Harry.Potter.and.the.Half.Blood.Prince.2009.1080p.BrRip.x264.YIFY.mp4",
        "Harry Potter and the Deathly Hallows Part 1 (2010) [1080p]/Harry.Potter.and.the.Deathly.Hallows.Part.1.2010.1080p.BrRip.x264.YIFY.mp4",
        "Harry Potter and the Deathly Hallows Part 2 (2011) 1080p.BRrip.scOrp.sujaidr (pimprg)/Harry Potter and the Deathly Hallows Part 2 (2011) 1080p.BRrip.scOrp.sujaidr (pimprg).mkv",
        "Home Alone (1990) [1080p]/Home.Alone.1990.1080p.BluRay.x264.YIFY.mp4",
        "Home Alone 2 Lost in New York (1992) [1080p]/Home.Alone.2.Lost.in.New.York.1992.1080p.BluRay.x264.YIFY.mp4",
        "Jumanji (1995)/Jumanji.1995.720p.BrRip.x264.BOKUTOX.YIFY.mp4",
        "Jumanji Welcome to the Jungle.2017.1080p.WEB-DL.6CH.MkvCage.mkv",
        "Jumanji The Next Level (2019) [720p] [BluRay] [YTS.MX]/Jumanji.The.Next.Level.2019.720p.BluRay.x264.AAC-[YTS.MX].mp4",
        "Indiana Jones and the Last Crusade (1989) [1080p]/Indiana.Jones.And.The.Last.Crusade.1989.1080p.BluRay.x264.YIFY.mp4",
        "Indiana Jones and the Kingdom of the Crystal Skull (2008) [1080p]/Indiana.Jones.And.The.Kingdom.of.the.Crystal.Skull.2008.1080p.BrRip.x264.YIFY.mp4",
        "Indiana Jones And The Dial Of Destiny (2023) [1080p] [WEBRip] [5.1] [YTS.MX]/Indiana.Jones.And.The.Dial.Of.Destiny.2023.1080p.WEBRip.x264.AAC5.1-[YTS.MX].mp4",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}

fn assert_match(query: &str, expected: &str) {
    let files = test_files();
    assert_eq!(find_best_match(query, &files), Some(expected))
}

#[test]
fn test_simpsons_s37e05() {
    assert_match("simpsons-s37e05", "The.Simpsons.S37E05.720p.HEVC.x265-MeGusta[EZTVx.to].mkv");
}

#[test]
fn test_jumanji() {
    assert_match("jumanji-1995", "Jumanji (1995)/Jumanji.1995.720p.BrRip.x264.BOKUTOX.YIFY.mp4");
    assert_match("jumanji-welcome", "Jumanji Welcome to the Jungle.2017.1080p.WEB-DL.6CH.MkvCage.mkv");
    assert_match("jumanji-2019", "Jumanji The Next Level (2019) [720p] [BluRay] [YTS.MX]/Jumanji.The.Next.Level.2019.720p.BluRay.x264.AAC-[YTS.MX].mp4");

    assert_match("jumanji-1", "Jumanji (1995)/Jumanji.1995.720p.BrRip.x264.BOKUTOX.YIFY.mp4");
}

#[test]
fn test_home_alone() {
    assert_match("home alone", "Home Alone (1990) [1080p]/Home.Alone.1990.1080p.BluRay.x264.YIFY.mp4");
    assert_match("home alone 1", "Home Alone (1990) [1080p]/Home.Alone.1990.1080p.BluRay.x264.YIFY.mp4");
    assert_match("home-alone-1990", "Home Alone (1990) [1080p]/Home.Alone.1990.1080p.BluRay.x264.YIFY.mp4");
    assert_match("home alone 2", "Home Alone 2 Lost in New York (1992) [1080p]/Home.Alone.2.Lost.in.New.York.1992.1080p.BluRay.x264.YIFY.mp4");
}

#[test]
fn test_lethal_weapon() {
    assert_match("lethal-weapon-1", "Lethal Weapon (1987) [1080p]/Lethal.Weapon.1987.1080p.BrRip.x264.BOKUTOX.YIFY.mp4");
    assert_match("lethal-weapon-2", "Lethal Weapon 2 (1989)  [1080p]/Lethal.Weapon.2.1989.1080p.BrRip.x264.BOKUTOX.YIFY.mp4");
    assert_match("lethal-weapon-3", "Lethal Weapon 3 (1992) [1080p]/Lethal.Weapon.3.1992.1080p.BrRip.x264.BOKUTOX.YIFY.mp4");
    assert_match("lethal-weapon-4", "Lethal Weapon 4 (1998) [1080p]/Lethal.Weapon.4.1998.1080p.BrRip.x264.BOKUTOX.YIFY.mp4");
}

#[test]
fn test_die_hard() {
    assert_match("die-hard-1", "Die Hard (1988) [1080p] {5.1}/Die.Hard.BluRay.1080p.x264.5.1.Judas.mp4");
    assert_match("die-hard-2", "Die Hard 2 (1990) [1080p] {5.1}/Die.Hard.2.BluRay.1080p.x264.5.1.Judas.mp4");
}

#[test]
fn test_harry_potter() {
    assert_match("harry-potter-prisoner", "Harry Potter and the Prisoner of Azkaban (2004) 1080p.BRrip.scOrp.sujaidr (pimprg)/Harry Potter and the Prisoner of Azkaban (2004) 1080p.BRrip.scOrp.sujaidr (pimprg).mkv");
    assert_match("harry-potter-deathly-1", "Harry Potter and the Deathly Hallows Part 1 (2010) [1080p]/Harry.Potter.and.the.Deathly.Hallows.Part.1.2010.1080p.BrRip.x264.YIFY.mp4");
    assert_match("harry-potter-deathly-2", "Harry Potter and the Deathly Hallows Part 2 (2011) 1080p.BRrip.scOrp.sujaidr (pimprg)/Harry Potter and the Deathly Hallows Part 2 (2011) 1080p.BRrip.scOrp.sujaidr (pimprg).mkv");
}

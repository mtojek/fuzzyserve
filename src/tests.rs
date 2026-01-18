use crate::find_best_match;

fn test_files() -> Vec<String> {
    vec![
        "Exploding Skyscraper 2 (1991) [1080p] {5.1}/Exploding.Skyscraper.2.BluRay.1080p.x264.5.1.Judas.mp4",
        "Exploding Skyscraper (1989) [1080p] {5.1}/Exploding.Skyscraper.BluRay.1080p.x264.5.1.Judas.mp4",
        "Deadly Buddy 4 (1999) [1080p]/Deadly.Buddy.4.1999.1080p.BrRip.x264.BOKUTOX.YIFY.mp4",
        "Deadly Buddy 3 (1993) [1080p]/Deadly.Buddy.3.1993.1080p.BrRip.x264.BOKUTOX.YIFY.mp4",
        "Deadly Buddy 2 (1990) [1080p]/Deadly.Buddy.2.1990.1080p.BrRip.x264.BOKUTOX.YIFY.mp4",
        "Deadly Buddy (1988) [1080p]/Deadly.Buddy.1988.1080p.BrRip.x264.BOKUTOX.YIFY.mp4",
        "The Yellows S37E01 1080p x265-ELiTE[EZTVx.to].mkv",
        "The Yellows S37E02 The Golden Temple 1080p DSNP WEB-DL DD 5 1 H 264-playWEB[EZTVx.to].mkv",
        "The.Yellows.S37E05.720p.HEVC.x265-MeGusta[EZTVx.to].mkv",
        "The.Yellows.S37E06.720p.HDTV.x264-SYNCOPY[EZTVx.to].mkv",
        "The.Yellows.S37E07.1080p.WEB.h264-EDITH[EZTVx.to].mkv",
        "The.Yellows.S37E08.1080p.WEB.h264-EDITH[EZTVx.to].mkv",
        "The.Yellows.S37E09.1080p.WEB.h264-EDITH[EZTVx.to].mkv",
        "The.Yellows.S37E10.1080p.WEB.h264-EDITH[EZTVx.to].mkv",
        "The.Yellows.S37E11.Spooky.Happenings.720p.HEVC.x265-MeGusta[EZTVx.to].mkv",
        "The.Yellows.S37E12.1080p.WEB.h264-EDITH[EZTVx.to].mkv",
        "The.Yellows.S37E13.1080p.WEB.h264-EDITH[EZTVx.to].mkv",
        "Time.Traveler.2006.S01.COMPLETE.720p.BluRay.x264-GalaxyTV[TGx]/Time.Traveler.2006.S01E01.720p.BluRay.x264-GalaxyTV.mkv",
        "Time.Traveler.2006.S01.COMPLETE.720p.BluRay.x264-GalaxyTV[TGx]/Time.Traveler.2006.S01E02.720p.BluRay.x264-GalaxyTV.mkv",
        "Time.Traveler.2006.S01.COMPLETE.720p.BluRay.x264-GalaxyTV[TGx]/Time.Traveler.2006.S01E03.720p.BluRay.x264-GalaxyTV.mkv",
        "Time.Traveler.2006.S01.COMPLETE.720p.BluRay.x264-GalaxyTV[TGx]/Time.Traveler.2006.S01E04.720p.BluRay.x264-GalaxyTV.mkv",
        "Time.Traveler.2006.S01.COMPLETE.720p.BluRay.x264-GalaxyTV[TGx]/Time.Traveler.2006.S01E05.720p.BluRay.x264-GalaxyTV.mkv",
        "Bobby Wizard and the Magic Rock (2002) [1080p]/Bobby.Wizard.and.the.Magic.Rock.2002.1080p.BrRip.x264.YIFY ( FIRST TRY).mp4",
        "Bobby Wizard and the Secret Room (2003) [1080p]/Bobby.Wizard.and.the.Secret.Room.2003.1080p.BrRip.x264.YIFY.mp4",
        "Bobby Wizard and the Escaped Criminal (2005) 1080p.BRrip.scOrp.sujaidr (pimprg)/Bobby Wizard and the Escaped Criminal (2005) 1080p.BRrip.scOrp.sujaidr (pimprg).mkv",
        "Bobby Wizard and the Fire Cup (2006) [1080p]/Bobby.Wizard.and.the.Fire.Cup.2006.1080p.BrRip.x264.YIFY.mp4",
        "Bobby Wizard and the Bird Club (2008) [1080p]/Bobby.Wizard.and.the.Bird.Club.2008.1080p.BrRip.x264.YIFY.mp4",
        "Bobby Wizard and the Mixed Blood Royal (2010) [1080p]/Bobby.Wizard.and.the.Mixed.Blood.Royal.2010.1080p.BrRip.x264.YIFY.mp4",
        "Bobby Wizard and the Spooky Artifacts Part 1 (2011) [1080p]/Bobby.Wizard.and.the.Spooky.Artifacts.Part.1.2011.1080p.BrRip.x264.YIFY.mp4",
        "Bobby Wizard and the Spooky Artifacts Part 2 (2012) 1080p.BRrip.scOrp.sujaidr (pimprg)/Bobby Wizard and the Spooky Artifacts Part 2 (2012) 1080p.BRrip.scOrp.sujaidr (pimprg).mkv",
        "Abandoned Child (1991) [1080p]/Abandoned.Child.1991.1080p.BluRay.x264.YIFY.mp4",
        "Abandoned Child 2 Forgotten in Manhattan (1993) [1080p]/Abandoned.Child.2.Forgotten.in.Manhattan.1993.1080p.BluRay.x264.YIFY.mp4",
        "Jungle Board Game (1996)/Jungle.Board.Game.1996.720p.BrRip.x264.BOKUTOX.YIFY.mp4",
        "Jungle Board Game Return to the Wilderness.2018.1080p.WEB-DL.6CH.MkvCage.mkv",
        "Jungle Board Game The Final Round (2020) [720p] [BluRay] [YTS.MX]/Jungle.Board.Game.The.Final.Round.2020.720p.BluRay.x264.AAC-[YTS.MX].mp4",
        "Artifact Hunter and the Final Quest (1990) [1080p]/Artifact.Hunter.And.The.Final.Quest.1990.1080p.BluRay.x264.YIFY.mp4",
        "Artifact Hunter and the Alien Skull (2009) [1080p]/Artifact.Hunter.And.The.Alien.Skull.2009.1080p.BrRip.x264.YIFY.mp4",
        "Artifact Hunter And The Wheel Of Fate (2024) [1080p] [WEBRip] [5.1] [YTS.MX]/Artifact.Hunter.And.The.Wheel.Of.Fate.2024.1080p.WEBRip.x264.AAC5.1-[YTS.MX].mp4",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}

fn assert_match(query: &str, expected: &str) {
    let files = test_files();
    assert_eq!(Some(expected), find_best_match(query, &files), "query: {}", query)
}

#[test]
fn test_yellows() {
    assert_match("yellows-s37e01", "The Yellows S37E01 1080p x265-ELiTE[EZTVx.to].mkv");
    assert_match("yellows.s37e01", "The Yellows S37E01 1080p x265-ELiTE[EZTVx.to].mkv");
    assert_match("yellows-s37e05", "The.Yellows.S37E05.720p.HEVC.x265-MeGusta[EZTVx.to].mkv");
    assert_match("yellows.s37e05", "The.Yellows.S37E05.720p.HEVC.x265-MeGusta[EZTVx.to].mkv");
    assert_match("yellows-s37e11", "The.Yellows.S37E11.Spooky.Happenings.720p.HEVC.x265-MeGusta[EZTVx.to].mkv");
    assert_match("yellows.s37e11", "The.Yellows.S37E11.Spooky.Happenings.720p.HEVC.x265-MeGusta[EZTVx.to].mkv");
    assert_match("yellows-s37e12", "The.Yellows.S37E12.1080p.WEB.h264-EDITH[EZTVx.to].mkv");
    assert_match("yellows.s37e12", "The.Yellows.S37E12.1080p.WEB.h264-EDITH[EZTVx.to].mkv");
    assert_match("yellows-s37e13", "The.Yellows.S37E13.1080p.WEB.h264-EDITH[EZTVx.to].mkv");
    assert_match("yellows.s37e13", "The.Yellows.S37E13.1080p.WEB.h264-EDITH[EZTVx.to].mkv");
}

#[test]
fn test_jungle_board_game() {
    assert_match("jungle-board-game-1996", "Jungle Board Game (1996)/Jungle.Board.Game.1996.720p.BrRip.x264.BOKUTOX.YIFY.mp4");
    assert_match("jungle-board-game-return", "Jungle Board Game Return to the Wilderness.2018.1080p.WEB-DL.6CH.MkvCage.mkv");
    assert_match(
        "jungle-board-game-2020",
        "Jungle Board Game The Final Round (2020) [720p] [BluRay] [YTS.MX]/Jungle.Board.Game.The.Final.Round.2020.720p.BluRay.x264.AAC-[YTS.MX].mp4",
    );
    assert_match("jungle-board-game-1", "Jungle Board Game (1996)/Jungle.Board.Game.1996.720p.BrRip.x264.BOKUTOX.YIFY.mp4");
}

#[test]
fn test_abandoned_child() {
    assert_match("abandoned child", "Abandoned Child (1991) [1080p]/Abandoned.Child.1991.1080p.BluRay.x264.YIFY.mp4");
    assert_match("abandoned child 1", "Abandoned Child (1991) [1080p]/Abandoned.Child.1991.1080p.BluRay.x264.YIFY.mp4");
    assert_match("abandoned-child-1991", "Abandoned Child (1991) [1080p]/Abandoned.Child.1991.1080p.BluRay.x264.YIFY.mp4");
    assert_match(
        "abandoned child 2",
        "Abandoned Child 2 Forgotten in Manhattan (1993) [1080p]/Abandoned.Child.2.Forgotten.in.Manhattan.1993.1080p.BluRay.x264.YIFY.mp4",
    );
}

#[test]
fn test_deadly_buddy() {
    assert_match("deadly-buddy-1", "Deadly Buddy (1988) [1080p]/Deadly.Buddy.1988.1080p.BrRip.x264.BOKUTOX.YIFY.mp4");
    assert_match("deadly-buddy-2", "Deadly Buddy 2 (1990) [1080p]/Deadly.Buddy.2.1990.1080p.BrRip.x264.BOKUTOX.YIFY.mp4");
    assert_match("deadly-buddy-3", "Deadly Buddy 3 (1993) [1080p]/Deadly.Buddy.3.1993.1080p.BrRip.x264.BOKUTOX.YIFY.mp4");
    assert_match("deadly-buddy-4", "Deadly Buddy 4 (1999) [1080p]/Deadly.Buddy.4.1999.1080p.BrRip.x264.BOKUTOX.YIFY.mp4");
}

#[test]
fn test_exploding_skyscraper() {
    assert_match("exploding-skyscraper-1", "Exploding Skyscraper (1989) [1080p] {5.1}/Exploding.Skyscraper.BluRay.1080p.x264.5.1.Judas.mp4");
    assert_match("exploding-skyscraper-2", "Exploding Skyscraper 2 (1991) [1080p] {5.1}/Exploding.Skyscraper.2.BluRay.1080p.x264.5.1.Judas.mp4");
}

#[test]
fn test_bobby_wizard() {
    assert_match(
        "bobby-wizard-escaped",
        "Bobby Wizard and the Escaped Criminal (2005) 1080p.BRrip.scOrp.sujaidr (pimprg)/Bobby Wizard and the Escaped Criminal (2005) 1080p.BRrip.scOrp.sujaidr (pimprg).mkv",
    );
    assert_match(
        "bobby-wizard-spooky-1",
        "Bobby Wizard and the Spooky Artifacts Part 1 (2011) [1080p]/Bobby.Wizard.and.the.Spooky.Artifacts.Part.1.2011.1080p.BrRip.x264.YIFY.mp4",
    );
    assert_match(
        "bobby-wizard-spooky-2",
        "Bobby Wizard and the Spooky Artifacts Part 2 (2012) 1080p.BRrip.scOrp.sujaidr (pimprg)/Bobby Wizard and the Spooky Artifacts Part 2 (2012) 1080p.BRrip.scOrp.sujaidr (pimprg).mkv",
    );
}

#[test]
fn test_time_traveler() {
    assert_match("time-traveler-s01e01", "Time.Traveler.2006.S01.COMPLETE.720p.BluRay.x264-GalaxyTV[TGx]/Time.Traveler.2006.S01E01.720p.BluRay.x264-GalaxyTV.mkv");
    assert_match("time-traveler-s01e03", "Time.Traveler.2006.S01.COMPLETE.720p.BluRay.x264-GalaxyTV[TGx]/Time.Traveler.2006.S01E03.720p.BluRay.x264-GalaxyTV.mkv");
}

#[test]
fn test_artifact_hunter() {
    assert_match(
        "artifact-hunter-final",
        "Artifact Hunter and the Final Quest (1990) [1080p]/Artifact.Hunter.And.The.Final.Quest.1990.1080p.BluRay.x264.YIFY.mp4",
    );
    assert_match("artifact-hunter-alien", "Artifact Hunter and the Alien Skull (2009) [1080p]/Artifact.Hunter.And.The.Alien.Skull.2009.1080p.BrRip.x264.YIFY.mp4");
    assert_match(
        "artifact-hunter-wheel",
        "Artifact Hunter And The Wheel Of Fate (2024) [1080p] [WEBRip] [5.1] [YTS.MX]/Artifact.Hunter.And.The.Wheel.Of.Fate.2024.1080p.WEBRip.x264.AAC5.1-[YTS.MX].mp4",
    );
}

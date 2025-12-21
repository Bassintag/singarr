use std::time::Duration;

#[derive(Debug, Clone)]
pub struct LrcIdTag {
    pub tag: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct LrcLyricLine {
    pub time: Option<Duration>,
    pub lyric: String,
}

#[derive(Debug, Clone)]
pub struct Lrc {
    pub tags: Vec<LrcIdTag>,
    pub lyrics: Vec<LrcLyricLine>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LyricsType {
    Synced,
    Unsynced,
    Mixed,
}

impl Lrc {
    pub fn lyrics_type(&self) -> LyricsType {
        let mut has_unsynced = false;
        let mut has_synced = false;
        for line in self.lyrics.iter() {
            if line.time.is_none() {
                if has_synced {
                    return LyricsType::Mixed;
                }
                has_unsynced = true;
            } else {
                if has_unsynced {
                    return LyricsType::Mixed;
                }
                has_synced = true;
            }
        }
        if has_unsynced {
            LyricsType::Unsynced
        } else {
            LyricsType::Synced
        }
    }
}

pub struct LrcParser {
    input: String,
    pos: usize,
}

impl LrcParser {
    pub fn new(input: String) -> Self {
        Self { input, pos: 0 }
    }

    fn peek(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn next(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos += c.len_utf8();
        Some(c)
    }

    fn expect(&mut self, f: impl FnOnce(char) -> bool) -> Option<char> {
        if f(self.peek()?) {
            self.next()
        } else {
            None
        }
    }

    fn expect_eq(&mut self, c: char) -> Option<char> {
        self.expect(|other| other == c)
    }

    fn expect_numeric(&mut self) -> Option<char> {
        self.expect(|c| c.is_numeric())
    }

    fn checkpoint(&self) -> usize {
        self.pos
    }

    fn restore(&mut self, pos: usize) {
        self.pos = pos
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.next();
            } else {
                break;
            }
        }
    }

    fn backtrack<T>(&mut self, f: impl FnOnce(&mut Self) -> Option<T>) -> Option<T> {
        let checkpoint = self.checkpoint();
        match f(self) {
            Some(v) => Some(v),
            None => {
                self.restore(checkpoint);
                None
            }
        }
    }

    pub fn parse(&mut self) -> Lrc {
        let mut lrc = Lrc {
            tags: Vec::new(),
            lyrics: Vec::new(),
        };
        while let Some(id_tag) = self.backtrack(Self::parse_id_tag) {
            lrc.tags.push(id_tag);
        }
        while let Some(lyric_line) = self.backtrack(Self::parse_lyric_line) {
            lrc.lyrics.push(lyric_line);
            if self.peek() == None {
                break;
            }
        }
        lrc
    }

    fn parse_id_tag(&mut self) -> Option<LrcIdTag> {
        self.skip_whitespace();
        self.expect_eq('[');
        let mut tag = String::new();
        loop {
            if !self.peek()?.is_alphabetic() {
                break;
            }
            tag.push(self.next()?);
        }
        self.expect_eq(':')?;
        let mut value = String::new();
        loop {
            if self.peek()? == ']' {
                self.next()?;
                break;
            }
            value.push(self.next()?);
        }
        Some(LrcIdTag {
            tag: tag.trim().into(),
            value: value.trim().into(),
        })
    }

    fn parse_lyric_line(&mut self) -> Option<LrcLyricLine> {
        self.skip_whitespace();
        let time = self.backtrack(Self::parse_time);
        let mut lyric = String::new();
        loop {
            match self.peek() {
                Some(c) => {
                    if c == '\n' {
                        break;
                    }
                }
                None => break,
            }
            lyric.push(self.next()?);
        }
        Some(LrcLyricLine {
            time,
            lyric: lyric.trim().into(),
        })
    }

    fn parse_time(&mut self) -> Option<Duration> {
        self.expect_eq('[')?;
        let mut secs = 0;
        let mut h = String::with_capacity(2);
        h.push(self.expect_numeric()?);
        h.push(self.expect_numeric()?);
        secs += h.parse::<u64>().ok()? * 3_600;
        self.expect_eq(':')?;
        let mut m = String::with_capacity(2);
        m.push(self.expect_numeric()?);
        m.push(self.expect_numeric()?);
        secs += m.parse::<u64>().ok()? * 60;
        self.expect_eq('.')?;
        let mut s = String::with_capacity(2);
        s.push(self.expect_numeric()?);
        s.push(self.expect_numeric()?);
        secs += s.parse::<u64>().ok()?;
        self.expect_eq(']')?;
        Some(Duration::new(secs, 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tags() {
        let mut parser = LrcParser::new(
            r#"
            [ar:Chubby Checker oppure  Beatles, The]
            [al:Hits Of The 60's - Vol. 2 – Oldies]
            [ti:Let's Twist Again]
            [au:Written by Kal Mann / Dave Appell, 1961]
            [length: 2:23]"#
                .into(),
        );
        let lrc = parser.parse();
        assert_eq!(lrc.tags.len(), 5);
        assert_eq!(lrc.tags[0].tag, String::from("ar"));
        assert_eq!(lrc.tags[1].tag, String::from("al"));
        assert_eq!(lrc.tags[2].tag, String::from("ti"));
        assert_eq!(lrc.tags[3].tag, String::from("au"));
        assert_eq!(lrc.tags[4].tag, String::from("length"));
    }

    #[test]
    fn test_lyrics_unsync() {
        let mut parser = LrcParser::new(r#"Hello world!"#.into());
        let lrc = parser.parse();
        assert_eq!(lrc.lyrics.len(), 1);
        assert_eq!(lrc.lyrics[0].time, None);
        assert_eq!(lrc.lyrics[0].lyric, String::from("Hello world!"));
    }

    #[test]
    fn test_lyrics_sync() {
        let mut parser = LrcParser::new(r#"[01:01:23] Hello world!"#.into());
        let lrc = parser.parse();
        assert_eq!(lrc.lyrics.len(), 1);
        assert_eq!(lrc.lyrics[0].time, Some(Duration::new(3683, 0)));
        assert_eq!(lrc.lyrics[0].lyric, String::from("Hello world!"));
    }

    #[test]
    fn test_lyrics_mixed() {
        let mut parser = LrcParser::new(
            r#"[00:00:01] Line 1
            Line 2
            Line 3
            [00:00:04] Line 4
            [00:00:05] Line 5"#
                .into(),
        );
        let lrc = parser.parse();
        assert_eq!(lrc.lyrics.len(), 5);
        assert_eq!(lrc.lyrics[0].time, Some(Duration::new(1, 0)));
        assert_eq!(lrc.lyrics[0].lyric, String::from("Line 1"));
        assert_eq!(lrc.lyrics[1].time, None);
        assert_eq!(lrc.lyrics[1].lyric, String::from("Line 2"));
        assert_eq!(lrc.lyrics[2].time, None);
        assert_eq!(lrc.lyrics[2].lyric, String::from("Line 3"));
        assert_eq!(lrc.lyrics[3].time, Some(Duration::new(4, 0)));
        assert_eq!(lrc.lyrics[3].lyric, String::from("Line 4"));
        assert_eq!(lrc.lyrics[4].time, Some(Duration::new(5, 0)));
        assert_eq!(lrc.lyrics[4].lyric, String::from("Line 5"));
    }

    #[test]
    fn test_real() {
        let mut parser = LrcParser::new(
            r#"[00:16.24] S.F.N, c'est des gorges tranchées dans des caves
[00:19.04] La nôtre ou celle des autres? Je sais pas
[00:20.59] 700 grammes dans l'cul, vas-y, fais la taupe
[00:22.36] Sista sort le ciseau, v'là qu'elle fait la coupe
[00:24.35] V'la qu'ils t'attendent, bébé, tu sais qu'j'suis moche, moi
[00:26.37] Jump sur ma sœur (Argh), j'vais t'faire du cinéma
[00:28.42] Son combat, c'est mon combat, espèce de cave, tu comprends pas
[00:30.41] Le premier qui veut tester, bâtard, je sors un gala gala
[00:32.31] Bo-bozo pour la guala, qu'est-ce qu'il n'a pas fait comme moi? (Han han)
[00:34.64] Sau-sauf que moi, j'avais pas l'choix (Han)
[00:36.46] Ces bozos pour la mala, qu'est-ce qu'ils ne font pas comme nous? (Tout)
[00:39.12] Ils puent la merde, c'est des fous
[00:40.32] Quelques traîtres au sein du camp (Han)
[00:41.74] On le sait, on fait semblant
[00:42.81] La cause est tellement grande, bâtard, j'suis prêt partir sans les gants
[00:44.87] À finir dans les faits divers, le travail, ça paye
[00:46.63] Bientôt, c'est nous en fourrure qui glissons aux sports d'hivers
[00:48.74] String qui dépasse, bientôt, c'est nous on fait la pose
[00:50.74] Si t'as un problème avec ça, j'mets ma teub dans ta throat
[00:52.88] J'vois que des swag chaser, j'vois que des mini-merdes
[00:54.95] Ça fait les foufous à la mort, ça s'croit dans Many Men
[00:56.99] Là, j'sors d'ma carapace, je viens t'péter l'hymen
[00:59.15] La lumière me brûle les yeux comme 100 000 lumens
[01:01.19] Va laver tes oreilles crasseuses pleines de cérumens
[01:03.18] Et prépare-toi à tanker les bounyas de S.F.N
[01:05.39] Y a d'l'espoir les mômes, y en aura toujours
[01:08.89] Personne nous a abattus, personne nous abattra
[01:10.61] La peur change de camp, bande de putes
[01:12.33] J'suis effrayé carrément, je sais, elle va envoyer un truc derrière, vous allez serrer
[01:15.08] On va continuer d'vous éduquer encore et encore, bande de merdes
[01:17.27] Sista c'est maman, papa, c'est FEMTO'
[01:18.66] J'prie chaque jour un peu plus pour la mort de Papacito
[01:20.72] Go on, babe
[01:21.35] Pour cette moula, ça n'a pas fait dans les métiers sages
[01:23.32] MNG, SPK, les nes-jeu collent sur le métissage
[01:25.38] Pour venger la p'tite d'avant, j'dois brasser l'APY d'un thug, gros
[01:27.52] J'cramais l'galet dans l'champ d'à côté, pas dans tes soirées tech, gros
[01:29.66] On n'est pas méchants, tu sais, on aime juste pas les gens
[01:31.59] On passait devant tous les jours, on cala pas les champs
[01:33.24] Nos deux teubs dans les oreilles du grand boug qui nous crie "allez, chante"
[01:35.30] Plus fort gamin, ça tend l'oreille seulement si l'offre est alléchante
[01:37.56] Sous cachet, l'reuf allait nous cracher dans l'entrée du virage
[01:39.67] Quatre heure pétante, j'sortais d'la tente pour faire les voitures du village
[01:41.51] 20k d'abo', 12k de cons, j'en fais partie, j'suis dans la conv'
[01:43.79] "Sœur, toi, t'es où?", j'suis dans la cave
[01:44.77] "Envoie des sous", j'suis pas la CAF
[01:45.99] C'est la vraie vie, jamais j'vends du rêve de Dubaï
[01:47.51] J'pull up avec le Fancy drip au père de Booba
[01:49.71] On s'en bat les c' de la structure
[01:50.92] Tu vas manger nos longs versets
[01:52.19] On peut pas v'nir à ta fête, j'ai peur d'en sortir le gland gercé
[01:54.07] Ah, là-celle va falloir bien la réviser les jeunes
[01:56.03] Attendez, il manque une leçon à ce chapitre, on reprend
[01:57.94] Dieu merci, j'fais l'argent sans le texte
[01:59.85] Lance le dé, j'mets toutes les stats en dex
[02:01.31] On a pop, toi, qu'est-ce que t'attendais?
[02:02.92] Écoute-moi, faire comme nous, p'tit, vaux mieux pas tenter
[02:05.04] Tout ça avec deux cros-mi à 100 balles donc
[02:06.81] Remballe ta propal', on la sent pas
[02:08.64] Dans les bureaux d'Virgin pour sept chiffres ou l'attentat
[02:10.63] On construit bien pire qu'Babylone qu'de la Pampa
[02:12.37] C'est papa, maman, bâtard, c'est pas tes grands-pa'
[02:14.11] On compte pas finir pendu près des remparts
[02:15.83] J'te voyais sortir le 12 et faire "tou-tou-tou" en bas
[02:17.62] C'est pas qu'dans ma tête, B, le teu-teuh me rend barge (Et va baiser ta mère la-)
[02:21.13] On s'est tués pour ça, tu fais l'difficile
[02:22.82] Si tu fais la folle, on sort le doux missile
[02:24.54] J'imprime d'ici, jamais je sors du domicile
[02:26.48] "#
                .into(),
        );
        let lrc = parser.parse();
        assert_eq!(lrc.tags.len(), 0);
        assert_eq!(lrc.lyrics.len(), 69);
        assert_eq!(lrc.lyrics_type(), LyricsType::Synced);
    }
}

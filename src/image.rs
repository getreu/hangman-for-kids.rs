//! Holds a dictionary of built-in ASCII art images and manages the piecemeal disclosure to the
//! image.  Also parses user provided images if given in the configuration file.

extern crate crossterm;
extern crate rand;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::{Ord, Ordering};
use std::fmt;

/// Identifier tagging image data in configuration files.
pub const CONF_LINE_IDENTIFIER__IMAGE: char = '|';

/// Threshold to decide from how many characters on the images is considered to be "big".
/// Big images are disclosed with another algorithm.
const BIG_IMAGE: usize = 100; // sort algorithm <-> random algorithm

/// A collection of built-in images from whom one is chosen at the start of the game.
// first char of image lines must be '|'
const DEFAULT_IMAGES: &[&str] = &[
    r#"
|    ,,,,,
|   (o   o)
|    /. .\
|   (_____)
|     : :
|    ##O##'
|  ,,,: :,,,
| _)\ : : /(____
|{  \     /  ___}
| \/)     ((/
|  (_______)
|    :   :
|    :   :
|   / \ / \
|   """ """
"#,
    r#"
|    |\_|X|_/|
|   /         \
| =(  O     O  )=
|  -\    o    /-
|   / .-----. \
| /_ | o   o |_ \
|(U  |       |  U)
|   _|_     _|_
|  (   )---(   )
"#,
    r#"
|        _.---._    /\\
|     ./'       "--`\//
|   ./              o \
|  /./\  )______   \__ \
| ./  / /\ \   | \ \  \ \
|    / /  \ \  | |\ \  \7
|     "     "    "  "        VK
"#,
    r#"
|       ,.
|      (_|,.
|     ,' /, )_______   _
|  __j o``-'        `.'-)'
| (")                 \'
|  `-j                |
|    `-._(           /
|       |_\  |--^.  /
|      /_]'|_| /_)_/
|         /_]'  /_]'
# Author: hjw
"#,
    r#"
|        _
|       [ ]
|      (   )
|       |>|
|    __/===\__
|   //| o=o |\\
| <]  | o=o |  [>
|     \=====/
|    / / | \ \
|   <_________>
"#,
    r#"
|                          (_)(_)
|                          /     \
|                         /       |
|                        /   \  * |
|          ________     /    /\__/
|  _      /        \   /    /
| / \    /  ____    \_/    /
|//\ \  /  /    \         /
|V  \ \/  /      \       /
|    \___/        \_____/
"#,
    r#"
|         .-.
|        (. .)__,')
|        / V      )
|  ()    \  (   \/
|<)-`\()  `._`._ \
|  <).>=====<<==`'====
|   C-'`(>
# Author: hjw
"#,
    r#"
| >(. )
|  |  (     /)
|  |   \___/ )
|  (   ----- )  >@)_//   >@)_//  >@)_//  >@)_//
|   \_______/    (__)     (__)    (__)    (__)
|~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~
"#,
    r#"
|           __
|           /(`o
|     ,-,  //  \\
|    (,,,) ||   V
|   (,,,,)\//
|   (,,,/w)-'
|   \,,/w)
|   `V/uu
|     / |
|     | |
|     o o
|     \ |
|\,/  ,\|,.  \,/
"#,
    r#"
|o
| \_/\o
|( Oo)                    \|/
|(_=-)  .===O-  ~~Z~A~P~~ -O-
|/   \_/U'                /|\
|||  |_/
|\\  |
|{K ||
| | PP
| | ||
| (__\\
# Author: ac
"#,
    r#"
|      ______
|     /     /\
|    /     /  \
|   /_____/----\_    (
|  "     "          ).
| _ ___          o (:') o
|(@))_))        o ~/~~\~ o
|                o  o  o
"#,
    r#"
|                             _______     |\
|                            |License|    | \
|  _____                     | ~~*~~ |    |  \
| |     |  (((        .--.   |_______|    |
| |DrJRO| ~OvO~ __   (////)               |
| |     | ( _ )|==|   \__/                |
| |o    |  \_/ |_(|  /    \   _______     |
| |     | //|\\   \\//|  |\\  |__o__|     |
| |   __|//\_/\\ __\/ |__|//  |__o__|     |
| |  |==""//=\\""====|||||)   |__o__|     |
|_|__||_|_||_||_____||||||____|__o__|_____|
|    ||  (_) (_)    ||||||                \
|    []             [(_)(_)
"#,
    r#"
|   _     _
|  ( |_ _| )
|   ( .". )
|  _( (Y) )_
| / /,---.\ \
|/ / | + | \ \
|\_)-"   "-(_/
|  |_______|
|  _)  |  (_
| (___,'.___)  hjw
# Art by Hayley Jane Wakenshaw
# (slightly modified)
"#,
    r#"
|          |
|        \ _ /
|      -= (_) =-
|        /   \         _\/_
|          |           //o\  _\/_
|   _____ _ __ __ ____ _ | __/o\\ _
| =-=-_-__=_-= _=_=-=_,-'|"'""-|-,_
|  =- _=-=- -_=-=_,-"          |
|jgs =- =- -=.--"
# Art by Genoveva Galarza
"#,
    r#"
|        __I__
|   .-'"  .  "'-.
| .'  / . ' . \  '.
|/_.-..-..-..-..-._\ .---------------------------------.
|         #  _,,_   ( I hear it might rain people today )
|         #/`    `\ /'---------------------------------'
|         / / 6 6\ \
|         \/\  Y /\/       /\-/\
|         #/ `'U` \       /a a  \               _
|       , (  \   | \     =\ Y  =/-~~~~~~-,_____/ )
|       |\|\_/#  \_/       '^--'          ______/
|       \/'.  \  /'\         \           /
|        \    /=\  /         ||  |---'\  \
|   jgs  /____)/____)       (_(__|   ((__|
# Art by Joan Stark
"#,
    r#"
| [][][] /""\ [][][]
|  |::| /____\ |::|
|  |[]|_|::::|_|[]|
|  |::::::__::::::|
|  |:::::/||\:::::|
|  |:#:::||||::#::|
| #%*###&*##&*&#*&##
|##%%*####*%%%###*%*#
"#,
    r#"
|  ,-~~-.___.
| / |  '     \
|(  )         0
| \_/-, ,----'
|    ====           //
|   /  \-'~;    /~~~(O)
|  /  __/~|   /       |
|=(  _____| (_________|
"#,
    r#"
|  \,`/ /
| _)..  `_
|( __  -\
|    '`.
|   ( \>_-_,
|   _||_ ~-/    W<
"#,
    r#"
|            __:.__
|           (_:..'"=
|            ::/ o o\         AHAH!
|           ;'-'   (_)     Spaceman Spiff      .
|           '-._  ;-'        wins again !  _'._|\/:
|           .:;  ;                .         '- '   /_
|          :.. ; ;,                \       _/,    "_<
|         :.|..| ;:                 \__   '._____  _)
|         :.|.'| ||                            _/ /
|snd      :.|..| :'                           `;--:
|         '.|..|:':       _               _ _ :|_\:
|      .. _:|__| '.\.''..' ) ___________ ( )_):|_|:
|:....::''::/  | : :|''| "/ /_=_=_=_=_=/ :_[__'_\3_)
| ''''      '-''-'-'.__)-'
# Art by Shanaka Dias
"#,
    r#"
|  _,                          _
|.'  `.                  ___.>"''-..-.
|`-.   ;           .--"""        .-._@;
|   ;  !_.--..._ .'      /     .[_@'`'.
|  ;            /       : .'  ; :_.._  `.
|  :           ;        ;[   _T-"  `.'-. `-.
|   \        .-:      ; `.`-=_,88p.   _.}.-"
|    `-.__.-'   \    /L._ Y",P$T888;  ""
|             .-'_.-'  / ;$$$$$$]8P;
|             \ /     / / "Y$$P" ^"
|     fsc      ;\_    `.\_._
|              ]__\     \___;
"#,
    r#"
|        _
|      _<_/_
|   __/    _>
|  '\  '  |
|    \___/
|    /+++\
| o=|..|..|
|   | o/..|
|0==|+++++|
| 0======/
"#,
    r#"
|        _../|_
|      ='__   _~-.
|           \'  ~-`\._
|                 |/~`
|   .    .    .    .    .
|_.`(._.`(._.`(._.`(._.`(._
"#,
    r#"
|                        ____
|                   .---'-    \
|      .-----------/           \
|     /           (         ^  |   __
|&   (             \        O  /  / .'
|'._/(              '-'  (.   (_.' /
|     \                    \     ./
|      |    |       |    |/ '._.'
|       )   @).____\|  @ |
|   .  /    /       (    | mrf
|  \|, '_:::\  . ..  '_:::\ ..\).
# Art by Morfina
"#,
    r#"
|           __n__n__
|    .------`-\00/-'
|   /  ##  ## (oo)
|  / \## __   ./
|     |//YY \|/
|snd  |||   |||
# Art by Shanaka Dias
"#,
    r#"
|                       .-'~~~-.
|                     .'o  oOOOo`.
|                    :~~~-.oOo   o`.
|                     `. \ ~-.  oOOo.
|                       `.; / ~.  OO:
|                       .'  ;-- `.o.'
|                      ,'  ; ~~--'~
|                      ;  ;
|_______\|/__________\\;_\\//___\|/________
"#,
    r#"
|    ____
|   (__  '.
|    /_____)
|   ()@ @ )))
|    'C ,()(()
|    ,.'_'.' \
| __/ )   (--'
|'._./     \
|   (_._._._)
|    _|| ||_
|mrf(__.).__)
"#,
    r#"
|        o    .   _     .
|          .     (_)         o
|   o      ____            _       o
|  _   ,-/   /)))  .   o  (_)   .
| (_)  \_\  ( e(     O             _
| o       \/' _/   ,_ ,  o   o    (_)
|  . O    _/ (_   / _/      .  ,        o
|     o8o/    \\_/ / ,-.  ,oO8/( -TT
|    o8o8O | } }  / /   \Oo8OOo8Oo||     O
|   Oo(""o8"""""""""""""""8oo""""""")
|  _   `\`'                  `'   /'   o
| (_)    \                       /    _   .
|      O  \           _         /    (_)
|o   .     `-. .----<(o)_--. .-'
|   --------(_/------(_<_/--\_)--------hjw
"#,
    r#"
|                \||/
|                |  @___oo
|      /\  /\   / (__,,,,|
|     ) /^\) ^\/ _)
|     )   /^\/   _)
|     )   _ /  / _)
| /\  )/\/ ||  | )_)
|<  >      |(,,) )__)
| ||      /    \)___)\
| | \____(      )___) )___
|  \______(_______;;; __;;;
"#,
    r#"
|   (\{\
|   { { \ ,~,
|  { {   \)))  *
|   { {  (((  /
|    {/{/; ,\/
|       (( '
|        \` \
|        (/  \
|ejm     `)  `\
"#,
    r#"
|                    /
|               ,.. /
|             ,'   ';
|  ,,.__    _,' /';  .
| :','  ~~~~    '. '~
|:' (   )         )::,
|'. '. .=----=..-~  .;'
| '  ;'  ::   ':.  '"
|   (:   ':    ;)
|    \\   '"  ./
|     '"      '"
# DR J
"#,
    r#"
|     __/\__
|. _  \\''//
|-( )-/_||_\
| .'. \_()_/
|  |   | . \
|  |mrf| .  \
| .'. ,\_____'.
"#,
    r#"
|         _.-.
|       ,'/ //\
|      /// // /)
|     /// // //|
|    /// // ///
|   /// // ///
|  (`: // ///
|   `;`: ///
|   / /:`:/
|  / /  `'
| / /
|(_/  hh
"#,
    r#"
| _____
||A .  | _____
|| /.\ ||A ^  | _____
||(_._)|| / \ ||A _  | _____
||  |  || \ / || ( ) ||A_ _ |
||____V||  .  ||(_'_)||( v )|
|       |____V||  |  || \ / |
|              |____V||  .  |
|                     |____V| ejm98
"#,
    r#"
|      !!!!\\\\
|    '`!_  ||||
|     ` \`-'''|
|       `\   /
|        )\  \
| ejm   /  \  \
|           \|
"#,
    r#"
|  ,~~--~~-.
| +      | |\
| || |~ |`,/-\
| *\_) \_) `-'#,
"#,
    r#"
|  (.  \
|   \  |
|    \ |___(\--/)
|  __/    (  . . )
| "'._.    '-.O.'
|      '-.  \ "|\
|         '.,,/'.,,mrf
"#,
    r#"
|             __
|   ,'```--'''  ``-''-.
| ,'            ,-- ,-'.
|(//            `"'| 'a \
|  |    `;         |--._/
|  \    _;-._,    /
|   \__/\\   \__,'
|    ||  `'   \|\\
|    \\        \\`'
|hjw  `'        `'
"#,
    r#"
|\\             //
| \\\' ,      / //
|  \\\//,   _/ //,
|   \_-//' /  //<,
|     \ ///  <//`
|    /  >>  \\\`__/_
|   /,)-^>> _\` \\\
|   (/   \\ //\\
|       // _//\\\\
|      ((` ((
"#,
    r#"
|>o)
|(_>   <o)
|      (_>
"#,
    r#"
|              I~
|          I~ /V\  I~
|      I~ /V\ | | /V\  I~
| @ @ /V\ | |_|_|_| | /V\ @ @
|@@@@@| |_| |_/V\_| |_| |@@@@@
|@@@@@| | |_|_|_|_|_| | |@@@@@
|@@@@@|_|_V_V|   |V_V_|_|@@@@@
|_._._._._._._._._._._._._._._
|:::::::::::::|X|:::::::::::::
|Sher^
"#,
    r#"
| W                   __
|[ ]                 |::|
| E          ._.     |::|   ._.
| |\         |:| ._. |::|   |/|
| \ \\|/     |:|_|/| |::|_  |/|
|  |-( )-    |:|"|/|_|::|\|_|/| _
|  | V L     |:|"|/|||::|\|||/||:|
|  \    `  ___   ~~~~~~~~~~~~~~~~~~~~
|   |    \/  /     ~~~~ ~~~~ ~~~ ~~~
"#,
    r#"
|      .___.
|     /     \
|    | O _ O |
|    /  \_/  \
|  .' /     \ `.
| / _|       |_ \
|(_/ |       | \_)
|    \       /
|   __\_>-<_/__
|   ~;/     \;~
"#,
];

/// One character of the ASCII art image.
#[derive(PartialOrd, Eq, PartialEq, Debug, Copy, Clone)] //omitting Ord
pub struct ImChar {
    pub point: (u8, u8),
    pub code: char,
}

/// Format an image character.
impl fmt::Display for ImChar {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.code)
    }
}

/// Ord enables us to v.sort() the image characters.
impl Ord for ImChar {
    /// Compares to ImChar.
    /// Points near the left lower corner are small.
    fn cmp(&self, other: &Self) -> Ordering {
        fn weight(ic: &ImChar) -> isize {
            let &ImChar { point: (x, y), .. } = ic;
            // points near the lower left corner are light
            x as isize - y as isize
        }
        weight(&self).cmp(&weight(&other))
    }
}

#[derive(Clone, Debug, PartialEq)]
/// An ASCII-art image.
pub struct Image {
    pub ichars: Vec<ImChar>,
    pub offset: (usize, usize),
    pub dimension: (u8, u8),
    pub visible_points: usize,
}

/// Format an image.
impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let x_max = self.dimension.0 as usize;
        let y_max = self.dimension.1 as usize;

        let mut i = vec![' '; ((x_max + 1) * y_max) as usize];
        for y in 0..y_max {
            i[((x_max + 1) * y + x_max) as usize] = '\n';
        }

        for ic in self.ichars.iter().take(self.visible_points) {
            let &ImChar {
                point: (x, y),
                code,
            } = ic;
            i[(x as usize + y as usize * (x_max + 1))] = code;
        }

        write!(f, "{}", i.into_iter().collect::<String>())
    }
}

impl Image {
    /// Constructor reading image data from configuration files.
    pub fn new(string: &str, offset: (usize, usize)) -> Self {
        let mut v: Vec<ImChar> = Vec::new();

        for (y, line) in string
            // split in lines
            .lines()
            // consider only lines starting with '|'
            .filter(|&l| l.starts_with(CONF_LINE_IDENTIFIER__IMAGE))
            .enumerate()
        //.inspect(|&(n,l)| println!("line {:?}: {:?} ", n,l))
        {
            let mut ii: Vec<_> = line
                .char_indices()
                // skip first char '|'
                .skip(1)
                // consider only chars != ' '
                .filter(|&(_, c)| c != ' ')
                // save in ImChar object
                .map(|(x, c)| ImChar {
                    // subtract the char we have skipped before
                    point: ((x - 1) as u8, y as u8),
                    code: c,
                })
                .collect();
            v.append(&mut ii);
        }

        // find dimensions
        let dimension = if !v.is_empty() {
            let mut x_max = 0;
            let mut y_max = 0;

            for i in &v {
                let &ImChar { point: (x, y), .. } = i;
                if x > x_max {
                    x_max = x
                };
                if y > y_max {
                    y_max = y
                };
            }
            // we know there is at least one char
            (x_max + 1, y_max + 1)
        } else {
            (0, 0)
        };

        // order points
        let v_len = v.len();
        if v_len <= BIG_IMAGE {
            v.sort(); // Sort algorithm, see "impl Ord for ImageChar"
        } else {
            let mut rng = thread_rng();
            (&mut v).shuffle(&mut rng); // points appear randomly.
        }

        if v.is_empty() {
            let mut rng = thread_rng();
            // this is recursive!
            Self::new((&DEFAULT_IMAGES).choose(&mut rng).unwrap(), offset)
        } else {
            Self {
                ichars: v,
                offset,
                dimension,
                visible_points: v_len,
            }
        }
    }

    /// Sets how much of the image will be disclosed next time the image is rendered.
    pub fn hide(&mut self, fraction: (usize, usize)) {
        let l = self.ichars.len();

        let as_points = |(n, d)| (5 * l * (d - n) as usize / d as usize + l) / 6;

        // silently ignore division by zero
        if fraction.1 > 0 {
            self.visible_points = as_points(fraction);
        };
    }
}

// *******************************

#[cfg(test)]
mod tests {
    use super::*;

    /// Test image parsing of configuration file data
    #[test]
    fn test_image_parser_syntax() {
        let config: &str = r#"
|ab
|cd"#;
        let image = Image::new(&config, (10, 20));
        //println!("{:?}",image);
        let expected = Image {
            ichars: [
                ImChar {
                    point: (0, 0),
                    code: 'a',
                },
                ImChar {
                    point: (0, 1),
                    code: 'c',
                },
                ImChar {
                    point: (1, 0),
                    code: 'b',
                },
                ImChar {
                    point: (1, 1),
                    code: 'd',
                },
            ]
            .to_vec(),
            offset: (10, 20),
            dimension: (2, 2),
            visible_points: 4,
        };

        assert!(image == expected);
    }

    /// Is non image data ignored?
    #[test]
    fn test_image_parser_syntax_ignore() {
        let config: &str = r#"
|/\
\/"#;
        let image = Image::new(&config, (10, 20));
        //println!("{:?}",image);
        let expected = Image {
            ichars: [
                ImChar {
                    point: (0, 0),
                    code: '/',
                },
                ImChar {
                    point: (1, 0),
                    code: '\\',
                },
            ]
            .to_vec(),
            offset: (10, 20),
            dimension: (2, 1),
            visible_points: 2,
        };

        assert!(image == expected);
    }

    #[test]
    fn test_image_renderer() {
        let config: &str = r#"
|>o)
|(_>   <o)
|      (_>
"#;
        let expected: &str = ">o)      \n(_>   <o)\n      (_>\n";
        let image = Image::new(&config, (10, 20));

        assert!(image.visible_points > 0);
        assert_eq!(format!("{}", image), expected);
    }

    #[test]
    fn test_image_parser_built_in_image() {
        let config: &str = "this is no image";
        let image = Image::new(&config, (10, 20));
        //println!("{:?}",image);

        assert!(image.visible_points > 0);
    }

    /// disclose image progressively
    #[test]
    fn test_image_parser_disclose() {
        let config: &str = "|abcde";
        let mut image = Image::new(&config, (10, 20));
        //println!("{:?}",image);
        let expected = Image {
            ichars: [
                ImChar {
                    point: (0, 0),
                    code: 'a',
                },
                ImChar {
                    point: (1, 0),
                    code: 'b',
                },
                ImChar {
                    point: (2, 0),
                    code: 'c',
                },
                ImChar {
                    point: (3, 0),
                    code: 'd',
                },
                ImChar {
                    point: (4, 0),
                    code: 'e',
                },
            ]
            .to_vec(),
            offset: (10, 20),
            dimension: (5, 1),
            visible_points: 5,
        };
        assert!(image == expected);

        image.hide((5, 5));
        assert!(image.visible_points == 0);

        image.hide((1, 5));
        assert!(image.visible_points == 4);

        image.hide((0, 5));
        assert!(image.visible_points == 5);
    }
}

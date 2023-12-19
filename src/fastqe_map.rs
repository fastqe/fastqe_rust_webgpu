use std::collections::HashMap;

pub fn get_seq_emoji_map() -> HashMap<char, &'static str> {


let mut seq_emoji_map: HashMap<char, &str> = HashMap::new();
seq_emoji_map.insert('A', ":apple:");
seq_emoji_map.insert('C', ":corn:");
seq_emoji_map.insert('T', ":tomato:");
seq_emoji_map.insert('G', ":grapes:");
seq_emoji_map.insert('N', ":question:");

seq_emoji_map
}
//let all_qualities = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";


pub fn get_fastq_emoji_map() ->  HashMap<char, &'static str> {

let mut fastq_emoji_map: HashMap<char, &str> = HashMap::new();
fastq_emoji_map.insert('!', ":no_entry_sign:");
fastq_emoji_map.insert('\"', ":x:");
fastq_emoji_map.insert('#', ":japanese_goblin:");
fastq_emoji_map.insert('$', ":broken_heart:");
fastq_emoji_map.insert('%', ":no_good:");
fastq_emoji_map.insert('&', ":space_invader:");
fastq_emoji_map.insert('\'', ":imp:");
fastq_emoji_map.insert('(', ":skull:");
fastq_emoji_map.insert(')', ":ghost:");
fastq_emoji_map.insert('*', ":see_no_evil:");
fastq_emoji_map.insert('+', ":hear_no_evil:");
fastq_emoji_map.insert(',', ":speak_no_evil:");
fastq_emoji_map.insert('/', ":pouting_cat:");
fastq_emoji_map.insert('-', ":monkey_face:");
fastq_emoji_map.insert('.', ":crying_cat_face:");
fastq_emoji_map.insert('0', ":scream_cat:");
fastq_emoji_map.insert('1', ":bomb:");
fastq_emoji_map.insert('2', ":fire:");
fastq_emoji_map.insert('3', ":rage:");
fastq_emoji_map.insert('4', ":poop:");
fastq_emoji_map.insert('5', "🚨");
fastq_emoji_map.insert('6', ":grinning:");
fastq_emoji_map.insert('7', ":sweat_smile:");
fastq_emoji_map.insert('8', ":smirk:");
fastq_emoji_map.insert('9', ":blush:");
fastq_emoji_map.insert(':', ":kissing_smiling_eyes:");
fastq_emoji_map.insert(';', ":kissing:");
fastq_emoji_map.insert('<', ":kissing_closed_eyes:");
fastq_emoji_map.insert('>', ":kissing_heart:");
fastq_emoji_map.insert('@', ":smile:");
fastq_emoji_map.insert('=', ":smiley:");
fastq_emoji_map.insert('?', ":laughing:");
fastq_emoji_map.insert('A', ":yum:");
fastq_emoji_map.insert('B', ":relaxed:");
fastq_emoji_map.insert('D', ":stuck_out_tongue:");
fastq_emoji_map.insert('C', ":stuck_out_tongue_closed_eyes:");
fastq_emoji_map.insert('E', ":stuck_out_tongue_winking_eye:");
fastq_emoji_map.insert('G', ":grin:");
fastq_emoji_map.insert('H', ":smile:");
fastq_emoji_map.insert('I', ":sunglasses:");
fastq_emoji_map.insert('J', ":heart_eyes:");
fastq_emoji_map.insert('F', ":wink:");


fastq_emoji_map

}
//binning - i.e https://www.illumina.com/documents/products/technotes/technote_understanding_quality_scores.pdf


pub fn get_fastq_emoji_map_binned() ->  HashMap<char, &'static str> {



let mut fastq_emoji_map_binned: HashMap<char, &str> = HashMap::new();
//N (no call) N (no call)
fastq_emoji_map_binned.insert('!', ":no_entry_sign:");
fastq_emoji_map_binned.insert('\"', ":no_entry_sign:");

//2–9 6
fastq_emoji_map_binned.insert('#', ":skull:");
fastq_emoji_map_binned.insert('$', ":skull:");
fastq_emoji_map_binned.insert('%', ":skull:");
fastq_emoji_map_binned.insert('&', ":skull:");
fastq_emoji_map_binned.insert('\'', ":skull:");
fastq_emoji_map_binned.insert('(', ":skull:");
fastq_emoji_map_binned.insert(')', ":skull:");
fastq_emoji_map_binned.insert('*', ":skull:");

//10–19 15
fastq_emoji_map_binned.insert('+', ":poop:");
fastq_emoji_map_binned.insert(',', ":poop:");
fastq_emoji_map_binned.insert('-', ":poop:");
fastq_emoji_map_binned.insert('.', ":poop:");
fastq_emoji_map_binned.insert('/', ":poop:");
fastq_emoji_map_binned.insert('0', ":poop:");
fastq_emoji_map_binned.insert('1', ":poop:");
fastq_emoji_map_binned.insert('2', ":poop:");
fastq_emoji_map_binned.insert('3', ":poop:");
fastq_emoji_map_binned.insert('4', ":poop:");

//20–24 22
fastq_emoji_map_binned.insert('5', "🚨");
fastq_emoji_map_binned.insert('6', "🚨");
fastq_emoji_map_binned.insert('7', "🚨");
fastq_emoji_map_binned.insert('8', "🚨");
fastq_emoji_map_binned.insert('9', "🚨");

//25–29 27
fastq_emoji_map_binned.insert(':', ":smile:");
fastq_emoji_map_binned.insert(';', ":smile:");
fastq_emoji_map_binned.insert('<', ":smile:");
fastq_emoji_map_binned.insert('=', ":smile:");
fastq_emoji_map_binned.insert('>', ":smile:");

//30–34 33
fastq_emoji_map_binned.insert('?', ":laughing:");
fastq_emoji_map_binned.insert('@', ":laughing:");
fastq_emoji_map_binned.insert('A', ":laughing:");
fastq_emoji_map_binned.insert('B', ":laughing:");
fastq_emoji_map_binned.insert('C', ":laughing:");

//35–39 37
fastq_emoji_map_binned.insert('D', ":sunglasses:");
fastq_emoji_map_binned.insert('E', ":sunglasses:");
fastq_emoji_map_binned.insert('F', ":sunglasses:");
fastq_emoji_map_binned.insert('G', ":sunglasses:");
fastq_emoji_map_binned.insert('H', ":sunglasses:");

//≥ 40 40
fastq_emoji_map_binned.insert('I', ":heart_eyes:");
fastq_emoji_map_binned.insert('J', ":heart_eyes:");


fastq_emoji_map_binned
}

pub fn get_fastq_noemoji_map() ->  HashMap<char, char> {

let mut fastq_noemoji_map: HashMap<char, char> = HashMap::new();

//N (no call) N (no call)
fastq_noemoji_map.insert('!', '▁');
fastq_noemoji_map.insert('\"', '▁');

//2–9 6
fastq_noemoji_map.insert('#', '▂');
fastq_noemoji_map.insert('$', '▂');
fastq_noemoji_map.insert('%', '▂');
fastq_noemoji_map.insert('&', '▂');
fastq_noemoji_map.insert('\'', '▂');
fastq_noemoji_map.insert('(', '▂');
fastq_noemoji_map.insert(')', '▂');
fastq_noemoji_map.insert('*', '▂');

//10–19 15
fastq_noemoji_map.insert('+', '▃');
fastq_noemoji_map.insert(',', '▃');
fastq_noemoji_map.insert('-', '▃');
fastq_noemoji_map.insert('.', '▃');
fastq_noemoji_map.insert('/', '▃');
fastq_noemoji_map.insert('0', '▃');
fastq_noemoji_map.insert('1', '▃');
fastq_noemoji_map.insert('2', '▃');
fastq_noemoji_map.insert('3', '▃');
fastq_noemoji_map.insert('4', '▃');

//20–24 22
fastq_noemoji_map.insert('5', '▄');
fastq_noemoji_map.insert('6', '▄');
fastq_noemoji_map.insert('7', '▄');
fastq_noemoji_map.insert('8', '▄');
fastq_noemoji_map.insert('9', '▄');

//25–29 27
fastq_noemoji_map.insert(':', '▅');
fastq_noemoji_map.insert(';', '▅');
fastq_noemoji_map.insert('<', '▅');
fastq_noemoji_map.insert('=', '▅');
fastq_noemoji_map.insert('>', '▅');

//30–34 33
fastq_noemoji_map.insert('?', '▆');
fastq_noemoji_map.insert('@', '▆');
fastq_noemoji_map.insert('A', '▆');
fastq_noemoji_map.insert('B', '▆');
fastq_noemoji_map.insert('C', '▆');

//35–39 37
fastq_noemoji_map.insert('D', '▇');
fastq_noemoji_map.insert('E', '▇');
fastq_noemoji_map.insert('F', '▇');
fastq_noemoji_map.insert('G', '▇');
fastq_noemoji_map.insert('H', '▇');

//≥ 40 40
fastq_noemoji_map.insert('I', '█');
fastq_noemoji_map.insert('J', '█');

fastq_noemoji_map
}

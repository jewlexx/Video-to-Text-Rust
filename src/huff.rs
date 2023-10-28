//  a,  8a
//  `8, `8)                            ,adPPRg,
//   8)  ]8                        ,ad888888888b
//  ,8' ,8'                    ,gPPR888888888888
// ,8' ,8'                 ,ad8""   `Y888888888P
// 8)  8)              ,ad8""        (8888888""
// 8,  8,          ,ad8""            d888""
// `8, `8,     ,ad8""            ,ad8""
//  `8, `" ,ad8""            ,ad8""
//     ,gPPR8b           ,ad8""
//    dP:::::Yb      ,ad8""
//    8):::::(8  ,ad8""
//    Yb:;;;:d888""  Normand
//     "8ggg8P"      Veilleux

use crate::DATA;

pub struct HuffmanTree {
    data_pos: usize,
}

impl HuffmanTree {
    pub fn peek_char(&self) -> Option<u8> {
        // TODO: having <= maybe breaks stuff, validate this
        if self.data_pos <= DATA.len() {
            Some(DATA[self.data_pos])
        } else {
            None
        }
    }
}

impl Iterator for HuffmanTree {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.data_pos += 1;
        self.peek_char()
    }
}

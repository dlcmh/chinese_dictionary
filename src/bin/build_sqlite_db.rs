use std::collections::HashMap;

use bincode::deserialize_from;
use once_cell::sync::Lazy;

type Searchable = HashMap<String, Vec<u32>>;

static ENGLISH: Lazy<Searchable> =
    Lazy::new(|| deserialize_from(&include_bytes!("../../data/english.dictionary")[..]).unwrap());

fn main() {
    // println!("{:?}", ENGLISH["dlc"]); // panicks
    println!("{:?}", ENGLISH["car"]); // [33228, 60091, 100447]

    for (key, value) in ENGLISH.iter() {
        println!("{}: {:?}", key, value);
        // abbr%20for%20特別行政區首席執行官|特别行政区首席执行官: [68595]
        // temporary%20arrangement: [51528]
        // to%20agree%20with: [32758, 78933]
        // keep%20putting%20sth%20off: [37054, 37089]
        // hacksaw: [106494]
        // to%20go%20over%20the%20head%20of%20ones%20boss: [99507]
        // also%20transliterated%20尼桑: [50153]
        // cairns%20city%20in%20queensland%20australia: [12400]
        // be%20buried: [90551]
        // catalog%20of%20prostitutes: [67071]
        // longstanding: [77965]
        // serial%20number: [29979, 36506, 36511, 82498, 92139, 92153]
        // panchen%20erdeni%20or%20panchen%20lama: [69886]
        // stratum: [23938, 33924, 33929, 109699]
        // to%20make%20good: [43650, 43673]
        // plan%20and%20consider: [96858]
        // zhongmu%20county%20in%20zhengzhou%20鄭州|郑州zheng%20zhou%20henan: [3454]
        // speak%20in%20a%20low%20muffled%20voice: [70678]
        // surname%20ban: [69849]
        // clear%20the%20road: [63470]
        // to%20develop%20as%20a%20government%20project: [81658]
        // straw%20sandals: [82016, 89465, 90722]
        // tibetan%20sandgrouse%20syrrhaptes%20tibetanus: [94582]
        // to%20love%20to%20hear%20and%20see: [21639]
        // over%20ten%20thousand: [1672]
        // to%20obtain%20money: [42496]
        // sea%20urchin: [62431]
        // qr%20code: [4615]
        // overawe: [41613]
        // stomping%20ground: [84280]
    }
}

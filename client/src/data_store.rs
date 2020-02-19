#[derive(Debug)]
pub struct Data {
    pub id: String,
    pub name: String,
    pub volume: i32,
}

impl Data {
    pub fn new_data() -> Vec<Data> {
        let ids = ["aaa", "bbb", "abc", "ghb", "ccc"];
        let name = ["bill", "jack", "fred", "charlie", "tim"];
        let volume = [1, 41, 3, 6, 4, 12];
        let mut data_vec = vec![];
        for i in 0..4 {
            data_vec.push(Data {
                id: ids[i].to_string(),
                name: name[i].to_string(),
                volume: volume[i],
            })
        }
        data_vec
    }
}

use axum::Json;

use crate::meta::meta::{Ack, Bfs, Dag, Meta};

pub async fn meta() -> Json<Meta> {
    let mut meta = Meta::default();

    let mut dag = Dag::default();
    dag.predict = "predict_default".to_string();
    dag.ack = "ack_default".to_string();
    dag.query = "query_default".to_string();
    dag.rough_sort = "rough_sort".to_string();
    meta.dag = Some(dag);

    let mut ack = Ack::default();
    ack.databus = "ack_feature".to_string();
    ack.enable_abase = true;
    meta.ack = Some(ack);

    let mut bfs = Bfs::default();
    bfs.bfs_id = "demo_bfsid".to_string();
    bfs.enable_fe = true;
    meta.bfs = Some(bfs);

    Json(meta)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use prost::Message;

    use crate::meta::meta::Meta;

    #[test]
    fn test_from_json() {
        let str = "{\"dag\":{\"predict\":\"predict_default\",\"rough_sort\":\"rough_sort\",\"ack\"\
                   :\"ack_default\",\"query\":\"query_default\"},\"ack\":{\"databus\":\"\
                   ack_feature\",\"enable_abase\":true},\"bfs\":{\"bfs_id\":\"demo_bfsid\",\"\
                   enable_fe\":true}}";

        let meta: Meta = serde_json::from_str(str).unwrap();

        println!("meta {:?}", meta);
    }

    #[test]
    fn test_marshal() {
        let str = "{\"dag\":{\"predict\":\"predict_default\",\"rough_sort\":\"rough_sort\",\"ack\"\
                   :\"ack_default\",\"query\":\"query_default\"},\"ack\":{\"databus\":\"\
                   ack_feature\",\"enable_abase\":true},\"bfs\":{\"bfs_id\":\"demo_bfsid\",\"\
                   enable_fe\":true}}";

        let meta: Meta = serde_json::from_str(str).unwrap();

        let mut buf = Vec::new();
        buf.reserve(meta.encoded_len());
        meta.encode(&mut buf).unwrap();
        println!("meta {:?}", buf);
    }

    #[test]
    fn test_unmarshal() {
        let buf: Vec<u8> = vec![
            10, 57, 10, 15, 112, 114, 101, 100, 105, 99, 116, 95, 100, 101, 102, 97, 117, 108, 116,
            18, 10, 114, 111, 117, 103, 104, 95, 115, 111, 114, 116, 26, 11, 97, 99, 107, 95, 100,
            101, 102, 97, 117, 108, 116, 34, 13, 113, 117, 101, 114, 121, 95, 100, 101, 102, 97,
            117, 108, 116, 18, 15, 10, 11, 97, 99, 107, 95, 102, 101, 97, 116, 117, 114, 101, 16,
            1, 26, 14, 10, 10, 100, 101, 109, 111, 95, 98, 102, 115, 105, 100, 16, 1,
        ];
        let meta = Meta::decode(&mut Cursor::new(buf)).unwrap();
        println!("meta {:?}", meta);
    }
}

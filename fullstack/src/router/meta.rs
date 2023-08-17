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
}

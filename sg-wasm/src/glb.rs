//! Minimal binary-glTF (GLB) writer for a set of coloured, possibly-transparent meshes.
use crate::mesh::Mesh;
use serde_json::{json, Value};

pub struct Group {
    pub mesh: Mesh,
    pub color: [u8; 4], // rgba, a<255 -> alpha blend
    pub metallic: f64,
    pub roughness: f64,
}

fn align4(v: &mut Vec<u8>, pad: u8) {
    while v.len() % 4 != 0 {
        v.push(pad);
    }
}

pub fn write_glb(groups: &[Group]) -> Vec<u8> {
    let mut bin: Vec<u8> = Vec::new();
    let mut views: Vec<Value> = Vec::new();
    let mut accessors: Vec<Value> = Vec::new();
    let mut materials: Vec<Value> = Vec::new();
    let mut meshes: Vec<Value> = Vec::new();
    let mut nodes: Vec<Value> = Vec::new();
    let mut node_idx: Vec<Value> = Vec::new();

    for g in groups {
        if g.mesh.is_empty() {
            continue;
        }
        let m = &g.mesh;

        // POSITION
        let pos_off = bin.len();
        for &f in &m.pos {
            bin.extend_from_slice(&f.to_le_bytes());
        }
        let pos_view = views.len();
        views.push(json!({"buffer":0,"byteOffset":pos_off,"byteLength":m.pos.len()*4,"target":34962}));
        // bounds
        let (mut mn, mut mx) = ([f32::INFINITY; 3], [f32::NEG_INFINITY; 3]);
        for c in m.pos.chunks(3) {
            for k in 0..3 {
                mn[k] = mn[k].min(c[k]);
                mx[k] = mx[k].max(c[k]);
            }
        }
        let pos_acc = accessors.len();
        accessors.push(json!({"bufferView":pos_view,"componentType":5126,"count":m.pos.len()/3,
            "type":"VEC3","min":[mn[0],mn[1],mn[2]],"max":[mx[0],mx[1],mx[2]]}));

        // NORMAL
        let nrm_off = bin.len();
        for &f in &m.nrm {
            bin.extend_from_slice(&f.to_le_bytes());
        }
        let nrm_view = views.len();
        views.push(json!({"buffer":0,"byteOffset":nrm_off,"byteLength":m.nrm.len()*4,"target":34962}));
        let nrm_acc = accessors.len();
        accessors.push(json!({"bufferView":nrm_view,"componentType":5126,"count":m.nrm.len()/3,"type":"VEC3"}));

        // INDICES
        align4(&mut bin, 0);
        let idx_off = bin.len();
        for &i in &m.idx {
            bin.extend_from_slice(&i.to_le_bytes());
        }
        let idx_view = views.len();
        views.push(json!({"buffer":0,"byteOffset":idx_off,"byteLength":m.idx.len()*4,"target":34963}));
        let idx_acc = accessors.len();
        accessors.push(json!({"bufferView":idx_view,"componentType":5125,"count":m.idx.len(),"type":"SCALAR"}));
        align4(&mut bin, 0);

        // material
        let a = g.color[3] as f64 / 255.0;
        let mut mat = json!({
            "pbrMetallicRoughness": {
                "baseColorFactor": [g.color[0] as f64/255.0, g.color[1] as f64/255.0, g.color[2] as f64/255.0, a],
                "metallicFactor": g.metallic,
                "roughnessFactor": g.roughness
            },
            "doubleSided": true
        });
        if g.color[3] < 255 {
            mat["alphaMode"] = json!("BLEND");
        }
        let mat_idx = materials.len();
        materials.push(mat);

        // mesh + node
        let mesh_idx = meshes.len();
        meshes.push(json!({"primitives":[{
            "attributes":{"POSITION":pos_acc,"NORMAL":nrm_acc},
            "indices":idx_acc,"material":mat_idx
        }]}));
        node_idx.push(json!(nodes.len()));
        nodes.push(json!({"mesh":mesh_idx}));
    }

    let gltf = json!({
        "asset": {"version":"2.0","generator":"sg-wasm"},
        "scene": 0,
        "scenes": [{"nodes": node_idx}],
        "nodes": nodes,
        "meshes": meshes,
        "materials": materials,
        "accessors": accessors,
        "bufferViews": views,
        "buffers": [{"byteLength": bin.len()}]
    });

    // ---- assemble GLB container ----
    let mut json_chunk = serde_json::to_vec(&gltf).unwrap();
    align4(&mut json_chunk, b' ');
    align4(&mut bin, 0);

    let total = 12 + 8 + json_chunk.len() + 8 + bin.len();
    let mut out = Vec::with_capacity(total);
    out.extend_from_slice(&0x4654_6C67u32.to_le_bytes()); // "glTF"
    out.extend_from_slice(&2u32.to_le_bytes());
    out.extend_from_slice(&(total as u32).to_le_bytes());
    out.extend_from_slice(&(json_chunk.len() as u32).to_le_bytes());
    out.extend_from_slice(&0x4E4F_534Au32.to_le_bytes()); // "JSON"
    out.extend_from_slice(&json_chunk);
    out.extend_from_slice(&(bin.len() as u32).to_le_bytes());
    out.extend_from_slice(&0x004E_4942u32.to_le_bytes()); // "BIN\0"
    out.extend_from_slice(&bin);
    out
}

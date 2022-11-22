https://osmcode.org/osmium-tool/manual.html

## Convert .osm.pbf to .osm
```
osmium cat leicestershire-latest.osm.pbf -o leicestershire-latest.osm
```

## Cut out a geographic region
```
osmium extract -b -1.263095,52.769673,-1.224116,52.755050 leicestershire-latest.osm -o theoffice-latest.osm
```

```rust
let reader = ElementReader::from_path("theoffice-latest.osm.pbf").unwrap();
let mut nodes = 0_u64;
let mut dense_nodes = 0_u64;
let mut ways = 0_u64;
let mut relations = 0_u64;
let _ = reader.for_each(|element| {
    match element {
        Element::Node(node) => {
            if nodes == 0 {
                println!("Node");
                println!("{:?}", node);
                }
                nodes += 1;
                }
                Element::DenseNode(dense_node) => {
                    if dense_nodes == 0 {
                        println!("DenseNode");
                        println!("{:?}", dense_node);
                        }
                        dense_nodes += 1;
                        }
                        Element::Way(way) => {
                            if ways == 0 {
                                println!("Way");
                                println!("{:?}", way);
                                }
                                ways += 1;
                                }
                                Element::Relation(relation) => {
                                    if relations == 0 {
                                        println!("Relation");
                                        println!("{:?}", relation);
                                        }
                                        relations += 1;
                                        }
                                        }
                                        });

println!("Number of ways: {ways}");
```
fn main() {
    // let graph: Vec<Vec<u32>> = vec![
    //     vec![0, 1, 1, 0, 1],
    //     vec![1, 0, 1, 0, 0],
    //     vec![0, 1, 0, 0, 1],
    //     vec![1, 1, 1, 0, 1],
    //     vec![0, 1, 1, 0, 0],
    // ];

    let graph: Vec<Vec<u32>> = vec![
        vec![0, 1, 1, 0],
        vec![1, 0, 1, 1],
        vec![0, 0, 0, 0],
        vec![1, 0, 1, 0],
    ];

    for (vertex, edges) in graph.iter().enumerate() {
        let mut is_yucca = true;

        // check if the vertex has n-1 outdegree (has outgoing edges to all other vertexes)
        for (edge_number, edge_value) in edges.iter().enumerate() {
            if edge_number == vertex {
                continue;
            }

            // println!( vertex {} edge {} is {}", vertex, edge_number, edge_value);

            // we're skipping our own vertex, so if any edge isn't 1, this isn't a yucca
            if *edge_value != 1 {
                is_yucca = false;
                break;
            }
        }

        if !is_yucca {
            continue;
        }

        // Check if no other vertex has a edge to this vertex

        for other_edges in graph.iter() {
            let value = other_edges[vertex];

            if value == 1 {
                is_yucca = false;
                break;
            }
        }

        if !is_yucca {
            continue;
        }

        // got to the end, we found a yucca

        println!(
            "This graph is a YUCCA. Vertex {} has indegree 0 and outdegree {}-1.",
            vertex,
            graph.len()
        );
        return;
    }

    println!("This graph is not a YUCCA.");
}

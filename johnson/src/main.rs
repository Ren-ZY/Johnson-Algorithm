mod johnson_find_cycles;
use johnson_find_cycles::ElementaryCyclesSearch::*;
use johnson_find_cycles::StrongConnectedComponents::*;

fn main(){
    let mut adjMatrix = vec![vec![false; 10]; 10];
    adjMatrix[0][1] = true;
    adjMatrix[1][2] = true;
    adjMatrix[2][0] = true;
    adjMatrix[2][6] = true;
    adjMatrix[3][4] = true;
    adjMatrix[4][5] = true;
    adjMatrix[4][6] = true;
    adjMatrix[5][3] = true;
    adjMatrix[6][7] = true;
    adjMatrix[7][8] = true;
    adjMatrix[8][6] = true;
    adjMatrix[6][1] = true;
    let adjList = getAdjacencyList(&adjMatrix);
    let nodes = vec!["0".to_string(),
                                 "1".to_string(),
                                  "2".to_string(),
                                  "3".to_string(),
                                  "4".to_string(),
                                  "5".to_string(),
                                  "6".to_string(),
                                  "7".to_string(),
                                  "8".to_string()];
    let mut ecs = ElementaryCyclesSearch::New(&adjMatrix, &nodes);
    let cycles = ecs.getElementaryCycles();
    for i in 0..cycles.len(){
        let mut cycle = cycles[i].clone();
        for j in 0..cycle.len(){
            let node = cycle[j].clone();
            if j < cycle.len() - 1 {
                print!("{}->", node);
            }else{
                print!("{}", node);
            }
        }
        print!("\n");
    }

}
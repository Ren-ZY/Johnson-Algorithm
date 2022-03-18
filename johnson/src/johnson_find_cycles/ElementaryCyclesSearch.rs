use crate::johnson_find_cycles::StrongConnectedComponents::*;


pub fn getAdjacencyList(adjacencyMatrix: &Vec<Vec<bool>>) -> Vec<Vec<i32>>{
    let mut LR = vec![];
    for i in 0..adjacencyMatrix.len() {
        let mut vx = vec![];
        for j in 0..adjacencyMatrix[i].len(){
            if adjacencyMatrix[i][j] {
                vx.push(j as i32);
            }
        }
        LR.push(vx);
    }
    LR
}

pub struct ElementaryCyclesSearch{
    cycles: Vec<Vec<String>>,
    adjList: Vec<Vec<i32>>,
    graphNodes: Vec<String>,
    blocked: Vec<bool>,
    B: Vec<Vec<i32>>,
    stack: Vec<i32>
}

impl ElementaryCyclesSearch{
    pub fn New(matrix: &Vec<Vec<bool>>, gNodes: &Vec<String>) -> Self{
        Self{
            cycles: vec![],
            adjList: getAdjacencyList(matrix),
            graphNodes: gNodes.clone(),
            blocked: vec![],
            B: vec![],
            stack: vec![]
        }
    }

    pub fn findCycles(&mut self, v: usize, s: usize, adjList: &Vec<Vec<i32>>) -> bool{
        let mut f = false;
        self.stack.push(v as i32);
        self.blocked[v] = true;
        for i in 0..adjList[v].len() {
            let w = adjList[v][i];
            // found cycle
            if w == s as i32 {
                let mut cycle = Vec::new();
                for j in 0..self.stack.len() {
                    let index = self.stack[j];
                    cycle.push(self.graphNodes[index as usize].clone());
                }
                self.cycles.push(cycle);
                f = true;
            } else if self.blocked[w as usize] == false{
                if self.findCycles(w as usize, s, adjList) == true{
                    f = true;
                }
            }
        }
    
        if f {
            self.unblock(v);
        } else {
            for i in 0..adjList[v].len() {
                let w = adjList[v][i] as usize;
                if self.B[w].contains(&(v as i32)) == false{
                    self.B[w].push(v as i32);
                }
            }
        }
    
        self.stack.retain(|x| *x != v as i32);//remove the element v in stack
    
        return f;
    }

    pub fn getElementaryCycles(&mut self) -> Vec<Vec<String>> {
		
        self.blocked.clear();
        self.B.clear();
        if self.blocked.len() == 0 {
            self.blocked.resize(self.adjList.len(), false);
        }
        if self.B.len() == 0 {
            self.B.resize(self.adjList.len(), vec![]);
        }
        let mut sccs = StrongConnectedComponents::New(&self.adjList);
        let mut s = 0 as usize;

        loop {
            let sccResult = sccs.getAdjacencyList(s as i32);
            if sccResult.is_some() && sccResult.as_ref().unwrap().getAdjList().len() > 0 {
                let scc = sccResult.as_ref().unwrap().getAdjList();
                    s = sccResult.as_ref().unwrap().getLowestNodeId();
                    for j in 0..scc.len(){
                        if scc[j].len() > 0 {
                            self.blocked[j] = false;
                            self.B[j].clear();
                    }
                }
                self.findCycles(s, s, &scc);
                s += 1;
            } else{
                break;
            } 
        }

        self.cycles.clone()
	}

    pub fn unblock(&mut self, node: usize) {
        self.blocked[node] = false;
        //vector<int> Bnode = B[node];
        
        while self.B[node].len() > 0 {
            let w = self.B[node][0] as usize;
            self.B[node].remove(0);
            if self.blocked[w] {
                self.unblock(w);
            }
        }
    }

}
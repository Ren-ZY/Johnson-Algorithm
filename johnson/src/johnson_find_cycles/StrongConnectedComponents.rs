use crate::johnson_find_cycles::SCCResult::*;

pub struct StrongConnectedComponents{
    adjListOriginal: Vec<Vec<i32>>,
    adjList: Vec<Vec<i32>>,
    visited: Vec<bool>,
    stack: Vec<i32>,
    lowlink: Vec<i32>,
    number: Vec<i32>,
    sccCounter: i32,
    currentSCCs: Vec<Vec<i32>>
}

impl StrongConnectedComponents{
    pub fn New(adjList: &Vec<Vec<i32>>) -> Self{
        Self{
            adjListOriginal: adjList.clone(),
            adjList: vec![],
            visited: vec![],
            stack: vec![],
            lowlink: vec![],
            number: vec![],
            sccCounter: 0,
            currentSCCs: vec![]
        }
    }

    pub fn getAdjacencyList(&mut self, node: i32) -> Option<SCCResult>{
        self.visited.clear();
        self.lowlink.clear();
        self.number.clear();
        self.stack.clear();
        self.currentSCCs.clear();
        if self.visited.len() == 0 {
            self.visited.resize(self.adjListOriginal.len(), false);
        }
        if self.lowlink.len() == 0{
            self.lowlink.resize(self.adjListOriginal.len(), 0);
        }
        if self.number.len() == 0{
            self.number.resize(self.adjListOriginal.len(), 0);
        }
        
        self.makeAdjListSubgraph(node);

        for i in node as usize..self.adjListOriginal.len(){
            if !self.visited[i] {
                self.getStrongConnectedComponents(i);
                let nodes = self.getLowestIdComponent();
                if nodes.is_some() && !nodes.as_ref().unwrap().contains(&node) && !nodes.as_ref().unwrap().contains(&(node+1)) {
                    return self.getAdjacencyList(node+1);
                }else{
                    let adjacencyList = self.getAdjList(&nodes);
                    if let Some(adjacencyList) = adjacencyList{
                        for j in 0.. self.adjListOriginal.len(){
                            if adjacencyList[j].len() > 0{
                                return Some(SCCResult::New(&adjacencyList, j));
                            }
                        }
                    }
                }
            }
        }
        return None;
    }

    pub fn makeAdjListSubgraph(&mut self, node: i32){
        self.adjList.clear();
        if self.adjList.len() == 0{
            self.adjList.resize(self.adjListOriginal.len(), vec![]);
        }
        for i in node as usize..self.adjList.len(){
            let mut successors = vec![];
            for j in 0..self.adjListOriginal[i].len(){
                if self.adjListOriginal[i][j] >= node {
                    successors.push(self.adjListOriginal[i][j]);
                }
            }
            if successors.len() > 0 {
                self.adjList[i].resize(successors.len(), 0);
                for j in 0..successors.len(){
                    let succ = successors[j];
                    self.adjList[i][j] = succ;
                }
            }
        }
    }

    pub fn getLowestIdComponent(&mut self) -> Option<Vec<i32>>{
        let mut min = self.adjList.len() as i32;
        let mut currScc = None;
        for i in 0..self.currentSCCs.len() {
            let scc = &self.currentSCCs[i];
            for j in 0..scc.len() {
                let node = scc[j];
                if node < min as i32 {
                    currScc = Some(scc.clone());
                    min = node;
                }
            }
        }
        return currScc;
    }

    pub fn getAdjList(&mut self, nodes: &Option<Vec<i32>>) -> Option<Vec<Vec<i32>>> {
        let mut lowestIdAdjacencyList = None;
        if let Some(nodes) = nodes {
           lowestIdAdjacencyList = Some(vec![vec![]; self.adjList.len()]);
           for i in 0..nodes.len(){
               let node = nodes[i] as usize;
               for j in 0..self.adjList[node].len() {
                   let succ = self.adjList[node][j];
                   if nodes.contains(&succ){
                       if let Some(ref mut lowestIdAdjacencyList) = lowestIdAdjacencyList{
                           lowestIdAdjacencyList[node].push(succ);
                       }
                   } 
               }
           }    
        }
    return lowestIdAdjacencyList;
    }

    pub fn getStrongConnectedComponents(&mut self, root: usize) {
        self.sccCounter += 1;
        self.lowlink[root] = self.sccCounter;
        self.number[root] = self.sccCounter;
        self.visited[root] = true;
        self.stack.push(root as i32);
        for i in 0..self.adjList[root].len(){
            let w = self.adjList[root][i] as usize;
            if !self.visited[w]{
                self.getStrongConnectedComponents(w);
                self.lowlink[root] = if self.lowlink[root] < self.lowlink[w]{
                    self.lowlink[root]
                }else{
                    self.lowlink[w]
                }

            } else if self.number[w] < self.number[root] {
                if self.stack.contains(&(w as i32)) {
                    self.lowlink[root] = if self.lowlink[root] < self.number[w]{
                        self.lowlink[root]
                    }else{
                        self.number[w]
                    }
                }
            }
        }

        //found scc
        if self.lowlink[root] == self.number[root] && self.stack.len() > 0 {
            let mut next = -1 as i32;
            let mut scc = vec![];
            next = self.stack[self.stack.len()-1];
            self.stack.remove(self.stack.len()-1);
            scc.push(next);
    
            while self.number[next as usize] > self.number[root] {
                next = self.stack[self.stack.len()-1];
                self.stack.remove(self.stack.len()-1);
                scc.push(next);
            }
            
            if scc.len() > 1{
                self.currentSCCs.push(scc);
            }
        }
    }
}

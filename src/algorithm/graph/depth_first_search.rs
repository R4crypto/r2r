use std::collections::HashSet;
use std::collections::VecDeque;


pub fn depth_first_search(graph: &Graph, root: Vertex, objective: Vertex) -> Option<Vec<u32>> {
    let mut visited: HashSet<Vertex> = HashSet::new();
    let mut history: Vec<u32> = Vec::new();
    let mut queue: VecDeque<Vertex> = VecDeque::new();
    queue.push_back(root);

    //queue.pop_front()는 Option<Vertex>를 반환. Some(Vertex)로 패턴 매칭 사용
    //None이면 패턴 매칭 실패로 while 종료
    while let Some(current_vertex) = queue.pop_front() {
        history.push(current_vertex.value());

        if current_vertex == objective {
            return Some(history); //조기 return. 참고: 표현식으로 사용하려면 타입 일치 고려해야 함
        }

        //into_iter()소유권 이동 하는 iterator 사용
        for neighbor in current_vertex.neighbors(graph).into_iter().rev() {
            if visited.insert(neighbor) { //insert 성공 시 true 반환, 즉, visited에 없는 경우
                queue.push_front(neighbor);
            }
        }
    }



    None //return타입 Option {Some(), None} 참고
}


// Data Structures

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vertex(u32);
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Edge(u32, u32);
#[derive(Clone)]
pub struct Graph {
    #[allow(dead_code)]
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

impl Graph {
    pub fn new(vertices: Vec<Vertex>, edges: Vec<Edge>) -> Self {
        Graph { vertices, edges }
    }
}

// u32를 Vertex로 변환
impl From<u32> for Vertex {
    fn from(item: u32) -> Self {
        Vertex(item)
    }
}

// Vertex의 메서드들
impl Vertex {
    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn neighbors(&self, graph: &Graph) -> VecDeque<Vertex> {
        graph
            .edges
            .iter()
            .filter(|e| e.0 == self.0)
            .map(|e| e.1.into())
            .collect()
    }
}

// (u32, u32) 튜플을 Edge로 변환
impl From<(u32, u32)> for Edge {
    fn from(item: (u32, u32)) -> Self {
        Edge(item.0, item.1)
    }
}



//Test 모듈 선언
//테스트 환경에서만 코드를 컴파일하도록 지정
#[cfg(test)]
mod tests {
    use super::*;

    #[test] //함수를 테스트로 지정
    fn find_1_fail() {
        let vertices = vec![1, 2, 3, 4, 5, 6, 7];//macro
        let edges = vec![(1, 2), (1, 3), (2, 4), (2, 5), (3, 6), (3, 7)];

        let root = 1;
        let objective = 99;

        let graph = Graph::new(
            vertices.into_iter().map(|v| v.into()).collect(),
            edges.into_iter().map(|e| e.into()).collect(),
        );

        assert_eq!(
            depth_first_search(&graph, root.into(), objective.into()),
            None
        );
    }

    #[test]
    fn find_1_sucess() {
        let vertices = vec![1, 2, 3, 4, 5, 6, 7];
        let edges = vec![(1, 2), (1, 3), (2, 4), (2, 5), (3, 6), (3, 7)];

        let root = 1;
        let objective = 7;

        let correct_path = vec![1, 2, 4, 5, 3, 6, 7];

        let graph = Graph::new(
            vertices.into_iter().map(|v| v.into()).collect(),
            edges.into_iter().map(|e| e.into()).collect(),
        );

        assert_eq!(
            depth_first_search(&graph, root.into(), objective.into()),
            Some(correct_path)
        );
    }
}

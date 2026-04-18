use crate::model::{TokenTrait, PointToken, LineToken};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};

pub struct ObjExporter {
    points: Vec<PointToken>,
    lines: Vec<LineToken>,
    vertex_offset: i32,
}

impl ObjExporter {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            lines: Vec::new(),
            vertex_offset: 1,
        }
    }
    
    pub fn add_token(&mut self, token: Box<dyn TokenTrait>) {
        if let Some(point_token) = token.as_any().downcast_ref::<PointToken>() {
            self.points.push(point_token.clone());
        } else if let Some(line_token) = token.as_any().downcast_ref::<LineToken>() {
            self.lines.push(line_token.clone());
        }
    }
    
    pub fn export_to_file(&mut self, file_path: &str) -> io::Result<()> {
        let mut file = File::create(file_path)?;
        
        writeln!(file, "# OBJ file generated from VDSL")?;
        writeln!(file, "# Lighting: Top illumination")?;
        writeln!(file, "# Material: Light gray for filled polygons")?;
        writeln!(file)?;
        
        let mut point_index_to_vertex_index = HashMap::new();
        for point in &self.points {
            let coords = point.coords();
            writeln!(file, "v {:.2} {:.2} {:.2}", 
                coords[0] as f64 / 10.0, 
                coords[1] as f64 / 10.0, 
                coords[2] as f64 / 10.0)?;
            point_index_to_vertex_index.insert(point.number(), self.vertex_offset);
            self.vertex_offset += 1;
        }
        
        writeln!(file)?;
        
        for line in &self.lines {
            let point_indices = line.points();
            if let (Some(&start_vertex), Some(&end_vertex)) = (
                point_index_to_vertex_index.get(&point_indices[0]),
                point_index_to_vertex_index.get(&point_indices[1])
            ) {
                writeln!(file, "l {} {}", start_vertex, end_vertex)?;
            }
        }
        
        let closed_polygons = self.detect_closed_polygons();
        if !closed_polygons.is_empty() {
            writeln!(file)?;
            writeln!(file, "# Filled polygons (light gray)")?;
            for polygon in &closed_polygons {
                write!(file, "f")?;
                for point_index in polygon {
                    if let Some(&vertex_index) = point_index_to_vertex_index.get(point_index) {
                        write!(file, " {}", vertex_index)?;
                    }
                }
                writeln!(file)?;
            }
        }
        
        writeln!(file)?;
        writeln!(file, "# Material definition")?;
        writeln!(file, "newmtl LightGray")?;
        writeln!(file, "Kd 0.8 0.8 0.8  # Diffuse color (light gray)")?;
        writeln!(file, "Ka 0.2 0.2 0.2  # Ambient color")?;
        writeln!(file, "Ks 0.1 0.1 0.1  # Specular color")?;
        writeln!(file, "Ns 32           # Shininess")?;
        writeln!(file, "illum 2         # Illumination model (with highlights)")?;
        
        Ok(())
    }
    
    fn detect_closed_polygons(&self) -> Vec<Vec<i32>> {
        let mut polygons = Vec::new();
        
        let adjacency_list = self.build_adjacency_list();
        
        let mut all_cycles = Vec::new();
        for &start_point in adjacency_list.keys() {
            let cycles_from_start = self.find_all_cycles_from_start(start_point, &adjacency_list);
            all_cycles.extend(cycles_from_start);
        }
        
        let filtered_cycles = self.filter_to_simple_polygons(all_cycles);
        
        for cycle in filtered_cycles {
            if self.is_polygon_valid(&cycle) {
                polygons.push(cycle);
            }
        }
        
        polygons
    }
    
    fn build_adjacency_list(&self) -> HashMap<i32, Vec<i32>> {
        let mut adjacency_list = HashMap::new();
        
        for line in &self.lines {
            let point_indices = line.points();
            let p1 = point_indices[0];
            let p2 = point_indices[1];
            
            adjacency_list.entry(p1).or_insert_with(Vec::new).push(p2);
            adjacency_list.entry(p2).or_insert_with(Vec::new).push(p1);
        }
        
        adjacency_list
    }
    
    fn find_all_cycles_from_start(&self, start: i32, adjacency_list: &HashMap<i32, Vec<i32>>) -> Vec<Vec<i32>> {
        let mut cycles = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut current_path = Vec::new();
        
        self.find_all_cycles_dfs(start, start, adjacency_list, &mut visited, &mut current_path, &mut cycles);
        
        cycles
    }
    
    fn find_all_cycles_dfs(
        &self,
        current: i32,
        start: i32,
        adjacency_list: &HashMap<i32, Vec<i32>>,
        visited: &mut std::collections::HashSet<i32>,
        current_path: &mut Vec<i32>,
        cycles: &mut Vec<Vec<i32>>,
    ) {
        visited.insert(current);
        current_path.push(current);
        
        if let Some(neighbors) = adjacency_list.get(&current) {
            for &neighbor in neighbors {
                if neighbor == start && current_path.len() >= 3 {
                    cycles.push(current_path.clone());
                } else if !visited.contains(&neighbor) && current_path.len() < 8 {
                    self.find_all_cycles_dfs(neighbor, start, adjacency_list, visited, current_path, cycles);
                }
            }
        }
        
        visited.remove(&current);
        current_path.pop();
    }
    
    fn filter_to_simple_polygons(&self, cycles: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut simple_polygons = Vec::new();
        let mut edge_sets = std::collections::HashSet::new();
        
        let mut sorted_cycles = cycles;
        sorted_cycles.sort_by_key(|cycle| cycle.len());
        
        for cycle in sorted_cycles {
            if cycle.len() != 4 {
                continue;
            }
            
            let normalized = self.normalize_cycle(&cycle);
            let edge_set = self.get_edge_set_string(&normalized);
            
            if !edge_sets.contains(&edge_set) {
                simple_polygons.push(normalized);
                edge_sets.insert(edge_set);
            }
        }
        
        simple_polygons
    }
    
    fn normalize_cycle(&self, cycle: &[i32]) -> Vec<i32> {
        if cycle.is_empty() {
            return cycle.to_vec();
        }
        
        let min_index = cycle.iter()
            .enumerate()
            .min_by_key(|(_, &value)| value)
            .map(|(index, _)| index)
            .unwrap_or(0);
        
        let mut normalized = Vec::new();
        for i in 0..cycle.len() {
            normalized.push(cycle[(min_index + i) % cycle.len()]);
        }
        
        let mut reversed = Vec::new();
        reversed.push(normalized[0]);
        for i in (1..normalized.len()).rev() {
            reversed.push(normalized[i]);
        }
        
        for i in 0..normalized.len() {
            if reversed[i] < normalized[i] {
                return reversed;
            } else if reversed[i] > normalized[i] {
                return normalized;
            }
        }
        
        normalized
    }
    
    fn get_edge_set_string(&self, cycle: &[i32]) -> String {
        let mut edges = std::collections::HashSet::new();
        for i in 0..cycle.len() {
            let p1 = cycle[i];
            let p2 = cycle[(i + 1) % cycle.len()];
            let edge = format!("{}-{}", std::cmp::min(p1, p2), std::cmp::max(p1, p2));
            edges.insert(edge);
        }
        format!("{:?}", edges)
    }
    
    fn is_polygon_valid(&self, polygon: &[i32]) -> bool {
        if polygon.len() < 3 {
            return false;
        }
        
        for i in 0..polygon.len() {
            let p1 = polygon[i];
            let p2 = polygon[(i + 1) % polygon.len()];
            
            let mut edge_exists = false;
            for line in &self.lines {
                let point_indices = line.points();
                if (point_indices[0] == p1 && point_indices[1] == p2) ||
                   (point_indices[0] == p2 && point_indices[1] == p1) {
                    edge_exists = true;
                    break;
                }
            }
            
            if !edge_exists {
                return false;
            }
        }
        
        true
    }
}

impl Clone for PointToken {
    fn clone(&self) -> Self {
        PointToken::new(self.number(), *self.coords())
    }
}

impl Clone for LineToken {
    fn clone(&self) -> Self {
        LineToken::new(*self.points())
    }
}

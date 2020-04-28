#[macro_use]
extern crate gdnative;

use std::collections::HashMap;

#[derive(gdnative::NativeClass)]
#[inherit(gdnative::Node)]
//#[user_data(gdnative::user_data::ArcData<DijkstraMap>)]
pub struct DijkstraMap {
    connections: HashMap<i32,HashMap<i32,f32>>, //for point1 stores weights of connections going from point1 to point2
    reverse_connections: HashMap<i32,HashMap<i32,f32>>, //for point1 stores weights of connections going from point2 to point1
    cost_map: HashMap<i32,f32>,
    direction_map: HashMap<i32,i32>,
    sorted_points: Vec<i32>,
    disabled_points: std::collections::HashSet<i32>,
}



// __One__ `impl` block can have the `#[methods]` attribute, which will generate
// code to automatically bind any exported methods to Godot.
#[gdnative::methods]
impl DijkstraMap {
    
    /// The "constructor" of the class.
    fn _init(_owner: gdnative::Node) -> Self {
        DijkstraMap{
            connections: HashMap::new(),
            reverse_connections: HashMap::new(),
            cost_map: HashMap::new(),
            direction_map: HashMap::new(),
            sorted_points: Vec::new(),
            disabled_points: std::collections::HashSet::new(),
        }
    }
    
    //clears the DijkstraMap.
    #[export]
    fn clear(&mut self, mut _owner: gdnative::Node){
        self.connections.clear();
        self.reverse_connections.clear();
        self.cost_map.clear();
        self.direction_map.clear();
        self.sorted_points.clear();
        self.disabled_points.clear();
    }

    //returns next ID not associated with any point
    #[export]
    unsafe fn get_available_point_id(&mut self, mut _owner: gdnative::Node)->i32{
        let mut id:i32=0;
        while self.has_point(_owner, id) {
            id=id+1;
        }
        return id;
    }
    //Adds new point with given ID into the graph and returns true. If point with that ID already exists, does nothing and returns false.
    #[export]
    fn add_point(&mut self, mut _owner: gdnative::Node, id: i32)->bool{
        if self.has_point(_owner, id){
            return false
        }else{
            self.connections.insert(id, HashMap::new());
            self.reverse_connections.insert(id, HashMap::new());
            return true
        }
        
    }

    //Removes point from graph along with all of its connections and returns true. If point doesn't exist, returns false.
    #[export]
    fn remove_point(&mut self, mut _owner: gdnative::Node, point: i32) -> bool{
        self.disabled_points.remove(&point);
        //remove this point's entry from connections
        match self.connections.remove(&point){ 
            None=> return false,
            Some(neighbours) => {
                //remove reverse connections to this point from neighbours
                for nbr in neighbours.keys(){
                    self.reverse_connections.get_mut(nbr).unwrap().remove(&point);    
                    }
                //remove this points entry from reverse connections
                let nbrs2=self.reverse_connections.remove(&point).unwrap();
                //remove connections to this point from reverse neighbours
                for nbr in nbrs2.keys(){
                    self.connections.get_mut(nbr).unwrap().remove(&point);    
                    }
                return true
                },
        }
        
    }
    //Returns true if point exists.
    #[export]
    fn has_point(&mut self, mut _owner: gdnative::Node, id: i32)->bool{
        return self.connections.contains_key(&id);
    }

    //Disables point from pathfinding and returns true. If point doesn't exist, returns false.
    //Note: points are enabled by default.
    #[export]
    fn disable_point(&mut self, mut _owner: gdnative::Node, point: i32) -> bool{
        if self.connections.contains_key(&point){
            self.disabled_points.insert(point);
            return true
        }
        return false        
    }
    
    //Enables point for pathfinding and returns true. If point doesn't exist, returns false.
    //Note: points are enabled by default.
    #[export]
    fn enable_point(&mut self, mut _owner: gdnative::Node, point: i32) -> bool{
        if self.connections.contains_key(&point){
            self.disabled_points.remove(&point);
            return true
        }
        return false        
    }

    //Returns true if point exists and is disabled. Returns false otherwise.
    #[export]
    fn is_point_disabled(&mut self, mut _owner: gdnative::Node, point: i32) -> bool{
        if self.connections.contains_key(&point) && self.disabled_points.contains(&point){
            return true
        }
        return false        
    }

    //Adds connection with given cost (or cost of existing existing connection) between a source point and target point if they exist.
    //Returns true if connection already existed.  
    //If bidirectional is true, it also adds connection from target to source too. Returns true if connection existed in at least one direction.
    #[export]
    fn connect_points(&mut self, mut _owner: gdnative::Node, source: i32, target: i32, cost: f32, bidirectional: bool) -> bool{
        if bidirectional{
            let a=self.connect_points(_owner, source, target, cost, false);
            let b=self.connect_points(_owner, target, source, cost, false);
            return a||b
        }
        if !self.connections.contains_key(&source) || !self.reverse_connections.contains_key(&target) {
            return false
        }
        let cost_got_updated:bool;
        match self.connections.get_mut(&source){
            None=>return false,
            Some(cons)=> {
                let prev=cons.insert(target,cost);
                cost_got_updated=prev.is_some();
            }
        }
        self.reverse_connections.get_mut(&target).unwrap().insert(source,cost);
        return cost_got_updated
    }

    //Removes connection between source point and target point. Returns true if both points and their connection existed.
    //If bidirectional is true, it also removes connection from target to source. Returns true connection existed in at least one direction.
    #[export]
    fn remove_connection(&mut self, mut _owner: gdnative::Node, source: i32, target: i32,bidirectional:bool) -> bool{
        if bidirectional==true{
            let a=self.remove_connection(_owner, source, target, false);
            let b=self.remove_connection(_owner, target, source, false);
            return a||b
        }
        
        if self.has_connection(_owner, source, target){
            self.connections.get_mut(&target).unwrap().remove(&source);
            return true;
        }else{
            return false
        }
    }

    //Returns true if source point and target point both exist and there's connection from source to target.
    #[export]
    fn has_connection(&mut self, mut _owner: gdnative::Node, source: i32, target: i32) -> bool{
        match self.connections.get(&source){
            None => return false,
            Some(src) => {
                return src.contains_key(&target)
            },
        }
    }   

    //methods for recalculating Dijkstra map
    //recalculate_* methods recalculate cost and direction information for each point, overriding previous results.
    //if reversed option is true, provided target will be treated as a source (ie. shortest paths are assumed to start at the source)
    //and directions will point towards that source. 
    //If all connections in the graph are symmetric (always bidirectional with identical cost), then this option has no effect.
    //max_cost specifies a maximum total cost for a path. Algorithm will terminate after it finds all paths shorter than that.
    //all points with longer paths are treated as inaccessible.

    //receives a single point as target. 
    #[export]
    fn recalculate_for_target(&mut self, mut _owner: gdnative::Node, target: i32, max_cost: f32, reversed: bool){
        let mut targets: Vec<i32> =Vec::new();
        targets.push(target);
        self.recalculate_map_intern(&mut targets,None, max_cost,reversed);
    }

    //receives multiple points as targets in form of PoolIntArray of IDs.
    #[export]
    fn recalculate_for_targets(&mut self, mut _owner: gdnative::Node, targets_in: gdnative::Int32Array, max_cost: f32, reversed:bool){
        
        let mut targets=targets_in.read().to_vec();
       
        self.recalculate_map_intern(&mut targets,None, max_cost,reversed);
    }

    //receives multiple points as targets along with initial costs.
    //Input takes form of a dictionary with points' IDs as keys and initial costs as values.
    //Initial cost may be thought of as a biased preference. Paths will preferentially lead towards targets with lower initial cost.
    #[export]
    fn recalculate_for_targets_with_costs(&mut self, mut _owner: gdnative::Node, targets_in: gdnative::Int32Array, costs_in: gdnative::Float32Array, max_cost: f32,reversed:bool){
        
        let mut targets=targets_in.read().to_vec();
        let costs=costs_in.read().to_vec();
        self.recalculate_map_intern(&mut targets,Some(&costs), max_cost,reversed);
    }


    //functions for acccessing results

    //Given a point, returns ID of the next point along the shortest path toward target or from source.
    //If given point is the target, returns ID of itself. Returns -1, if target is inaccessible from this point.
    #[export]
    fn get_direction_at_point(&mut self, mut _owner: gdnative::Node, point:i32)->i32{
        return *self.direction_map.get(&point).unwrap_or(&-1);
    }
    //Returns cost of the shortest path from this point to the target.
    #[export]
    fn get_cost_at_point(&mut self, mut _owner: gdnative::Node, point:i32)->f32{
        return *self.cost_map.get(&point).unwrap_or(&std::f32::INFINITY);
    }

    //Given a PoolIntArray of point IDs, returns PoolIntArray of IDs of points along respective shortest paths.
    #[export]
    fn get_direction_at_points(&mut self, mut _owner: gdnative::Node, points: gdnative::Int32Array)-> gdnative::Int32Array{
        let mut dirs=gdnative::Int32Array::new();
        dirs.resize(points.len());
        {
            let points_read=points.read();
            let mut dirs_write=dirs.write();
            for i in 0..points_read.len(){
                dirs_write[i]=*self.direction_map.get(&points_read[i]).unwrap_or(&-1)
            }
        }
        return dirs
    }
    //Given a PoolIntArray of point IDs, returns PoolRealArray of costs of shortest paths from those points.
    #[export]
    fn get_cost_at_points(&mut self, mut _owner: gdnative::Node, points: gdnative::Int32Array)-> gdnative::Float32Array{
        let mut costs=gdnative::Float32Array::new();
        costs.resize(points.len());
        {
            let points_read=points.read();
            let mut costs_write=costs.write();
            for i in 0..points_read.len(){
                costs_write[i]=*self.cost_map.get(&points_read[i]).unwrap_or(&std::f32::INFINITY);
            }
        }
        return costs
    }

    //Returns the entire Dijktra map of costs in form of a dictionary. Keys are points' IDs and values are costs.
    //Inaccessible points are not present in the dictionary.
    #[export]
    fn get_cost_map(&mut self, mut _owner: gdnative::Node)->gdnative::Dictionary{
        let mut dict=gdnative::Dictionary::new();
        for id in self.sorted_points.iter(){
            dict.set(&gdnative::Variant::from_i64(*id as i64), &gdnative::Variant::from_f64(self.cost_of(*id) as f64));
        }
        return dict
    }

    //Returns the entire Dijkstra map of directions 
    #[export]
    fn get_direction_map(&mut self, mut _owner: gdnative::Node)->gdnative::Dictionary{
        let mut dict=gdnative::Dictionary::new();
        for id in self.sorted_points.iter(){
            dict.set(&gdnative::Variant::from_i64(*id as i64), &gdnative::Variant::from_i64(*self.direction_map.get(id).unwrap() as i64));
        }
        return dict
    }

    //returns all points with costs between min_cost and max_cost (inclusive), in sorted order.
    #[export]
    fn get_all_points_with_cost_between(&mut self, mut _owner: gdnative::Node,min_cost:f32,max_cost:f32)->gdnative::Int32Array{
        let start_point=match self.sorted_points.binary_search_by(
            |a|
            {if self.cost_of(*a)<min_cost{
                return std::cmp::Ordering::Less
            }else{
                return std::cmp::Ordering::Greater
            }}
            ){Ok(a)=>a,Err(a)=>a};
        
        let end_point=match self.sorted_points.binary_search_by(
            |a|
            {if self.cost_of(*a)>max_cost{
                return std::cmp::Ordering::Greater
            }else{
                return std::cmp::Ordering::Less
            }}
            ){Ok(a)=>a,Err(a)=>a};
        
        let slice=start_point..end_point;
        let mut poolintarray=gdnative::Int32Array::new();
        poolintarray.resize(slice.len() as i32);
        {
        let mut pool_write_access=poolintarray.write();
        for i in slice{
            //poolintarray.set((i-start_point) as i32, self.sorted_points[i]);
            pool_write_access[i-start_point]=self.sorted_points[i];
        }
        }
        return poolintarray
    }
    
    //returns PoolIntArray of point IDs corresponding to a shortest path from given point (note: given point isn't included).
    //If point is a target or is inaccessible, returns empty array.
    #[export]
    fn get_shortest_path_from_point(&mut self, mut _owner: gdnative::Node, point:i32)-> gdnative::Int32Array{
        let mut path: Vec<i32>=Vec::new();
        let mut next_point=self.get_direction_at_point(_owner, point);
        let mut current_point: i32=point;
        
        while current_point!=next_point || next_point!=-1 {
            path.push(next_point);
            current_point=next_point;
            next_point=self.get_direction_at_point(_owner, current_point);
        }

        let mut out_array=gdnative::Int32Array::new();
        if path.len()>0{
            out_array.resize(path.len() as i32);
            let mut path_write=out_array.write();
            for i in 0..path.len(){
                path_write[i]=path[i];
            }
        }
        return out_array
    }


    fn cost_of(&self ,a:i32)->f32 {
        *self.cost_map.get(&a).unwrap_or(&std::f32::INFINITY)
    }

    fn compare_cost(&self, a:i32, b:i32)->std::cmp::Ordering{
        if self.cost_of(a)<self.cost_of(b){
            return std::cmp::Ordering::Greater
        }else{
            return std::cmp::Ordering::Less
        }
    }

    //internal
    //recalculates the cost map and direction map in given direction
    //receives hashmap of sources with initial costs
    //stops updating once maximum cost is reached
    fn recalculate_map_intern(&mut self, open_set: &mut Vec<i32>, initial_costs: Option<&Vec<f32>> ,max_cost: f32, reversed: bool){
        //initialize containers
        self.cost_map.clear();
        self.direction_map.clear();
        self.sorted_points.clear();
        let capacity=std::cmp::max((f32::sqrt(self.connections.len() as f32) as usize) * 6, open_set.len());
        open_set.reserve( capacity-open_set.len() );
        let mut open_set_set =std::collections::HashSet::<i32>::with_capacity(capacity);
        
        //switches direction of connections
        let connections = if reversed {&self.connections}else{ &self.reverse_connections};
        
        

        //add targets to open_set
        {
            let mut invalid_targets:Vec<usize>=Vec::new();

            for (src,i) in open_set.iter().zip(0..){
                if connections.contains_key(src){
                    self.direction_map.insert(*src, *src);
                    self.cost_map.insert(*src, match initial_costs{None=>0.0,Some(t)=>*t.get(i).unwrap_or(&0.0)});
                    open_set_set.insert(*src);
                }else{
                    invalid_targets.push(i); //mark invalid targets for removal
                }
            }
            for i in invalid_targets{
                open_set.remove(i);
            }
        }
        open_set.sort_unstable_by(|a,b| self.compare_cost(*a, *b) );

        
        let mut c=connections.len() as i32;
        //iterrate over open_set
        while !(open_set.is_empty()) && c>=0 {
            c=c-1;
            //pop point with smallest cost
            let point1=open_set.pop().unwrap();
            open_set_set.remove(&point1);
            self.sorted_points.push(point1);
            let point1_cost=self.cost_of(point1);
            //iterrate over it's neighbours
            for (&point2,dir_cost) in connections.get(&point1).unwrap().iter(){
                let cost=dir_cost+point1_cost;
                //add to the open set (or update values if already present)
                //if point is enabled and new cost is better than old one, but not bigger than maximum cost
                if cost<self.cost_of(point2) && cost<=max_cost && !self.disabled_points.contains(&point2) {
                    
                    //remove from open_set if already present
                    if open_set_set.remove(&point2){
                        open_set.remove(open_set.iter().position(|&x| x==point2).unwrap());
                    }

                    self.direction_map.insert(point2,point1);
                    self.cost_map.insert(point2, cost);
                    let insertion=match open_set.binary_search_by(|a| self.compare_cost(*a, point2)){Err(i)=>i,Ok(i)=>i};
                    open_set.insert(insertion, point2);
                    open_set_set.insert(point2);
                    
                }

            }
        } 
    }
    

    
    //initializes map as a 2D grid. Walkable tiles are specified by BitMap (true=>point gets added, false=>point gets ignored).
    //point IDs are setup such that ID=(x+w*width)+initial_offset. Conversely x=(ID-initial_offset)%width and y=(ID-initial_offset)/width
    //warning: If points with reqired IDs already exist, this method will treat them as part of the grid. 
    //second argument is a Dictionary, that defines connections. Keys are relative positions of points in the grid and values are costs.
    //Example for orthogonal (4-directional) movement {Vector2(1,0): 1.0, Vector(0,1): 1.0, Vector2(-1,0): 1.0, Vector(0,-1): 1.0}
    #[export]
    unsafe fn initialize_as_grid(&mut self, mut _owner: gdnative::Node, bitmap: gdnative::BitMap, relative_connections_in: gdnative::Dictionary, initial_offset: i32){
        let vec_size=bitmap.get_size();
        let width=vec_size.x as i32;
        let height=vec_size.y as i32;
        let mut relative_connections: HashMap<i32,f32>=HashMap::new();

        //extract relative connections to rust types.
        for dirs in relative_connections_in.keys().iter(){
            match dirs.try_to_vector2(){
                None=>continue,
                Some(vec2)=>{
                    let cost=relative_connections_in.get(dirs);
                    relative_connections.insert((vec2.x as i32)+(vec2.y as i32)*width ,cost.to_f64() as f32);
                }
            }  
        }

        let mut grid=std::collections::HashSet::<i32>::new();
        for y in 0..height{
            for x in 0..width{
                if bitmap.get_bit(gdnative::Vector2::new(x as f32,y as f32)){
                    self.add_point(_owner, x+y*width+initial_offset);
                    grid.insert(x+y*width+initial_offset);
                }
        
            }
        }
        
        for y in 0..height{
            for x in 0..width{
                let id=y*x;
                for (offs,cost) in relative_connections.iter(){
                    if grid.contains(&(id+offs+initial_offset)){
                        self.connect_points(_owner, id+initial_offset, id+offs+initial_offset, *cost, false);
                    }
                }
            }
        }


    }
   
   
}


// Function that registers all exposed classes to Godot
fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<DijkstraMap>();
}

// macros that create the entry-points of the dynamic library.
godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();

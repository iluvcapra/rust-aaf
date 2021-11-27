use anymap::AnyMap;

/// A Session maintains the AAF object graph
struct Session {
    objects : AnyMap,
}

impl Session {
    fn new() -> Self {
        objects: AnyMap::new()
    }

    fn add_object<T>(&mut self, object: T) -> usize {
        if !self.objects.contains::<Vec<T>>() {
            let v : Vec<T> = Vec::new();
            self.objects.insert(v);
        }
        let mut list : Vec<T> = self.objects.get_mut();
        let retval = list.len();
        list.push(object);
        retval 
    }
}

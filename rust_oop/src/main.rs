pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}


fn main() {
    // an instace of average collection
    let mut ac_list = AveragedCollection {
        list: vec![10, 20, 30],
        average: 0.0,
    };

    // Initial update of the average
    ac_list.update_average();

    println!("Initial average: {}", ac_list.average());

    // Add a value
    ac_list.add(40);
    println!("Average after adding 40: {}", ac_list.average());

    // Remove a value
    ac_list.remove();
    println!("Average after removing last element: {}", ac_list.average());
}

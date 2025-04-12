use alloc::vec;
use alloc::vec::Vec;

const MAX_TASKS: usize = 20;
const MAX_RESOURCE: usize = 20;

/// detect deadlocd
pub struct Detector {
    available: Vec<usize>,
    allocated: Vec<Vec<usize>>,
    need: Vec<Vec<usize>>,
    resource_number: usize,
}

impl Detector {
    /// Create a new instance of the Detector
    pub fn new() -> Self {
        Self {
            available: vec![0; MAX_RESOURCE],
            allocated: vec![vec![0; MAX_RESOURCE]; MAX_TASKS],
            need: vec![vec![0; MAX_RESOURCE]; MAX_TASKS],
            resource_number: 0,
        }
    }

    /// n -> number of task
    /// m -> number of resource
    pub fn is_safe(&self) -> bool {
        let mut work = self.available.clone();
        let mut finish = vec![false; self.allocated.len()];
        let mut loop_limit = self.allocated.len();

        while loop_limit > 0 {
            for i in 0..self.allocated.len() {
                // find a process
                if !finish[i] && self.need[i].iter().zip(work.iter()).all(|(n, w)| n <= w) {
                    for j in 0..work.len() {
                        work[j] += self.allocated[i][j];
                    }
                    finish[i] = true;
                }
            }
            loop_limit -= 1;
        }

        log::info!(
            "issafe??? available {:?} finish {:?} need {:?} allocated {:?}",
            self.available,
            finish,
            self.need,
            self.allocated
        );
        if finish.iter().all(|f| *f) {
            return true; // 所有进程均能完成 → 系统安全
        } else {
            return false; // 存在未完成的进程 → 系统不安全（可能死锁）
        }
    }

    /// Add a new resource to the available resources
    pub fn add_resource(&mut self, res_count: usize) {
        self.available[self.resource_number] += res_count;
        self.resource_number += 1;
    }

    /// add need of a resource to a task
    pub fn require_resource(&mut self, task: usize, id: usize) {
        self.need[task][id] += 1;
    }

    /// add a resource to a task
    pub fn alloc_resource(&mut self, task: usize, id: usize) {
        self.allocated[task][id] += 1;
        self.available[id] -= 1;
        self.need[task][id] -= 1;
        log::info!(
            "task {} alloc resource {} available {:?}",
            task,
            id,
            self.available
        );
    }

    /// Release a resource from the allocated resources
    pub fn release_resource(&mut self, task: usize, id: usize) {
        self.allocated[task][id] -= 1;
        self.available[id] += 1;
        log::info!(
            "task {} release resource {} available {:?}",
            task,
            id,
            self.available
        );
    }
}

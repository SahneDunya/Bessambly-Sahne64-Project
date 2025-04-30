use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum MemorySection {
    Static,
    // Stack ve Heap yönetimi Sahne64 API'si üzerinden yapılabilir
    Handle, // Handle'ları takip etmek için ayrı bir bölüm
}

#[derive(Debug, Clone)]
pub struct MemoryAllocation {
    pub section: MemorySection,
    pub size: usize,
    pub address: usize, // Sanal adres (derleyici için) veya handle değeri
    // Diğer meta bilgiler (değişken adı vb.) eklenebilir
}

pub struct MemoryManager {
    static_allocations: HashMap<String, MemoryAllocation>, // Değişken adı -> Tahsisat
    handle_allocations: HashMap<String, MemoryAllocation>, // Handle adı -> Tahsisat (sembolik takip)
    next_static_address: usize,
    // Stack ve Heap boyutları/yönetimi derleyici tarafından doğrudan yapılmayabilir
}

impl MemoryManager {
    pub fn new(static_base: usize) -> Self {
        MemoryManager {
            static_allocations: HashMap::new(),
            handle_allocations: HashMap::new(),
            next_static_address: static_base,
        }
    }

    pub fn allocate_static(&mut self, name: &str, size: usize) -> Result<MemoryAllocation, String> {
        if self.static_allocations.contains_key(name) {
            return Err(format!("Statik değişken '{}' zaten tanımlı", name));
        }
        let allocation = MemoryAllocation {
            section: MemorySection::Static,
            size,
            address: self.next_static_address,
        };
        self.static_allocations.insert(name.to_string(), allocation.clone());
        self.next_static_address += size;
        Ok(allocation)
    }

    pub fn get_static_allocation(&self, name: &str) -> Option<&MemoryAllocation> {
        self.static_allocations.get(name)
    }

    // Handle yönetimi (sembolik olarak)
    pub fn allocate_handle(&mut self, name: &str) -> Result<MemoryAllocation, String> {
        if self.handle_allocations.contains_key(name) {
            return Err(format!("Handle '{}' zaten tanımlı", name));
        }
        let allocation = MemoryAllocation {
            section: MemorySection::Handle,
            size: std::mem::size_of::<usize>(), // Handle boyutu platforma bağlı
            address: 0, // Gerçek değeri Sahne64 atayacak
        };
        self.handle_allocations.insert(name.to_string(), allocation.clone());
        Ok(allocation)
    }

    pub fn get_handle_allocation(&self, name: &str) -> Option<&MemoryAllocation> {
        self.handle_allocations.get(name)
    }

    // Handle'ı serbest bırakma (sembolik takip)
    pub fn release_handle(&mut self, name: &str) -> Result<(), String> {
        if !self.handle_allocations.contains_key(name) {
            return Err(format!("Tanımsız handle '{}'", name));
        }
        self.handle_allocations.remove(name);
        Ok(())
    }

    // İleride sembol tablosu ile entegrasyon gerekebilir
}
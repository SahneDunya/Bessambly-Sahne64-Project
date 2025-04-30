#[derive(Debug, Clone, PartialEq)]
pub enum BaseType {
    Integer { signed: bool, size: IntegerSize },
    Pointer,
    String, // Sahne64 kaynak adları için
    // Handle ve TaskId aslında usize olabilir, ancak semantik analizde ayırt etmek için buraya ekleyebiliriz
    Handle,
    TaskId,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IntegerSize {
    Byte,   // 8-bit
    Word,   // 16-bit
    DWord,  // 32-bit
    QWord,  // 64-bit (Handle ve TaskId için uygun olabilir)
    USize,  // Platforma bağımlı boyut (Handle ve TaskId için yaygın)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub base: BaseType,
}

impl Type {
    pub fn integer(signed: bool, size: IntegerSize) -> Self {
        Type { base: BaseType::Integer { signed, size } }
    }

    pub fn pointer() -> Self {
        Type { base: BaseType::Pointer }
    }

    pub fn string() -> Self {
        Type { base: BaseType::String }
    }

    pub fn handle() -> Self {
        Type { base: BaseType::Handle }
    }

    pub fn task_id() -> Self {
        Type { base: BaseType::TaskId }
    }

    pub fn size(&self) -> Option<usize> {
        match self.base {
            BaseType::Integer { size, .. } => match size {
                IntegerSize::Byte => Some(1),
                IntegerSize::Word => Some(2),
                IntegerSize::DWord => Some(4),
                IntegerSize::QWord => Some(8),
                IntegerSize::USize => Some(std::mem::size_of::<usize>()),
            },
            BaseType::Pointer => Some(std::mem::size_of::<usize>()),
            BaseType::String => None, // String'in boyutu dinamik olabilir
            BaseType::Handle => Some(std::mem::size_of::<usize>()),
            BaseType::TaskId => Some(std::mem::size_of::<usize>()),
        }
    }
}

// Örnek sabit tanımları
pub const TYPE_BYTE: Type = Type::integer(true, IntegerSize::Byte);
pub const TYPE_WORD: Type = Type::integer(true, IntegerSize::Word);
pub const TYPE_DWORD: Type = Type::integer(true, IntegerSize::DWord);
pub const TYPE_QWORD: Type = Type::integer(true, IntegerSize::QWord);
pub const TYPE_USIZE: Type = Type::integer(true, IntegerSize::USize);
pub const TYPE_POINTER: Type = Type::pointer();
pub const TYPE_STRING: Type = Type::string();
pub const TYPE_HANDLE: Type = Type::handle();
pub const TYPE_TASK_ID: Type = Type::task_id();
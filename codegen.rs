use crate::parser::{AstNode, Expression};
use crate::symbol_table::SymbolTable;
use crate::memory_manager::MemoryManager;
use crate::types::{Type, IntegerSize}; // Tip bilgisi gerekebilir

pub struct CodeGenerator {
    symbol_table: SymbolTable,
    memory_manager: MemoryManager,
    output: Vec<String>, // Üretilen (metin tabanlı veya bayt kodu olabilir) kod
}

impl CodeGenerator {
    pub fn new(symbol_table: SymbolTable, memory_manager: MemoryManager) -> Self {
        CodeGenerator {
            symbol_table,
            memory_manager,
            output: Vec::new(),
        }
    }

    pub fn generate_code(&mut self, ast: &AstNode) -> &Vec<String> {
        self.output.clear();
        match ast {
            AstNode::Program(statements) => {
                for statement in statements {
                    self.generate_statement(statement);
                }
            }
            _ => {}
        }
        &self.output
    }

    fn generate_statement(&mut self, node: &AstNode) {
        match node {
            AstNode::Label { name } => {
                self.emit_line(&format!("{}:", name));
            }
            AstNode::Assignment { variable, value } => {
                let symbol = self.symbol_table.lookup(variable).expect("Değişken bulunamadı");
                if let Expression::Number(num) = value {
                    if symbol.scope == crate::symbol_table::Scope::Global {
                        if let Some(allocation) = self.memory_manager.get_static_allocation(&symbol.name) {
                            self.emit_line(&format!("MOV [{}], {}", allocation.address, num));
                        }
                    } else {
                        // Yığın veya register ataması (basitlik için atlandı)
                        self.emit_line(&format!("MOV {}, {}", variable, num));
                    }
                } else if let Expression::Identifier(other_var) = value {
                    self.emit_line(&format!("MOV {}, {}", variable, other_var));
                }
            }
            AstNode::JumpStatement { target } => {
                self.emit_line(&format!("JUMP {}", target));
            }
            AstNode::AllocateMemory { size, handle } => {
                let size_operand = self.generate_expression(size);
                let handle_symbol = self.symbol_table.lookup(handle).expect("Handle bulunamadı");
                // Sahne64 API çağrısı (numara ve argümanlar varsayımsal)
                self.emit_line(&format!("SYS_CALL {}", 1)); // Örn: ALLOCATE sistem çağrı numarası
                self.emit_line(&format!("ARG {}", size_operand));
                self.emit_line(&format!("RES {}", handle_symbol.name)); // Sonuç handle'a yazılacak
            }
            AstNode::ReleaseMemory { handle } => {
                let handle_operand = self.generate_expression(handle);
                self.emit_line(&format!("SYS_CALL {}", 2)); // Örn: RELEASE sistem çağrı numarası
                self.emit_line(&format!("ARG {}", handle_operand));
            }
            AstNode::SpawnTask { function, priority } => {
                let function_operand = self.generate_expression(&Expression::Identifier(function.clone()));
                self.emit_line(&format!("SYS_CALL {}", 3)); // Örn: SPAWN sistem çağrı numarası
                self.emit_line(&format!("ARG {}", function_operand));
                if let Some(prio) = priority {
                    let prio_operand = self.generate_expression(prio);
                    self.emit_line(&format!("ARG {}", prio_operand));
                }
            }
            AstNode::ExitTask { code } => {
                self.emit_line(&format!("SYS_CALL {}", 4)); // Örn: EXIT sistem çağrı numarası
                if let Some(c) = code {
                    let code_operand = self.generate_expression(c);
                    self.emit_line(&format!("ARG {}", code_operand));
                }
            }
            AstNode::SleepTask { duration } => {
                let duration_operand = self.generate_expression(duration);
                self.emit_line(&format!("SYS_CALL {}", 5)); // Örn: SLEEP sistem çağrı numarası
                self.emit_line(&format!("ARG {}", duration_operand));
            }
            AstNode::YieldTask => {
                self.emit_line(&format!("SYS_CALL {}", 6)); // Örn: YIELD sistem çağrı numarası
            }
            AstNode::AcquireResource { name, handle } => {
                let name_operand = self.generate_expression(name);
                let handle_symbol = self.symbol_table.lookup(handle).expect("Handle bulunamadı");
                self.emit_line(&format!("SYS_CALL {}", 7)); // Örn: ACQUIRE sistem çağrı numarası
                self.emit_line(&format!("ARG {}", name_operand));
                self.emit_line(&format!("RES {}", handle_symbol.name));
            }
            AstNode::ControlResource { handle, command } => {
                let handle_operand = self.generate_expression(handle);
                let command_operand = self.generate_expression(command);
                self.emit_line(&format!("SYS_CALL {}", 8)); // Örn: CTRL sistem çağrı numarası
                self.emit_line(&format!("ARG {}", handle_operand));
                self.emit_line(&format!("ARG {}", command_operand));
            }
            AstNode::SendMessage { handle, message } => {
                let handle_operand = self.generate_expression(handle);
                let message_operand = self.generate_expression(message);
                self.emit_line(&format!("SYS_CALL {}", 9)); // Örn: SEND sistem çağrı numarası
                self.emit_line(&format!("ARG {}", handle_operand));
                self.emit_line(&format!("ARG {}", message_operand));
            }
            AstNode::ReceiveMessage { handle, buffer } => {
                let handle_operand = self.generate_expression(handle);
                let buffer_symbol = self.symbol_table.lookup(buffer).expect("Buffer bulunamadı");
                self.emit_line(&format!("SYS_CALL {}", 10)); // Örn: RECV sistem çağrı numarası
                self.emit_line(&format!("ARG {}", handle_operand));
                self.emit_line(&format!("RES {}", buffer_symbol.name)); // Alınan mesaj buffer'a yazılacak
            }
            AstNode::GetTaskId { target } => {
                let target_symbol = self.symbol_table.lookup(target).expect("Hedef bulunamadı");
                self.emit_line(&format!("SYS_CALL {}", 11)); // Örn: GET_TASK_ID
                self.emit_line(&format!("RES {}", target_symbol.name));
            }
            AstNode::GetCoreId { target } => {
                let target_symbol = self.symbol_table.lookup(target).expect("Hedef bulunamadı");
                self.emit_line(&format!("SYS_CALL {}", 12)); // Örn: GET_CORE_ID
                self.emit_line(&format!("RES {}", target_symbol.name));
            }
            AstNode::GetTotalCores { target } => {
                let target_symbol = self.symbol_table.lookup(target).expect("Hedef bulunamadı");
                self.emit_line(&format!("SYS_CALL {}", 13)); // Örn: GET_TOTAL_CORES
                self.emit_line(&format!("RES {}", target_symbol.name));
            }
            AstNode::Instruction { opcode, operands } => {
                let operand_strs: Vec<String> = operands.iter().map(|op| self.generate_expression(op)).collect();
                self.emit_line(&format!("{} {}", opcode, operand_strs.join(", ")));
            }
            _ => {}
        }
    }

    fn generate_expression(&self, expression: &Expression) -> String {
        match expression {
            Expression::Identifier(name) => name.clone(),
            Expression::Number(num) => num.to_string(),
            Expression::Flag(flag) => flag.clone(),
            Expression::StringLiteral(s) => format!("\"{}\"", s),
            // Handle ve TaskId da identifier olarak ele alınabilir (sembol tablosunda tutuluyor)
            _ => panic!("Beklenmeyen ifade türü"),
        }
    }

    fn emit_line(&mut self, line: &str) {
        self.output.push(line.to_string());
    }
}
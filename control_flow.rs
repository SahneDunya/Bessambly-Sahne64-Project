use crate::parser::AstNode;
use crate::semantic_analyzer::SemanticAnalyzer;

impl SemanticAnalyzer {
    pub fn analyze_control_flow(&mut self, ast: &AstNode) {
        match ast {
            AstNode::Program(statements) => {
                for statement in statements {
                    self.analyze_control_flow_statement(statement);
                }
            }
            _ => {}
        }
    }

    fn analyze_control_flow_statement(&mut self, node: &AstNode) {
        match node {
            AstNode::JumpStatement { target } => {
                println!("Kontrol akışı: JUMP -> {}", target);
            }
            AstNode::FlagDeclaration { flag } => {
                println!("Kontrol akışı: FLAG {}", flag);
            }
            AstNode::Label { name } => {
                println!("Kontrol akışı: Etiket '{}'", name);
            }
            AstNode::SpawnTask { function, priority } => {
                println!("Kontrol akışı: SPAWN yeni görev '{}' (öncelik: {:?})", function, priority);
                // Burada yeni bir kontrol akışı bloğu başlayabilir (ileride daha detaylı analiz için)
            }
            AstNode::ExitTask { code } => {
                println!("Kontrol akışı: EXIT görev (kod: {:?})", code);
                // Bu noktadan sonraki kod ulaşılamaz olabilir (ileride analiz edilebilir)
            }
            AstNode::YieldTask => {
                println!("Kontrol akışı: YIELD görev");
                // Görev zamanlayıcıya kontrolü bırakır
            }
            AstNode::Instruction { opcode, operands } => {
                println!("Kontrol akışı: {} {:?}", opcode, operands);
                // CALL, RET gibi opcodelar kontrol akışını değiştirebilir
            }
            AstNode::Assignment { .. } |
            AstNode::AllocateMemory { .. } |
            AstNode::ReleaseMemory { .. } |
            AstNode::AcquireResource { .. } |
            AstNode::ControlResource { .. } |
            AstNode::SendMessage { .. } |
            AstNode::ReceiveMessage { .. } |
            AstNode::GetTaskId { .. } |
            AstNode::GetCoreId { .. } |
            AstNode::GetTotalCores { .. } => {
                // Bu API çağrıları genellikle doğrudan kontrol akışını değiştirmez
            }
            _ => {}
        }
    }

    // İleride daha karmaşık kontrol akışı analizleri eklenebilir:
    // - Görevler arası senkronizasyon analizi (ACQUIRE/RELEASE)
    // - Mesajlaşma örüntüleri analizi (SEND/RECV)
    // - Olası dead-lock durumları (basit düzeyde)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::semantic_analyzer::SemanticAnalyzer;

    #[test]
    fn test_control_flow_analysis_with_sahne64() {
        let input = "
            start:
                ALLOCATE 1024 AS handle1
                SPAWN worker_func WITH prio=5
                JUMP target
            target:
                YIELD
                EXIT 0
        ";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let ast = parser.parse_program();

        let mut analyzer = SemanticAnalyzer::new();
        analyzer.analyze(&ast); // Önce semantik analiz yapılmalı

        analyzer.analyze_control_flow(&ast);
        // Bu test şu anda sadece çıktı üretiyor, daha detaylı analizler eklenebilir.
    }
}
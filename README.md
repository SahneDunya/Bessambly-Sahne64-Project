# Bessambly-Sahne64-Project
Bessambly is the lowest level language developed by Sahne Dünya where every code is not hardware specific. The reason for the development of Bessambly programming language is to provide cross-platform support in terms of CPU instruction set for low-level operations. This programming language basically supports imperative programming. It uses Jump and Flag as control structures. In this language, memories are managed manually but as follows: Static Memory Allocation, Stack Management, Heap (with operating system calls) Management. This language lacks import and include etc. features instead Procedures (Subprograms), Combining Separate Files, Macros and External Symbols (EXTERN/GLOBAL) are used for modularity and code reuse. Every code in this programming language is not hardware specific but every code in this language is operating system specific, each operating system has its own Bessambly. Although Bessambly programming language does not have hardware specific codes, it contains operating system specific codes. This is because in such low-level languages, system calls are required for basic features such as Heap Management, Input/Output (I/O) Operations, etc.

# Basic features
* File extension: .b64 (For Sahne64 Bessambly)
* Memory Management: Manually
* Compilation type: Ahead-of-Time
* The executable file developed for it: .bs64
* Underlying programming language: none but similar to Assembly
* Modern language features: no
* Standard Libray: no

# Target Hello World code 
```
; Sahne64 Bessambly "Merhaba Dünya" Programı
; Donanımdan bağımsız ama Sahne64 OS'ye özel

.data        ; Statik veri bölümü tanımlama
hello_msg:   .string "Merhaba Dünya!\n" ; Konsola yazılacak metin. Sonundaki '\n' yeni satır karakteri.
hello_len =  . - hello_msg            ; Metnin uzunluğunu hesapla (mevcut adres '.' ile hello_msg adresi arasındaki fark)

console_id:  .string "sahne://console" ; Konsol kaynağının Sahne64 içindeki adresi/ID'si
console_id_len = . - console_id       ; Konsol ID stringinin uzunluğu

.code        ; Kod bölümü tanımlama
.global _start ; Programın giriş noktası (linker için global sembol)

_start:
    ; Adım 1: Konsol Kaynağı Handle'ını Edinme
    ; SYSCALL_RESOURCE_ACQUIRE = 5
    ; Arg1 (R1): Resource ID Adresi ($console_id)
    ; Arg2 (R2): Resource ID Uzunluğu (console_id_len)
    ; Arg3 (R3): Mode (Yazma izni için resource::MODE_WRITE = 2)
    ; Diğer argümanlar (R4, R5) şimdilik 0
    LOAD R0, 5               ; R0'a syscall numarası 5'i yükle (RESOURCE_ACQUIRE)
    LOAD R1, $console_id     ; R1'e console_id stringinin adresini yükle
    LOAD R2, console_id_len  ; R2'ye console_id stringinin uzunluğunu yükle
    LOAD R3, 2               ; R3'e yazma modunu yükle (MODE_WRITE)
    LOAD R4, 0               ; R4'ü 0 yap (kullanılmayan arg)
    LOAD R5, 0               ; R5'i 0 yap (kullanılmayan arg)
    SYSCALL                  ; Sistemi çağır! Çekirdek SYSCALL handler'ı devreye girer.
    ; SYSCALL dönüş değeri R0'a yazılır. Başarılıysa Handle (>=0), hata ise negatif kod (<0).

    ; Handle'ı kaydet ve Hata Kontrolü
    MOV R7, R0             ; R0'daki Handle'ı R7'ye taşı (R0'ı kontrol edeceğiz)
    ; Bessambly'de flag register veya R0'a dayalı atlama varsayalım
    JLT _error_exit        ; Eğer R0 (dönüş kodu) 0'dan küçükse (hata), _error_exit'e atla.

    ; Adım 2: Handle'ı Kullanarak "Merhaba Dünya!" Yazma
    ; SYSCALL_RESOURCE_WRITE = 7
    ; Arg1 (R1): Resource Handle (R7'de saklı)
    ; Arg2 (R2): Buffer Adresi ($hello_msg)
    ; Arg3 (R3): Buffer Uzunluğu (hello_len)
    LOAD R0, 7               ; R0'a syscall numarası 7'yi yükle (RESOURCE_WRITE)
    MOV R1, R7               ; R1'e konsol Handle'ını yükle (Handle edinme adımından)
    LOAD R2, $hello_msg      ; R2'ye hello_msg stringinin adresini yükle
    LOAD R3, hello_len       ; R3'e hello_msg stringinin uzunluğunu yükle
    LOAD R4, 0               ; R4'ü 0 yap (kullanılmayan arg)
    LOAD R5, 0               ; R5'i 0 yap (kullanılmayan arg)
    SYSCALL                  ; Sistemi çağır!
    ; SYSCALL dönüş değeri R0'a yazılır. Başarılıysa yazılan byte sayısı (>=0), hata ise negatif kod (<0).

    ; Hata Kontrolü (Yazma)
    JLT _error_exit          ; Eğer R0 (dönüş kodu) 0'dan küçükse (hata), _error_exit'e atla.
    ; Eğer başarılı ise, yazılan byte sayısı R0'dadır. Bu örnekte kullanmıyoruz.

    ; Adım 3: Programı Başarılı Bir Şekilde Sonlandırma
    ; SYSCALL_TASK_EXIT = 4
    ; Arg1 (R1): Çıkış Kodu (0 = Başarı)
    LOAD R0, 4               ; R0'a syscall numarası 4'ü yükle (TASK_EXIT)
    LOAD R1, 0               ; R1'e çıkış kodu 0'ı yükle (Başarı)
    SYSCALL                  ; Sistemi çağır! Bu çağrı geri dönmez.

_error_exit:
    ; Hata Durumunda Çıkış
    ; SYSCALL_TASK_EXIT = 4
    ; Arg1 (R1): Çıkış Kodu (Örnek: 1 = Genel Hata)
    LOAD R0, 4               ; R0'a syscall numarası 4'ü yükle (TASK_EXIT)
    LOAD R1, 1               ; R1'e hata çıkış kodu 1'i yükle
    SYSCALL                  ; Sistemi çağır! Bu çağrı geri dönmez.

; Buraya asla ulaşılmamalıdır, çünkü TASK_EXIT çağrıları programı sonlandırır.
; Ancak bir hata durumunda sonsuz döngü gibi bir fallback eklenebilir.
_halt:
    JMP _halt ; İşlemciyi durdurmak için sonsuz döngü (olası bir fallback)

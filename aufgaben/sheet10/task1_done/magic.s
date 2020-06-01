magic:
    push    rbp         ;Rücksprungadresse auf Stack legen
    mov     rbp, rsp    ;Stackende ist neuer Base Pointer
    mov     rcx, rdi    ;Argument aus Funktion nach RCX laden
    mov     esi, 2      ;rsi mit 2 initialisieren
.LBB0_1:
    mov     al, 1       ;rax mit 1 initialisieren
    cmp     rsi, rcx    ;vergleiche RSI und RCX
    jae     .LBB0_4     ;jump if RSI > RCX
    xor     edx, edx    ;setze rdx zu 0
    mov     rax, rcx    ;rax := rcx
    div     rsi         ;divide rax/rsi, result: rax (remainder rdx)
    inc     rsi         ;rsi++
    test    rdx, rdx    ; Rest der Division
    jne     .LBB0_1     ; if rdx!=0, jump back to LBB0_1
    xor     eax, eax    ; eax = 0
.LBB0_4:
    pop     rbp
    ret

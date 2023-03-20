const A1: u8 = 3;
const B1: u8 = 4;
const C1: u8 = 5;
const D1: u8 = 6;
const A2: u8 = 7;
const B2: u8 = 8;
const C2: u8 = 9;
const D2: u8 = 10;
const A3: u8 = 11;
const B3: u8 = 12;
const C3: u8 = 13;
const D3: u8 = 14;
const A4: u8 = 15;
const B4: u8 = 16;
const C4: u8 = 2;
const D4: u8 = 1;
const POT: u8 = A3;
const A: [u8; 4] = [A1, A2, A3, A4];
const B: [u8; 4] = [B1, B2, B3, B4];
const C: [u8; 4] = [C1, C2, C3, C4];
const D: [u8; 4] = [D1, D2, D3, D4];

fn setup() {
    for i in 0..4 {
        pinMode(A[i] as u8, OUTPUT);
        pinMode(B[i] as u8, OUTPUT);
        pinMode(C[i] as u8, OUTPUT);
        pinMode(D[i] as u8, OUTPUT);
        digitalWrite(A[i] as u8, HIGH);
        digitalWrite(B[i] as u8, HIGH);
        digitalWrite(C[i] as u8, HIGH);
        digitalWrite(D[i] as u8, HIGH);
    }
    pinMode(POT as u8, INPUT);
}

fn r#loop() {
    for i in 0..10 {
        for m in 0..4 {
            writenumber(m as u8, i as u8);
        }
        delay(500);
    }
}

fn writenumber(a: u8, b: u8) {
    match b {
        0 => {
            digitalWrite(A[a as usize], LOW);
            digitalWrite(B[a as usize], LOW);
            digitalWrite(C[a as usize], LOW);
            digitalWrite(D[a as usize], LOW);
        }
        9 => {
            digitalWrite(A[a as usize], HIGH);
            digitalWrite(B[a as usize], LOW);
            digitalWrite(C[a as usize], LOW);
            digitalWrite(D[a as usize], LOW);
        }
        8 => {
            digitalWrite(A[a as usize], LOW);
            digitalWrite(B[a as usize], HIGH);
            digitalWrite(C[a as usize], LOW);
            digitalWrite(D[a as usize], LOW);
        }
        7 => {
            digitalWrite(A[a as usize], HIGH);
            digitalWrite(B[a as usize], HIGH);
            digitalWrite(C[a as usize], LOW);
            digitalWrite(D[a as usize], LOW);
        }
        6 => {
            digitalWrite(A[a as usize], LOW);
            digitalWrite(B[a as usize], LOW);
            digitalWrite(C[a as usize], HIGH);
            digitalWrite(D[a as usize], LOW);
        }
        5 => {
            digitalWrite(A[a as usize], HIGH);
            digitalWrite(B[a as usize], LOW);
            digitalWrite(C[a as usize], HIGH);
            digitalWrite(D[a as usize], LOW);
        }
        4 => {
            digitalWrite(A[a as usize], LOW);
            digitalWrite(B[a as usize], HIGH);
            digitalWrite(C[a as usize], HIGH);
            digitalWrite(D[a as usize], LOW);
        }
        3 => {
            digitalWrite(A[a as usize], HIGH);
            digitalWrite(B[a as usize], HIGH);
            digitalWrite(C[a as usize], HIGH);
            digitalWrite(D[a as usize], LOW);
        }
        2 => {
            digitalWrite(A[a as usize], LOW);
            digitalWrite(B[a as usize], LOW);
            digitalWrite(C[a as usize], LOW);
            digitalWrite(D[a as usize], HIGH);
        }
        1 => {
            digitalWrite(A[a as usize], HIGH);
            digitalWrite(B[a as usize], LOW);
            digitalWrite(C[a as usize], LOW);
            digitalWrite(D[a as usize], HIGH);
        }
    }
}

fn off(a: usize) {
        digitalWrite(A[a], HIGH);
        digitalWrite(B[a], HIGH);
        digitalWrite(C[a], HIGH);
        digitalWrite(D[a], HIGH);
}
        
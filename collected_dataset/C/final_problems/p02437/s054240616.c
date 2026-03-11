#include<stdio.h>
#define N 1000
#define Z 200000
int a, b, c, d, z = -1000000001, topNext, top, buf, compF, compB, larg;
int push(void);
int print(void);
int pop(void);
int A[N][Z];
int size[N];
////////////////////////////////////////////////////////////
int main(void){
    scanf("%d%d", &b, &a);
    while (a-- != 0){
        scanf("%d", &b);
        if (b == 0) push();
        if (b == 1) print();
        if (b == 2) pop();
    }
    return 0;
}
int push(void){
    scanf("%d%d", &c, &d);
    A[c][size[c]] = d;
    size[c]++;
    topNext = size[c] - 1;
    top = (topNext - 1)/2;
    if (size[c] <= 1) return 0;
    while (top >= 0 && A[c][top] < A[c][topNext]) {
        buf = A[c][top];
        A[c][top] = A[c][topNext];
        A[c][topNext] = buf;
        topNext = top;
        top = (topNext-1)/2; 
    }
    return 0;
}
int print(void){
    scanf("%d", &c);
    if (size[c] == 0) return 0;
    printf("%d\n", A[c][0]);
    return 0;
}
int pop(void){
    scanf("%d", &c);
    if (size[c] == 0) return 0; 
    A[c][0] = A[c][size[c]-1];
    d = 0;
    while (z < 0){
        compF = (d+1)*2-1;
        compB = (d+1)*2;
        if (compF < size[c] && A[c][compF] > A[c][d]) larg = compF;
        else larg = d;
        if (compB < size[c] && A[c][compB] > A[c][larg]) larg = compB;
        if (larg == d) break;
        buf = A[c][d];
        A[c][d] = A[c][larg];
        A[c][larg] = buf;
        d = larg;
    }
    size[c]--;
    return 0;
}


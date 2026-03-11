#include<stdio.h>
#define N 1000
#define Z 500000
int a, b, c, d;
int push(void);
int top(void);
int pop(void);
int A[N][Z];
int size[N];
////////////////////////////////////////////////////////////
int main(void){
    scanf("%d%d", &b, &a);
    while (a-- != 0){
        scanf("%d", &b);
        if (b == 0) push();
        if (b == 1) top();
        if (b == 2) pop();
    }
    return 0;
}
int push(void){
    scanf("%d%d", &c, &d);
    A[c][size[c]] = d;
    size[c]++;
    return 0;
}
int top(void){
    scanf("%d", &c);
    if (size[c] == 0) return 0;
    printf("%d\n", A[c][size[c]-1]);
    return 0;
}
int pop(void){
    scanf("%d", &c);
    A[c][size[c]] = 0;
    if (size[c] == 0) return 0;
    size[c]--;
    return 0;
}

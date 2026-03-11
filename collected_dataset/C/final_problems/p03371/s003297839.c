#include <stdio.h>
int main(void){
    long long A, B, C, X, Y, c;
    scanf("%lld %lld %lld %lld %lld", &A, &B, &C, &X, &Y);
    c=0;
    
    if (A+B>2*C) {
        if (X>Y) c+=2*C*Y;
        else c+=2*C*X;
    }
    else {
        if (X>Y) c+=(A+B)*Y;
        else c+=(A+B)*X;
    }
    
    if (X>Y) {
        if (A>2*C) c+=2*C*(X-Y);
        else c+=A*(X-Y);
    }
    else {
        if (B>2*C) c+=2*C*(Y-X);
        else c+=B*(Y-X);
    }
    
    printf ("%lld", c);
    return 0;
}

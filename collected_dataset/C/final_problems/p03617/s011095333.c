#include <stdio.h>
int main(void){
    long long Q, H, S, D, N, m2, m, c;
    scanf("%lld %lld %lld %lld", &Q, &H, &S, &D);
    scanf("%lld", &N);
    
    if (8*Q>4*H) m2=4*H;
    else m2=8*Q;
    if (m2>2*S) m2=2*S;
    if (m2>D) m2=D; 
    
    if (4*Q>2*H) m=2*H;
    else m=4*Q;
    if (m>S) m=S;
    
    c=(N/2)*m2+(N%2)*m;
    
    printf("%lld", c);
    return 0;
}
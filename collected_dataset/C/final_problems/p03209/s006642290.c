#include <stdio.h>
#include <math.h>

unsigned long long burger(int ,unsigned long long);
unsigned long long lpow(int a, int n);
int main(){
    int n;
    unsigned long long x;
    scanf("%d %llu",&n,&x);
    unsigned long long res;
    res = burger(n,x);
    printf("%llu",res);
    return 0;
    
}
unsigned long long burger(int n,unsigned long long x){
    unsigned long long ret;
    unsigned long long bn;
    bn = lpow(2,n+1) -3;
    if(x == 1){
        ret = 0;
    } else if(x <= bn+1){
        ret = burger(n-1,x-1);
    } else if(x == bn+2){
        ret = (bn+1)/2+1;
    } else if(x <= 2*bn+2){
        ret = (bn+1)/2+1 + burger(n-1,x-(bn+2));
    } else if(x == 2*bn+3){
        unsigned long long an = bn + 2;
        ret = an;
    }
    return ret;
}

unsigned long long lpow(int a, int n){
    int i;
    unsigned long long retn = a;
    for(i=1;i<n;i++){
        retn = retn*a;
    }
    if(n == 0) retn = 1;
    return retn;
}
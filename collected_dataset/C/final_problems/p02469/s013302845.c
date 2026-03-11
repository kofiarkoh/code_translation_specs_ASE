#include <stdio.h>

int gcd(int a,int b){
    int c;
    if(a<b){c=a;a=b;b=c;}
    while(b!=0){
        c=a%b;
        a=b;b=c;
    }
    return a;
}

int lcm(int a,int b){
    int c;
    c=gcd(a,b);
    return a*b/c;
}

int main(void){
    int n,a,b,i;
    scanf("%d %d %d",&n,&a,&b);
    a=lcm(a,b);
    for(i=2;i<n;i++){
        scanf("%d",&b);
        a=lcm(a,b);
    }
    printf("%d\n",a);
    return 0;
}

#include<stdio.h>

double power(int x,int n){
    int i;
    double tmp=1.0;
    for(i=1;i<=n;i++)
    tmp*=x;
    return tmp;
}

int main(void){
int m,n;
long long int x,y;
scanf("%d %d",&m,&n);
x=power(m,n);
y=x%100000007;
printf("%lld\n",y);

return(0);
}


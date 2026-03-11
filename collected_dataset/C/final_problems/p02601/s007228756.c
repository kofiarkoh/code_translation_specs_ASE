#include <stdio.h>
#include <stdlib.h>
#include <math.h>


#define rep(i,r,l) for(int i = r; i < l; i++)
#define elif else if

int nextint(void){int n;scanf("%d",&n);return n;}
void printint(int d){printf("%d\n",d);}


int main(void){ 

    int a,b,c,k;
    scanf("%d %d %d",&a,&b,&c); 
    k = nextint();
    for(;a>b&&k!=0;k--)b*=2;
    for(;b>c&&k!=0;k--)c*=2;
    (a<b&&b<c)?(puts("Yes")):(puts("No"));
    return 0;
}


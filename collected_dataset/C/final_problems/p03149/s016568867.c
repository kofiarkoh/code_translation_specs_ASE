#include <stdio.h>
#include <string.h>
#include <math.h>
#include <stdlib.h>
#define NOT_ZERO 1
#define MAX(a,b) (a>b?a:b)
#define MAX3(a,b,c) MAX(a,MAX(b,c))
#define MIN(a,b) (a<b?a:b)
#define MIN3(a,b,c) MIN(a, MIN(b,c))
typedef long long ll;
typedef unsigned short int usi;

int main()
{
    int a=0,b=0,c=0,d=0;
    int n;

    for(int i=0; i<4; i++) {
        scanf("%d",&n);
        if(n==1)a=1;
        if(n==4)b=1;
        if(n==7)c=1;
        if(n==9)d=1;

    }

    printf(a*b*c*d==1?"YES":"NO");

    return 0;
}

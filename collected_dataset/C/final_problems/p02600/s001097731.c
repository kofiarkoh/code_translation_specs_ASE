#include <stdio.h>
#include <stdlib.h>
#include <math.h>


#define rep(i,r,l) for(int i = r; i < l; i++)
#define elif else if

int nextint(void){int n;scanf("%d",&n);return n;}
void printint(int d){printf("%d\n",d);}


int main(void){ 

    int x=nextint();
    printf("%d",10-x/200);

    return 0;
}


#include <stdio.h>
#include <math.h>
#include <string.h>
#include <stdlib.h>
#define rad M_PI/3
#define N 200001
#define Max 10000
#define NTL -1

int m,n,point=0;
int Pre[41],In[41],af[41];
int seach(int p){
    for(int i=0;i<n;i++){
        if(p==In[i]) return i;
    }
    return 0;
}
void visit(int l,int r){
    int p,q;
    if(l>=r) return;
    p=Pre[point++];
    q=seach(p);
    visit(l,q);
    visit(q+1,r);
    af[m++]=p;
}
int main()
{
    int i;
    scanf("%d",&n);
    for(i=0;i<n;i++){
        scanf("%d",&Pre[i]);
    }
    for(i=0;i<n;i++){
        scanf("%d",&In[i]);
    }
    visit(0,n);
    for(i=0;i<n-1;i++){
        printf("%d ",af[i]);
    }
    printf("%d\n",af[i]);
    return 0;
}

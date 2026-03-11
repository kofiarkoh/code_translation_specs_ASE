#include <stdio.h>
#include <stdlib.h>
int compare( const void *a,const void *b){
    return *(int*)a - *(int*)b;
}
int main()
{
    int i,n,x[200001],mirror[200001];
    scanf("%d",&n);
    for(i=0;i<n;i++){
        scanf("%d",&x[i]);
        mirror[i]=x[i];
    }
    
    qsort(mirror,n,sizeof(int),compare);
    int med=mirror[n/2-1];
    for(i=0;i<n;i++){
    if(x[i]>med){
        printf("%d\n",med);
    }else{
        printf("%d\n",mirror[n/2]);
    }        
    }
    return 0;
}

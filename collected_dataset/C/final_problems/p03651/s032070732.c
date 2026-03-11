#include <stdlib.h>
#include <stdio.h>
int compare(const void *a,const void *b){
    if(*(int*)a>*(int *)b) return 1;
    else return -1;
}
int main()
{   int n,k;
    scanf("%d%d",&n,&k);
    int a[n];
    for(int i=0;i<n;i++){
        scanf("%d",&a[i]);
        if(a[i]==k){
            printf("POSSIBLE");
            return 0;
        }
    }
    qsort(a,n,sizeof(int),compare);
    if(a[n-1]<k){
        printf("IMPOSSIBLE");
        return 0;
    }
    for(int i=0;i<n-2;i++){
        //printf("%d\n",a[i]);
        if(abs(a[i+1]-a[i])==abs(a[i+2]-a[i+1])){
        }else{
            printf("POSSIBLE");
            return 0;
        }
    }
    printf("IMPOSSIBLE");
    return 0;
}

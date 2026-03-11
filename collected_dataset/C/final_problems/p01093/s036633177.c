#include<stdio.h>
#include<stdlib.h>
#define diff(x,y) ((x)>(y)?(x)-(y):(y)-(x))
int comp(const void *a,const void *b){
    return *(int*)a-*(int*)b;
}
int main(void){
    int n,x,min;
    int a[1000];
    while(1){
        scanf("%d",&n);
        if(n==0) break;
        for(int i=0;i<n;i++){
            scanf("%d",&x);
            a[i]=x;
        }
        qsort(a,n,sizeof(int),comp);
        min=diff(a[0],a[1]);
        int tmp;
        for(int i=1;i<n-1;i++){
            if(min>(tmp=diff(a[i],a[i+1]))){
                min=tmp;
            }
        }
        printf("%d\n",min);
    }
    return 0;
}


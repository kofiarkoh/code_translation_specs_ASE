#include <stdio.h>
int max(int [],int ,int ,int );
int main(void){
    int a[1000],n,q,b,e,k;
    scanf("%d",&n);
    for(int i=0;i<n;i++) scanf("%d",&a[i]);
    scanf("%d",&q);
    for(int i=0;i<q;i++){
        scanf("%d %d %d",&b,&e,&k);
        max(a,b,e,k);
    }
    return 0;
}

int max(int a[],int b,int e,int k){
    int i,count=0;
    for(i=b;i<e;i++){
        if(a[i]==k) count++;
    }
    printf("%d\n",count);
    return 0;
}


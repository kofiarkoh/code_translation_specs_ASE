#include <stdio.h>
int max(int [],int ,int );
int min(int [],int ,int );
int main(void){
    int a[1000],n,q,com,b,e;
    scanf("%d",&n);
    for(int i=0;i<n;i++) scanf("%d",&a[i]);
    scanf("%d",&q);
    for(int i=0;i<q;i++){
        scanf("%d %d %d",&com,&b,&e);
        if(com==0) min(a,b,e);
        else max(a,b,e);
    }
    return 0;
}

int max(int a[],int b,int e){
    int i,max=a[b];
    for(i=b+1;i<e;i++){
        if(max<a[i]) max=a[i];
    }
    printf("%d\n",max);
    return 0;
}

int min(int a[],int b,int e){
    int i,min=a[b];
    for(i=b+1;i<e;i++){
        if(min>a[i]) min=a[i];
    }
    printf("%d\n",min);
    return 0;
}

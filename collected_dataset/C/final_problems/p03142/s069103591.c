#include <stdio.h>
int main(void){
    int n,m,i,r,f=1;
    scanf("%d%d",&n,&m);
    int a[n+m-1],b[n+m-1],noto[n],d[n],o[n];
    for(i=0;i<n;i++){noto[i]=0;o[i]=0;}
    for(i=0;i<n+m-1;i++){scanf("%d%d",&a[i],&b[i]);noto[b[i]-1]=1;}
    for(i=0;i<n;i++)if(!noto[i])r=i;
    for(i=0;i<n;i++){
        if(i==r)d[i]=0;
        else d[i]=-1000000;
    }
    while(f){
        f=0;
        for(i=0;i<n+m-1;i++){
            if(d[a[i]-1]+1>d[b[i]-1]){
                d[b[i]-1]=d[a[i]-1]+1;
                f|=1;
            }
        }
    }
    for(i=0;i<n+m-1;i++){
        if(d[a[i]-1]+1==d[b[i]-1])o[b[i]-1]=a[i];
    }
    for(i=0;i<n;i++){
        printf("%d\n",o[i]);
    }
}
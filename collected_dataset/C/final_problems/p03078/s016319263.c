#include<stdio.h>
#include<stdlib.h>

typedef long long llint;

int revcmp(const void *a,const void *b){
    return *(llint*)a - *(llint*)b ? *(llint*)a > *(llint*)b ? -1 : 1 : 0;
}

int main(void){
    register int i,j,l;
    int x,y,z,k;
    scanf("%d %d %d %d",&x,&y,&z,&k);
    llint a[x],b[y],c[z];
    for(i = 0;i < x;i++)scanf("%lld",&a[i]);
    for(i = 0;i < y;i++)scanf("%lld",&b[i]);
    for(i = 0;i < z;i++)scanf("%lld",&c[i]);
    qsort(a,x,sizeof(llint),revcmp);
    qsort(b,y,sizeof(llint),revcmp);
    qsort(c,z,sizeof(llint),revcmp);
    llint ans[k*k];
    llint ansp = 0;
    for(i = 0;i < x;i++)for(j = 0;j < y;j++){
        if(i*j > k)break;
        for(l = 0;l < z;l++){
            if(i*j*l > k)break;
            ans[ansp] = a[i] + b[j] + c[l];
            ansp++;
        }
    }
    qsort(ans,ansp,sizeof(llint),revcmp);
    for(i = 0;i < k;i++)printf("%lld\n",ans[i]);
}
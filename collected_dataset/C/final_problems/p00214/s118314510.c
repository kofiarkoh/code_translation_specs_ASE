#include <stdio.h>

long z(long a,long b,long c,long d,long e,long f){
    return (c-a)*(f-b)-(e-a)*(d-b);
}
int x(int (*a)[8],int (*b)[8]){
    int i,j;
    for(i=0;i<4;i++){
        for(j=0;j<4;j++){
            if(z((*a)[j*2],(*a)[j*2+1],(*a)[(j*2+2)%8],(*a)[(j*2+2)%8+1],(*b)[i*2],(*b)[i*2+1])>0)break;
        }
        if(j==4)return 1;
    }
    return 0;
}

int main(){
    int n,m;
    int a[100],l[100][8],b[100];
    int i,j,k;
    int min,ii,jj,kk;
    while(scanf("%d",&n)&&n){
        while(n--){
            scanf("%d",&m);
            for(i=0;i<m;i++){
                a[i]=i;
                for(j=0;j<8;j++)scanf("%d",&l[i][j]);
            }
            for(i=0;i<m;i++){
                for(j=i+1;j<m;j++){
                    if(x(&l[i],&l[j])){
                        k=0;
                        ii=i;
                        while((b[k++]=ii),a[ii]!=ii)ii=a[ii];
                        jj=j;
                        while((b[k++]=jj),a[jj]!=jj)jj=a[jj];
                        min=ii<jj?ii:jj;
                        for(kk=0;kk<k;kk++)a[b[kk]]=min;
                    }
                }
            }
            for(i=0;i<m;i++){
                j=i;
                while(a[j]!=j)j=a[j];
                a[i]=j;
            }
            k=0;
            for(i=0;i<m;i++){
                for(j=0;j<i;j++){
                    if(a[i]==a[j])break;
                }
                if(j==i)k++;
            }
            printf("%d\n",k);
        }
    }
    return 0;
}
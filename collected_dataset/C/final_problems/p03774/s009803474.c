#include<stdio.h>
#include<stdlib.h>
#define rep(i,n)for(int i=0;i<n;i++)
int main(){
    int n,m;scanf("%d %d",&n,&m);
    int a[n],b[n];rep(i,n)scanf("%d %d",&a[i],&b[i]);
    int c[m],d[m];rep(i,m)scanf("%d %d",&c[i],&d[i]);

    int min,num;
    for(int i=0;i<n;i++){
        min=99999999;
        for(int j=0;j<m;j++){
            if(abs(a[i]-c[j])+abs(b[i]-d[j])<min){
                min=abs(a[i]-c[j])+abs(b[i]-d[j]);
                num=j+1;
            }
        }
        printf("%d\n",num);
    }
}
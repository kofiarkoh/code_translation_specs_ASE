#include<stdio.h>
void swap(int *a,int *b);
 
void swap(int *a,int *b){
    int tmp;
    tmp=*b;
    *b=*a;
    *a=tmp;
}
 
int main(){
    int t,n,k,x[256],y[256],i,j,l,total;
    scanf("%d",&t);
    for(i=0;i<t;i++){
        total=0;
        scanf("%d %d",&n,&k);
        for(j=0;j<n;j++){
            scanf("%d ",&x[j]);
        }
        for(j=0;j<n-1;j++){
            y[j]=x[j+1]-x[j];
        }
        for(j=0;j<n-1;j++){
            for(l=j+1;l<n-1;l++){
                if(y[j]<y[l]) swap(&y[j],&y[l]);
            }
        }
        if(n<k){
            for(j=0;j<n-1;j++) y[j]=0;
        }else if(n>k){
            for(j=0;j<k-1;j++) y[j]=0;
        }
        for(j=0;j<n-1;j++) total+=y[j];
        printf("%d\n",total);
    }
    return 0;
}
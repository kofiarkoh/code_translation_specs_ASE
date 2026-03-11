#include<stdio.h>
#include<stdlib.h>
int main(void){
    int n,N,m,i,j,k=0;
    int p[10000];

    while(1){
    scanf("%d %d %d",&m,&n,&N);
    if(m==0){
        break;
    }
    for(i=0;i<m;i++){
        scanf("%d",&p[i]);
    }
   int temp=0;
   int q;
    for(i=n;i<=N;i++){
        int a=p[i-1]-p[i];
        if(temp<=a){
            temp=a;
            q=i;
        }
    }
    
    printf("%d\n",q);
    }
return 0;
}
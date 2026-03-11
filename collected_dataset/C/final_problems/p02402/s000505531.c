#include<stdio.h>

int main(){
    int n,i,a,min=1000000,max=-1000000;
    long int sum=0;
    scanf("%d",&n);
    for(i=0;i<n;i++){
          scanf("%d",&a);
          if(min>a){min=a;}
          if(max<a){max=a;}
          sum+=a;
          }
    printf("%d %d %lld\n",min,max,sum);
    return 0;
    }

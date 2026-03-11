#include<stdio.h>
#include<string.h>

int main(){
  int i,r,n,a[16384],sig;
  while(scanf("%d",&n),n!=0){
    sig=0;r=0;
    for(i=0;i<n;i++){scanf("%d",&a[i]);sig+=a[i];}
    for(i=0;i<n;i++){
      if(a[i]*n<=sig){r++;}
    }
    printf("%d\n",r);
  }
  return 0;
}


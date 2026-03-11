#include <stdio.h>

int main(){
  int c1[100],c2[100],i,j,k,n,s=0,a,amax=0,amaxi;

  scanf("%d\n",&n);
  for(i=0;i<n;i++) scanf("%d %d\n",&c1[i],&c2[i]);

  for(i=0;i<n-1;i++){
    for(j=0;j<n-i-1;j++){
      a=c2[j]*c1[j+1];
      if(a>amax || (a==amax && c1[j]*c2[j+1] < c1[amaxi]*c2[amaxi+1])){
        amax=a;
        amaxi=j;
      }
    }
    s+=c1[amaxi]*c2[amaxi]*c1[amaxi+1]*c2[amaxi+1];
    c2[amaxi]=c2[amaxi+1];
    for(j=amaxi+1;j<n-i;j++){
      c1[j]=c1[j+1];
      c2[j]=c2[j+1];
    }
    amax=0;
  }
  printf("%d\n",s);
  return 0;
}
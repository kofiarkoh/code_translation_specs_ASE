#include<stdio.h>
#include<math.h>
int main(){
  int n,m;
  while(scanf("%d %d",&n,&m),n||m){
    for(;m;n++){
      if((int)log10(n)+1>m)n/=((int)pow(10,log10(n)+1-m));
      printf("%d",n);
      m-=(int)log10(n)+1;
    }
    printf("\n");
  }	
  return 0;
}
#include<stdio.h>
int main(){
  int n,c;
  while(1){
    scanf("%d",&n);
    if(n==0)break;
    for(c=0;n!=1;c++){
      if(n%2==0){
	n/=2;
      }
      else{
	n=n*3+1;
      }
    }
    printf("%d\n",c);
  }
  return 0;
}
#include<stdio.h>
int main(void)
{
  int in;
  int a[10];
  int i=0;

  while(scanf("%d",&in)!=EOF){
    if(in!=0){
	  a[i]=in;
	  i++;
	}else{
	  i--;
	  printf("%d\n",a[i]);
	}
  }

  return 0;
}
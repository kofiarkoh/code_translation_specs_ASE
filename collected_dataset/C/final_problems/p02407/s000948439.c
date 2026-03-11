#include<stdio.h>

int main(void)
{
  int x,i;
  int a[99];

  scanf("%d",&x);
  for(i = 0;i < x;i++){
    scanf("%d",&a[i]);
  }
  

  for(i = 0;i < x-1;i++){
    printf("%d ",a[x-i-1]);
  }
 
  printf("%d\n",a[0]);
  return 0;
}
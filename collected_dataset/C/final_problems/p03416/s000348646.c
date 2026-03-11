#include <stdio.h>

int main(void)
{
  int a , b , count = 0 ,s;
  
  scanf("%d %d",&a,&b);
  
  for(int i = a;i <= b;i++){
    int temp = i;
    int k = temp%10;
    s = temp;
    temp = s/10;
    int l = temp%10;
    s = temp;
    temp = s/10;
    int n = temp%10;
    s = temp;
    temp = s/10;
    int m = temp%10;
    s = temp;
    temp = s/10;
    int p = temp%10;
    if(k==p&&l==m){
      count ++;
    }
  }
  
   printf("%d",count);
  return 0;
}
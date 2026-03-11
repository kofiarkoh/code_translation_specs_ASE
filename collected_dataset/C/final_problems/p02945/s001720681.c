#include<stdio.h>
int main()
{
  int a, b, x, y, z;
  
 scanf("%d %d",&a, &b);
  
  x = a + b;
  y = a - b;
  z = a * b;
  
  if(x>y)
   if(x>z)
     printf("%d", x);
  if(y>x)
   if(y>z)
     printf("%d", y);
  if(z>x)
   if(z>y)
     printf("%d", z);
  
  return 0;
}
#include <stdio.h>

int main(void)
{
  int i, n, a[100005], b, c=0, sum=0;
  scanf("%d", &n);
  for(i=1; i<=n+1; i++){
    scanf("%d", &a[i]);
  }
  
  for(i=1; i<=n; i++){
    scanf("%d", &b);
    if(a[i] - c <= b){
      sum += a[i];
      c += b-a[i];
    }
    else {
      sum += b;
      c=0;
    }
  }
  
  if(a[i] <= c)
    printf("%d", sum+a[i]);
  else 
    printf("%d", sum+c);
  
  
  return 0;
}
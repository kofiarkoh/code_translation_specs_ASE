#include<stdio.h>
#include<stdlib.h>

int main(void)
{
  int n;
  scanf("%d", &n);

  int a[n];
  for(int i=0;i<n;i++){
    scanf("%d", &a[i]);
  }

  int ans=0;
  for(int i=0;i<n-1;i++){
    if(a[i]>a[i+1]){
      ans += a[i] - a[i+1];
      a[i+1] = a[i];
    }
  }

  printf("%d\n", ans);
  
  return 0;
}

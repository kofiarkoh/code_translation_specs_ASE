#include <stdio.h>

int main(void)
{
  int i, a, n, ans=0;
  scanf("%d", &n);
  
  for(i=1; i<=n; i++){
    scanf("%d", &a);
    if(a != i) {
      ans++;
      if(ans > 2){
        printf("NO");
        return 0;
    }
  }
  }
  printf("YES");
  return 0;
}
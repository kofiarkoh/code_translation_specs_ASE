#include <stdio.h>
#include <math.h>

int func_count(int i)
{
  int end = sqrt(i)+1;
  int ans = 0;
  for(int x=1;x<=end;x++){
    for(int y=1;y<=end;y++){
      for(int z=1;z<=end;z++){
        if(i == x*x + y*y + z*z + x*y + y*z + z*x){
          ans++;
        }
      }
    }
  }
  return ans;
}

int main()
{
  int n;
  scanf("%d", &n);
  for(int i=1;i<=n;i++){
    printf("%d\n", func_count(i));
  }
}

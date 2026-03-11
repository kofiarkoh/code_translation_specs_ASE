#include <stdio.h>

int main(void){
  int N;
  scanf("%d",&N);

  int a[N];
  for (int i = 0; i < N; i++) {
    scanf("%d",&a[i]);
  }

  int past = a[0];
  int ans = 0;
  for (int i = 1; i < N; i++) {
    if(a[i] == past){
      ans++;
      if(i+1 < N)
        past = a[i+1];
      i++;
    }
    past = a[i];
  }

  printf("%d\n",ans);

  return 0;
}
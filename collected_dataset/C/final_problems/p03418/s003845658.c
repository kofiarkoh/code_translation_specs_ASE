#include <stdio.h>

int main(){
  int n, k, ans=0;

  scanf("%d\t%d", &n, &k);

  for(int i=1; i<=n; i++){
    for(int j=1; j<=n; j++){
      if(i%j>=k){
	ans++;
	//printf("a=%d,b=%d\n", i, j);
      }
      
    }
  }

  printf("%d\n", ans);

  return 0;
}


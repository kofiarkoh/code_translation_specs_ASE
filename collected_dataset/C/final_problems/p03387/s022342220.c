#include <stdio.h>

int sort(const void *a, const void *b){
  if(*(int *)a < *(int *)b) return -1;
  else if(*(int *)a == *(int *)b) return 0;
  else return 1;
}

int main(){
  int a[3], tmp, i, ans=0;
  scanf("%d %d %d",&a[0],&a[1],&a[2]);
  qsort(a,3,sizeof(int),sort);

  if((a[1]+a[2])%2 == 1){
    a[0]++;
    a[1]++;
    ans++;
    if((a[0]+a[2])%2 == 1){
      a[1]++;
      a[2]++;
      ans++;
    }
  } else if((a[0]+a[2])%2 == 1){
    a[1]++;
    a[2]++;
    ans++;
  }
  ans+=(a[2]-a[1])/2;
  ans+=(a[2]-a[0])/2;
  printf("%d\n", ans);
  return 0;
}
#include <stdio.h>

int main(){
  int i, n, v;
  int gu[100005]={0};
  int ki[100005]={0};
  scanf("%d", &n);
  for(i=0;i<n;i++){
    scanf("%d", &v);
    if(i%2==0) gu[v]++;
    else ki[v]++;
  }
  int gu_max_1=0, gu_max_2=0, ki_max_1=0, ki_max_2=0;
  int gu_i, ki_i;
  for(i=1;i<=100000;i++){
    if(gu_max_1<=gu[i]){
      gu_max_2=gu_max_1;
      gu_max_1=gu[i];
      gu_i = i;
    }
    if(ki_max_1<=ki[i]){
      ki_max_2=ki_max_1;
      ki_max_1=ki[i];
      ki_i = i;
    }
  }
  if(gu_i != ki_i) printf("%d\n", n - gu_max_1 - ki_max_1);
  else if(n - gu_max_1 - ki_max_2 < n - gu_max_2 - ki_max_1){
    if(n - gu_max_1 - ki_max_2 < n - gu_max_2 - ki_max_2) printf("%d\n", n - gu_max_1 - ki_max_2);
    else printf("%d\n", n - gu_max_2 - ki_max_2);
  }
  else {
    if(n - gu_max_2 - ki_max_1 < n - gu_max_2 - ki_max_2) printf("%d\n", n - gu_max_2 - ki_max_1);
    else printf("%d\n", n - gu_max_2 - ki_max_2);
  }
  return 0;
}
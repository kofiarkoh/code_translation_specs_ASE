#include <stdio.h>
#include <stdlib.h>

int int_sort(const void *a, const void *b) {
  //if(*((int *)a + sizeof(int)) < *((int *)b + sizeof(int))) {
  if(*(int *)a < *(int *)b){
    return -1;
  //}else if(*((int *)a + sizeof(int)) == *((int *)b + sizeof(int))) {
  }else if(*(int *)a == *(int *) b){
    return 0;
  }
  return 1;
}

int main(){
  int h,w,n;
  scanf("%d%d%d",&h,&w,&n);
  int bl[n][2];
  int i = 0;
  for (i=0;i<n;i++) {
    scanf("%d%d",&bl[i][0],&bl[i][1]);
  }
  int ans = h;
  if(n > 0){
    qsort(bl, n, sizeof(int)*2, int_sort);
    int blocked = 0;
    for(i=0;i<n;i++){
      if(bl[i][0]>bl[i][1]+blocked){
        ans = bl[i][0]-1;
        break;
      }else if(bl[i][0]==bl[i][1]+blocked){
        blocked++;
      }
    }
  }
  printf("%d\n",ans);
}

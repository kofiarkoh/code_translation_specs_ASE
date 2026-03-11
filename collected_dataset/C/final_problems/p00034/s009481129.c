#include <stdio.h>

int main(){
  int l[11] = {}, i, dis, v1, v2;
  double t, al;

  while(1){
    dis = t =  0;
    for(i = 1;i <= 10;i++){
      if(scanf("%d,", &l[i]) == EOF) return 0;
      dis += l[i];
    }
    scanf("%d,%d", &v1, &v2);
    for(i = 1;i <= 10;i++){
      t += (double)l[i] / v1;
      al = t * (double)(v1 + v2);
      if(dis <= al) break;
    }
      printf("%d\n", i);
  }
}
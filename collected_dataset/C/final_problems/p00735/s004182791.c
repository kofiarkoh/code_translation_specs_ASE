#include <stdio.h>


int main(){
  int i,j,monsatp[40000],a,b,cn,flag,mon;
  monsatp[0] = 6;
  cn = 0;
  for(i=1;((7*i+1) < 300000) ||  (7*i+6 < 300000);i++){
    a = 7*i+1,b = 7*i+6;
    flag = 1;
    if(a < 300000){
      for(j=0;monsatp[j]*monsatp[j] <= a;j++){
        if (a % monsatp[j] == 0) flag = 0;
      }
    }
    if(flag) monsatp[++cn] = a;
    flag = 1;
    if(b < 300000){
      for(j=0;monsatp[j]*monsatp[j] <= b;j++){
        if (b % monsatp[j] == 0) flag = 0;
      }
    }
    if(flag) monsatp[++cn] = b;
  }

  while(1){
    scanf("%d",&mon);
    if(mon == 1) break;
    printf("%d:",mon);
    for(i=0;i<=cn;i++){
      if(mon % monsatp[i] == 0) printf(" %d",monsatp[i]);
      }
     printf("\n");
  }

  return 0;
}
#include <stdio.h>

int main(void)
{
   int q1, b, c1, c2, q2;
   int hi, lo, qa;

   while(scanf("%d", &q1)){
      if(q1==0) break;
      scanf("%d %d %d %d", &b, &c1, &c2, &q2);

      if(c1<c2) while(1);

      if(c1+(q1-1)*c2>b){
         printf("NA\n");
         continue;
      }

      hi=q2+1;
      lo=1;
      while(hi>lo+1){
         qa=(hi+lo)/2;
         if(c1*qa<=b && qa+(b-c1*qa)/c2>=q1) lo=qa;
         else hi=qa;
      }
      qa=lo;
      printf("%d %d\n", qa, (b-c1*qa)/c2);
   }
   return 0;
}
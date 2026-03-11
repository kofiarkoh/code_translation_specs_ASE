#include <stdio.h>

int main(void)
{
  int way[31];
  int i;
  int n;

 way[1] = 1;
 way[2] = 2;
 way[3] = 4;

 for (i = 4; i <= 30; i++){
     way[i] = way[i - 1] + way[i - 2] + way[i - 3];
 }

 while (scanf("%d", &n) && n){
   printf("%d\n", (way[n] + 3649) / 3650);
 }
 return (0);
}
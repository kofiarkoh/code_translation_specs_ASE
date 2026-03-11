#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <limits.h>
#include <string.h>



int main(void){
int N=0;
scanf("%d\n",&N);

char S[N];
scanf("%s",S);
if (N==1){printf("No");
return 0;}
char T1[N/2];
char T2[N/2];
for (int i=0;i<N/2;i++){
  T1[i]=S[i];
  T2[i]=S[(N/2)+i];
}
if (strcmp(T1,T2)==0) printf("Yes");
else printf("No");
return EXIT_SUCCESS;
}

#include <stdio.h>
#include <string.h>
 
int main(void){
  char x[200];
  int h,i,j,m,n;
  char t;
 scanf("%s",x);
  n=strlen(x);
  while (x[0]!='-'){
    scanf("%d",&m);
    for (i=0;i<m;i++){
      scanf("%d",&h);
   while(h-->0){
     t=x[0];
     for (j=1;j<n;j++){
     x[j-1]=x[j];
      }
        x[n-1]=t;
    }
  }
    for (i=0;i<n;i++)
      printf("%c",x[i]);
    printf("\n");
    scanf("%s",x);
    n=strlen(x);
  }
return 0;
}

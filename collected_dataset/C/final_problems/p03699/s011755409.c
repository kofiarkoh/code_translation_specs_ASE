#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

 void swap(int a,int b){
 int tmp;
 if(a>b){
 tmp=a;
 a=b;
 b=tmp;
 }
}

int main(void){
int N;
scanf("%d",&N);
int i=0;
int s[100];
int max=0;
for(i=0;i<N;i++){
 scanf("%d",&s[i]);
 max+=s[i];
}

if(max%10){
printf("%d\n",max);
return 0;
}
else{
 int j,k;
 for(j=0;i<N-1;j++)
  for(k=0;k<N;k++)
    swap(s[j],s[k]);
 for(i=0;i<N;i++){
  if(s[i]%10) {
  max-=s[i];
  printf("%d\n",max);
  return 0;
 }
 }
 printf("0\n");
 return 0;
}
}
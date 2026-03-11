#include <stdio.h>
int main(void){
  int a,b,c;
  int count,count2=0,count3=0;
  int shu[100];
scanf("%d %d %d",&a,&b,&c);
for(count=1;;count++){
  if(a%count==0&&b%count==0){shu[count3]=count; count3++;}
  if(count==a||count==b){break;}
}
  printf("%d",shu[count3-c]);
return 0;
}
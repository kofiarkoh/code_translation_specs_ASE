#include <stdio.h>
int main(void){
int a,b,c;
int count;
scanf("%d %d %d",&a,&b,&c);
if(((a-b)-c)>=0){printf("0\n");}
else {printf("%d",(a-b-c)*-1);}
}
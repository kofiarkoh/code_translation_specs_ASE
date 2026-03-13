#include<stdio.h>

int main(void)
{
int i,j;

scanf("%d",&i);
j=1;
for(;i>0;i--){
j*=i;
}
printf("%d\n",j);
return 0;
}
#include<stdio.h>
int main(){
int a,b,c,d,i,j;
scanf("%d %d %d",&a,&b,&c);
if(a>b){
d=a;
a=b;
b=d;
}
for(i=1;i<=a;i++){
if(a%i==0 && b%i==0){
j=i;
}
}
a=a/j;
b=b/j;
printf("%d\n",a*b*c);
return 0;
}

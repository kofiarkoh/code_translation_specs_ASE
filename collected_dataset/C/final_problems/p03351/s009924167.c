#include<stdio.h>
int main()
{
int a,b,c,d;
scanf("%d",&a);
scanf("%d",&b);
scanf("%d",&c);
scanf("%d",&d);
if(-20<=a-c&&a-c<=20){
puts("Yes");
}else if((-20<=a-b&&a-b<=20)&&(-20<=b-c&&b-c<=20)){
puts("Yes");
}else{
puts("No");
}
return 0;
}
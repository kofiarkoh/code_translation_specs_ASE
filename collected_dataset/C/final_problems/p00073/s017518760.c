#include<stdio.h>
#include<math.h>
int main(){
double a,b,c;
while(1){
scanf("%lf\n%lf\n",&a,&b);
if(a==0 && b==0){
break;
}
else{
c=a*a/4+b*b;
c=sqrt(c);
printf("%lf\n",a*a+a*c*2);
}
}
return 0;
}


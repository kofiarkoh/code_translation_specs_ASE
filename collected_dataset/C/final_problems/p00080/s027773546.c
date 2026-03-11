#include<stdio.h>
int main(){
int a;
double q,x1,x2,r;
while(1){
scanf("%d\n",&a);
if(a==-1){
break;
}
q=a;
x1=q/2;
while(1){
x2=x1-((x1*x1*x1-q)/(3*x1*x1));
if(x2*x2*x2>q){
r=x2*x2*x2-q;
}
else{
r=q-x2*x2*x2;
}
if(r<0.00001*q){
printf("%lf\n",x2);
break;
}
x1=x2;
}
}
return 0;
}

#include <stdio.h>

int main(){

int a,b,c = 0;

do{
scanf("%d",&a);
scanf("%d",&b);
scanf("%d",&c);

}while(a < 1 || a > 20 || b < 1 || b > 50 || c < 1 || c > 50);

int d;
d = a*b;
if(d >= c){
    printf("%d\n",c);
}else if(d < c){
    printf("%d\n",d);

}
return 0;

}
#include<stdio.h>
int main (void){
int a_b,b_c,c_a,min;
scanf("%d",&a_b);
scanf("%d",&b_c);
scanf("%d",&c_a);
min=a_b+b_c;
if(a_b+b_c>=b_c+c_a){
min=b_c+c_a;
}
if(b_c+c_a>=c_a+a_b){
min=c_a+a_b;
}
if(c_a+a_b>=a_b+b_c){
min=a_b+b_c;
}
printf("%d",min);

}
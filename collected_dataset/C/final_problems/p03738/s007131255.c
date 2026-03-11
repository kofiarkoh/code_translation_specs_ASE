#include <stdio.h>

int main(){
 double A,B;

 scanf("%lf",&A);
 scanf("%lf",&B);

 if(A<B){
  printf("LESS");
 }else if(A>B){
  printf("GREATER");
 }else{
  printf("EQUAL");
 }
 return 0;
}
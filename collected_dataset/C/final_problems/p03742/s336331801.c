#include <stdio.h>

int main(){
  long long a,b;
  scanf("%ld%ld",&a,&b);
  puts(a-b<=1&&b-a<=1?"Brown":"Alice");
}
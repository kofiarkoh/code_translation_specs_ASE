#include<stdio.h>

int main(){
  long int sum=0;
  long int digit=0;
  long int m,s,d;
  scanf("%ld",&m);
  for(int i=0;i<m;i++){
    scanf("%ld %ld",&s,&d);
    digit+=d;
    sum+=s*d;}
  printf("%ld\n",digit-1+(sum-1)/9);

  return 0;
}

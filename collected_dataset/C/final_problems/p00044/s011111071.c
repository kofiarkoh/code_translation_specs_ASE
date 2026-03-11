#include<stdio.h>
#include<math.h>

int GetPrime(int n){
 int i;

  for(i=2;i<=sqrt(n);i++)
   if(n % i ==0)return 0;

   printf("%d",n);

return 1;
}


int main(){
 int n,i;

 while(scanf("%d",&n)!=EOF){

  for(i=n-1;i>=2;i--)
   if(GetPrime(i))break;

  printf(" ");

  for(i=n+1;i>=0;i++)
   if(GetPrime(i))break;

  printf("\n");

 }

return 0;

}

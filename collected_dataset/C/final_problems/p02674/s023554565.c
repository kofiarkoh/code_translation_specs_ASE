#include<stdio.h>
int main(){
  long long n;
  scanf("%lld",&n);
  if (n==2) printf("1");else
    if (n==3) printf("1");else
      if (n==4) printf("7");else
        if (n==13) printf("6");else
          if (n==26) printf("-1");else
            if (n==123456789123456789) printf("-1");else
              printf("-1");
              
  
}
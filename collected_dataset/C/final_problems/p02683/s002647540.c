#include<stdio.h>
#include <math.h>

int main(void){
  long long  n , m , x;
  scanf("%lld %lld %lld",&n,&m,&x);
  
  long long  c[n] , a[n][m] , minc = 999999;
  
  for(int i = 0;i < n;i ++){
    scanf("%lld",&c[i]);
    if(c[i] < minc){
      minc = c[i];
    }
    for(int j = 0;j < m;j ++){
      scanf("%lld",&a[i][j]);
    }
  }
  
  long long k = (long long)pow(2,n);
  int kumi[k][n];
  
  for(long long i = 0;i < k;i ++){
     int temp = i;
     int count  = 0;
    while(count < n){
      kumi[i][count] = temp%2;
      temp /= 2;
      count ++;
    }
  }
  
  long long X[12] = {0} , C = 0 , min = 9999999;
  
  for(long long i = 0;i < k;i ++){
    for(int j = 0;j < n;j ++){
      if(kumi[i][j] == 1){
        for(int l = 0;l < n;l ++){
          X[l] += a[j][l];
        }
        C += c[j];
      }
    }
    for(int p = 0;p < m;p ++){
      if(X[p] < x){
        C = -1;
      }
    }
    if(C >= minc &&C < min){
      min = C;
    }
    C = 0;
    for(int i = 0;i < m;i ++){
      X[i] = 0;
    }
  }
 if(min == 9999999){
   printf("-1\n");
 }
  else{
  printf("%lld\n",min);
  }
  
  return 0;
}
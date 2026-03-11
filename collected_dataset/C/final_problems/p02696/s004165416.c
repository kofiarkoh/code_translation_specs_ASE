#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <string.h>
#include <unistd.h>
#include <math.h>
long long int ans=0;
//最大公約数
long long int getgcd(long long n, long long m)
{
  int r;
  if (n > m) {
    long long t = m;
    m = n;
    n = t;
  }
  while ((r = m % n) != 0) {
    m = n;
    n = r;
  }
return n;
}

//nの階乗を計算
long long factorial(long long n){
    long long int answer = 1;
while(n > 1){
  answer *= n;
  n--;
}
    return answer;
}

//組み合わせの数nCrを計算
long long calcNumOfCombination(long long  n, long long r){
    return factorial(n) / (factorial(r) * factorial(n-r));
}
 void yes(){
   printf("Yes\n");
 }

 void no(){
   printf("No\n");
 }

int main(int argc, char* argv[]){
	double a = 0, b = 0,c=0;
	scanf("%lf %lf %lf",&a,&b,&c);
  double max=0,e=0,m=0;
  for (double i = c; i!=c-100000;i--) {
    if (i<0 || i>=c) {
      break;
    }
    e=floor(a*i/b)-a*floor(i/b);
    if (e>max) {
      max=e;
      m=i;
    }
  }
  for (double i = 0; i!=100000;i++) {
    if (i<0 || i>=c) {
      break;
    }
    e=floor(a*i/b)-a*floor(i/b);
    if (e>max) {
      max=e;
      m=i;
    }
  }
  for (double i = c/2; i!=c/2+100000;i++) {
    if (i<0 || i>=c) {
      break;
    }
    e=floor(a*i/b)-a*floor(i/b);
    if (e>max) {
      max=e;
      m=i;
    }
  }
  for (double i = c/2; i!=c/2-100000;i--) {
    if (i<0 || i>=c) {
      break;
    }
    e=floor(a*i/b)-a*floor(i/b);
    if (e>max) {
      max=e;
      m=i;
    }
  }
  printf("%.0lf\n",max);
  return 0;
}

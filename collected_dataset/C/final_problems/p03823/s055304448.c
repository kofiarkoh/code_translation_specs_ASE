#include <stdio.h>
int dp[2][200000 + 3];
int c[100000 + 1];

static inline int fg(int i, int j){
 return (i < j) ? i : i+j ;
}

int main() {
	int n;
  int a;
  int b;
  scanf("%d", &n);
  scanf("%d", &a);
  scanf("%d", &b);
  int min_diff = (a<b)? b+1 : a+1;
  scanf("%d", &c[0]);
  for(int i=1; i<n; i++){
    scanf("%d", &c[i]);
    if (c[i]-c[i-1]<min_diff) min_diff=c[i]-c[i-1];
    //min_diff = (c[i] - c[i-1] > min_diff) ? min_diff :c[i]-c[i-1];
  }
  //printf("%d\n", min_diff);

  dp[1][0]=1;
  dp[1][1]=1;

  for(int i=1; i<=n-2; i++){
    for(int j=0; j<=i-1; j++){
    int ij = dp[i%2][fg(i,j)]%1000000007;
    int ji = dp[i%2][fg(j,i)]%1000000007;
    if(c[i+1] - c[i] >= a) dp[(i+1)%2][fg(i+1,j)] += ij;// if c[i+1] - c[i] >= a
    if(c[i+1] - c[j] >= b) dp[(i+1)%2][fg(i,i+1)] += ij; 
    if(c[i+1] - c[i] >= b) dp[(i+1)%2][fg(j,i+1)] += ji; 
    if(c[i+1] - c[j] >= a) dp[(i+1)%2][fg(i+1,i)] += ji;
    dp[i%2][fg(i,j)]=0;
    dp[i%2][fg(j,i)]=0;
    }
  }

int sum = 0;
  if(min_diff>=a)sum += 1;
  if(min_diff>=b)sum += 1;
//sum += 1 if c.each_cons(2).map{|k| (k[1]-k[0]) >= a[1]}.all?#(TRUE)
//sum += 1 if c.each_cons(2).map{|k| (k[1]-k[0]) >= a[2]}.all?#(TRUE)
  for(int j=0;j<2*n-1;j++){
    sum += dp[(n-1)%2][j] % 1000000007;
  //printf("%d %d\n",j, dp[(n-1)%2][j]);
  //sum += dp[(n-1)%2][j];
  }
  printf("%d\n", sum);
  
}
